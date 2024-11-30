use colored::Colorize as _;

use crate::{
	api::{
		context::context,
		scope::ScopeId,
		traits::{TerminalOutput, TryAs as _},
	},
	bail_err,
	comptime::{memory::VirtualPointer, CompileTime},
	err,
	lexer::{Span, TokenType},
	parser::{
		expressions::{
			function_declaration::FunctionDeclaration,
			literal::LiteralConvertible as _,
			literal::LiteralObject,
			name::Name,
			object::{Field, InternalFieldValue, ObjectConstructor},
			operators::PrimaryExpression,
			represent_as::RepresentAs,
			Expression, Spanned, Typed,
		},
		statements::tag::TagList,
		Parse, TokenQueue, TokenQueueFunctionality as _,
	},
	transpiler::TranspileToC,
};

/// A type describing how fields are accessed on this type of objects via the dot operator.
/// For example, on a normal object, the dot operator just gets a field with the given name,
/// but for `eithers`, it indexes into the either's variants and finds the one with the given
/// name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldAccessType {
	Normal,
	Either,
}

#[derive(Debug, Clone)]
pub enum FieldAccessOperator {
	Dot,
	Colon,
}

impl TryFrom<TokenType> for FieldAccessOperator {
	type Error = anyhow::Error;

	fn try_from(value: TokenType) -> Result<Self, Self::Error> {
		Ok(match value {
			TokenType::Dot => FieldAccessOperator::Dot,
			TokenType::Colon => FieldAccessOperator::Colon,
			_ => anyhow::bail!("literally how did you even get ths error to happen"),
		})
	}
}

#[derive(Debug, Clone)]
pub struct FieldAccess {
	left: Box<Expression>,
	right: Name,
	scope_id: ScopeId,
	span: Span,
	access_type: FieldAccessOperator,
}

impl Parse for FieldAccess {
	type Output = Expression;

	fn parse(tokens: &mut TokenQueue) -> anyhow::Result<Self::Output> {
		let mut expression = PrimaryExpression::parse(tokens)?; // There should be no map_err here
		let start = expression.span();
		while tokens.next_is_one_of(&[TokenType::Dot, TokenType::Colon]) {
			let access_type = FieldAccessOperator::try_from(tokens.pop_front().unwrap().token_type)?;
			let right = Name::parse(tokens)?;
			let end = right.span();
			expression = Expression::FieldAccess(Self {
				left: Box::new(expression),
				right,
				scope_id: context().scope_data.unique_id(),
				span: start.to(&end),
				access_type,
			});
		}

		Ok(expression)
	}
}

impl CompileTime for FieldAccess {
	type Output = Expression;

	fn evaluate_at_compile_time(self) -> anyhow::Result<Self::Output> {
		let left_evaluated = self.left.evaluate_at_compile_time()?;

		// Resolvable at compile-time
		if let Ok(pointer) = left_evaluated.try_as_literal().map(|value| value.address.unwrap()) {
			let literal = pointer.virtual_deref();

			Ok(match self.access_type {
				FieldAccessOperator::Dot => match literal.field_access_type() {
					// Object fields
					FieldAccessType::Normal => {
						let field = literal.get_field(self.right.clone()).ok_or_else(|| {
							anyhow::anyhow!(
								"Attempted to access a the field \"{}\" on an object, but no field with that name exists on that object.",
								self.right.unmangled_name().bold().cyan()
							)
						})?;

						let field_value_literal = field.virtual_deref();
						if field_value_literal.type_name() == &"Function".into() {
							let mut function_declaration = FunctionDeclaration::from_literal(field_value_literal).unwrap();
							function_declaration.set_this_object(left_evaluated);
							context().virtual_memory.replace(field.to_owned(), function_declaration.to_literal());
							Expression::Pointer(field.to_owned())
						} else {
							Expression::Pointer(field)
						}
					},

					// Either fields
					FieldAccessType::Either => {
						let variants = literal.get_internal_field::<Vec<(Name, VirtualPointer)>>("variants").unwrap();
						variants
							.iter()
							.find_map(|(name, value)| (name == &self.right).then_some(Expression::Pointer(value.to_owned())))
							.ok_or_else(|| {
								anyhow::anyhow!(
									"Attempted to access a variant called \"{}\" on an either, but the either has no variant with that name.",
									self.right.unmangled_name().cyan().bold()
								)
							})?
					},
				},
				FieldAccessOperator::Colon => match literal.field_access_type() {
					FieldAccessType::Normal => {
						let declaration = RepresentAs::from_literal(
							context()
								.scope_data
								.get_variable(&self.right)
								.ok_or_else(|| {
									err! {
										base = "no represent as with this name was found",
									}
								})?
								.try_as::<VirtualPointer>()?
								.virtual_deref(),
						)?;

						if !declaration.can_represent(&left_evaluated)? {
							bail_err! {
								base = "Attempted to represent an object with a represent-as declaration, but that declaration can't apply to that object.",
								details = format!(
									"
									Here, you use the colon operator to attempt to represent an object of type \"{}\" using the represent-as declaration
									\"{}\". However, That represent-as declaration can only represent objects of type \"{}\".
									",
									left_evaluated.get_type()?.virtual_deref().name().unmangled_name().bold().yellow(),
									self.right.unmangled_name().bold().yellow(),
									declaration.representables()?.bold().yellow(),
								).as_terminal_output(),
							}
						}

						let mut fields = declaration.fields().to_vec();
						let mut original_fields = literal
							.fields
							.clone()
							.into_iter()
							.map(|(name, field_value)| Field {
								name,
								field_type: None,
								value: Some(Expression::Pointer(field_value)),
							})
							.collect::<Vec<_>>();
						fields.append(&mut original_fields);

						let mut internal_fields = literal.internal_fields.clone();
						let _ = internal_fields.insert("representing_type_name".to_owned(), InternalFieldValue::Name(literal.type_name().clone()));

						Expression::Pointer(
							LiteralObject::try_from(ObjectConstructor {
								type_name: declaration.type_to_represent_as().try_as::<VirtualPointer>().unwrap().virtual_deref().name().to_owned(),
								fields,
								internal_fields,
								inner_scope_id: context().scope_data.file_id(),
								field_access_type: FieldAccessType::Normal,
								name: "anonymous_represent_as_casted".into(),
								outer_scope_id: context().scope_data.file_id(),
								tags: TagList::default(),
								span: declaration.span(),
							})
							.unwrap()
							.store_in_memory(),
						)
					},
					_ => todo!(),
				},
			})
		}
		// Not resolvable at compile-time - return the original expression
		else {
			Ok(Expression::FieldAccess(FieldAccess {
				left: Box::new(left_evaluated),
				right: self.right,
				scope_id: self.scope_id,
				span: self.span,
				access_type: self.access_type,
			}))
		}
	}
}

impl TranspileToC for FieldAccess {
	fn to_c(&self) -> anyhow::Result<String> {
		let left = if let Ok(name) = self.left.as_ref().try_as::<Name>() {
			format!("{}_{}", self.left.to_c()?, name.clone().evaluate_at_compile_time()?.try_as_literal()?.address.unwrap())
		} else {
			self.left.to_c()?
		};
		Ok(format!("{}->{}", left, self.right.mangled_name()))
	}
}

impl Spanned for FieldAccess {
	fn span(&self) -> Span {
		self.span
	}
}

impl FieldAccess {
	pub fn new(left: Expression, right: Name, scope_id: ScopeId, span: Span) -> FieldAccess {
		FieldAccess {
			left: Box::new(left),
			right,
			scope_id,
			span,
			access_type: FieldAccessOperator::Dot,
		}
	}
}
