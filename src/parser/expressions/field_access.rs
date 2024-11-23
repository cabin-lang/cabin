use std::{collections::HashMap, hash::Hash};

use colored::Colorize as _;

use crate::{
	api::{context::Context, traits::TryAs as _},
	comptime::{memory::VirtualPointer, CompileTime},
	err,
	lexer::{Span, TokenType},
	parser::{
		expressions::{
			function_declaration::FunctionDeclaration, literal::LiteralConvertible as _, name::Name, object::ObjectType, operators::PrimaryExpression, Expression, Spanned,
		},
		statements::tag::TagList,
		Parse, TokenQueue, TokenQueueFunctionality as _,
	},
	transpiler::TranspileToC,
};

use super::{literal::LiteralObject, object::ObjectConstructor};

#[derive(Debug, Clone)]
pub enum FieldAccessType {
	Normal,
	RepresentAs,
}

impl TryFrom<TokenType> for FieldAccessType {
	type Error = anyhow::Error;

	fn try_from(value: TokenType) -> Result<Self, Self::Error> {
		Ok(match value {
			TokenType::Dot => FieldAccessType::Normal,
			TokenType::Colon => FieldAccessType::RepresentAs,
			_ => anyhow::bail!("literally how did you even get ths error to happen"),
		})
	}
}

#[derive(Debug, Clone)]
pub struct FieldAccess {
	left: Box<Expression>,
	right: Name,
	scope_id: usize,
	span: Span,
	access_type: FieldAccessType,
}

impl Parse for FieldAccess {
	type Output = Expression;

	fn parse(tokens: &mut TokenQueue, context: &mut Context) -> anyhow::Result<Self::Output> {
		let mut expression = PrimaryExpression::parse(tokens, context)?; // There should be no map_err here
		let start = expression.span(context);
		while tokens.next_is_one_of(&[TokenType::Dot, TokenType::Colon]) {
			let access_type = FieldAccessType::try_from(tokens.pop_front().unwrap().token_type)?;
			let right = Name::parse(tokens, context)?;
			let end = right.span(context);
			expression = Expression::FieldAccess(Self {
				left: Box::new(expression),
				right,
				scope_id: context.scope_data.unique_id(),
				span: start.to(&end),
				access_type,
			});
		}

		Ok(expression)
	}
}

impl CompileTime for FieldAccess {
	type Output = Expression;

	fn evaluate_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self::Output> {
		let left_evaluated = self.left.evaluate_at_compile_time(context)?;

		// Resolvable at compile-time
		if let Ok(pointer) = left_evaluated.try_as_literal_or_name(context).map(|value| value.address.unwrap()) {
			let literal = pointer.virtual_deref(context);

			Ok(match self.access_type {
				FieldAccessType::Normal => match literal.object_type() {
					// Object fields
					ObjectType::Normal => {
						let field = literal.get_field(self.right.clone()).ok_or_else(|| {
							anyhow::anyhow!(
								"Attempted to access a the field \"{}\" on an object, but no field with that name exists on that object.",
								self.right.unmangled_name().bold().cyan()
							)
						})?;

						let pointer = field.try_as::<VirtualPointer>();
						if let Ok(pointer) = pointer {
							let literal = pointer.virtual_deref(context).clone();
							if literal.object_type() == &ObjectType::Function {
								let mut function_declaration = FunctionDeclaration::from_literal(&literal).unwrap();
								function_declaration.set_this_object(left_evaluated);
								context.virtual_memory.replace(pointer.to_owned(), function_declaration.to_literal());
								Expression::Pointer(pointer.to_owned())
							} else {
								field
							}
						} else {
							field
						}
					},

					// Either fields
					ObjectType::Either => {
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
							.clone()
					},
					_value => todo!("{literal:?} {}", self.right.unmangled_name()),
				},
				FieldAccessType::RepresentAs => match literal.object_type() {
					ObjectType::Normal => {
						let declaration = context.scope_data.get_represent_as(self.right).ok_or_else(|| {
							err! {
								base = "no represent as with this name was found",
								context = context,
							}
						})?;

						Expression::Pointer(
							LiteralObject::try_from_object_constructor(
								ObjectConstructor {
									type_name: declaration
										.type_to_represent_as()
										.expect_as::<VirtualPointer>()
										.unwrap()
										.virtual_deref(context)
										.name()
										.to_owned(),
									fields: declaration.fields().to_vec(),
									internal_fields: HashMap::new(),
									inner_scope_id: 0,
									object_type: ObjectType::Normal,
									name: "anonymous_represent_as_casted".into(),
									scope_id: 0,
									tags: TagList::default(),
									span: declaration.span(context),
								},
								context,
							)
							.unwrap()
							.store_in_memory(context),
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
	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		let left = if let Ok(name) = self.left.as_ref().try_as::<Name>() {
			format!(
				"{}_{}",
				self.left.to_c(context)?,
				name.clone().evaluate_at_compile_time(context)?.try_as_literal_or_name(context)?.address.unwrap()
			)
		} else {
			self.left.to_c(context)?
		};
		Ok(format!("{}->{}", left, self.right.mangled_name()))
	}
}

impl Spanned for FieldAccess {
	fn span(&self, _context: &Context) -> Span {
		self.span.clone()
	}
}

impl FieldAccess {
	pub fn new(left: Expression, right: Name, scope_id: usize, span: Span) -> FieldAccess {
		FieldAccess {
			left: Box::new(left),
			right,
			scope_id,
			span,
			access_type: FieldAccessType::Normal,
		}
	}
}
