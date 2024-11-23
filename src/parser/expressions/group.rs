use std::collections::{HashMap, VecDeque};

use crate::{
	api::{context::Context, scope::ScopeType},
	bail_err,
	comptime::{memory::VirtualPointer, CompileTime},
	lexer::{Span, Token, TokenType},
	mapped_err, parse_list,
	parser::{
		expressions::{
			literal::{LiteralConvertible, LiteralObject},
			name::Name,
			object::{Field, ObjectType},
			Expression, Parse,
		},
		statements::tag::TagList,
		ListType, TokenQueueFunctionality,
	},
	transpiler::TranspileToC,
};

use super::{object::InternalFieldValue, Spanned, Typed};

#[derive(Debug, Clone)]
pub struct GroupDeclaration {
	pub fields: Vec<Field>,
	pub scope_id: usize,
	pub name: Name,
	pub span: Span,
}

impl Parse for GroupDeclaration {
	type Output = VirtualPointer;

	fn parse(tokens: &mut VecDeque<Token>, context: &mut Context) -> anyhow::Result<Self::Output> {
		let start = tokens.pop(TokenType::KeywordGroup)?.span;
		context.scope_data.enter_new_unlabeled_scope(ScopeType::Group);
		let inner_scope_id = context.scope_data.unique_id();

		// Fields
		let mut fields = Vec::new();
		let end = parse_list!(tokens, ListType::Braced, {
			// Parse tags
			let tags = if tokens.next_is(TokenType::TagOpening) {
				Some(TagList::parse(tokens, context)?)
			} else {
				None
			};

			// Name
			let name = Name::parse(tokens, context).map_err(mapped_err! {
				while = "attempting to parse an the type name of object constructor",
				context = context,
			})?;

			// Type
			let field_type = if tokens.next_is(TokenType::Colon) {
				tokens.pop(TokenType::Colon)?;
				Some(Expression::parse(tokens, context)?)
			} else {
				None
			};

			// Value
			let value = if tokens.next_is(TokenType::Equal) {
				tokens.pop(TokenType::Equal)?;
				let mut value = Expression::parse(tokens, context)?;

				// Set tags
				if let Some(tags) = tags.clone() {
					value.set_tags(tags, context);
				}

				Some(value)
			} else {
				None
			};

			// Add field
			fields.push(Field { name, value, field_type });
		})
		.span;
		context.scope_data.exit_scope()?;

		Ok(GroupDeclaration {
			fields,
			scope_id: inner_scope_id,
			name: "anonymous_group".into(),
			span: start.to(&end),
		}
		.to_literal()
		.store_in_memory(context))
	}
}

impl CompileTime for GroupDeclaration {
	type Output = GroupDeclaration;

	fn evaluate_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self::Output> {
		let previous = context.scope_data.set_current_scope(self.scope_id);
		let mut fields = Vec::new();

		for field in self.fields {
			// Field value
			let value = if let Some(value) = field.value {
				let evaluated = value.evaluate_at_compile_time(context).map_err(mapped_err! {
					while = format!("evaluating the default value of the field \"{}\" of a group declaration at compile-time", field.name.unmangled_name().bold().cyan()),
					context = context,
				})?;

				if !evaluated.is_pointer() {
					bail_err! {
						base = "Attempted to assign a default value to a group field that's not known at compile-time",
						while = format!("while checking the default value of the field \"{}\"", field.name.unmangled_name().bold().cyan()),
						context = context,
					};
				}

				Some(evaluated)
			} else {
				None
			};

			// Field type
			let field_type = if let Some(field_type) = field.field_type {
				Some(field_type.evaluate_at_compile_time(context).map_err(mapped_err! {
					while = format!(
						"evaluating the value of the field \"{}\" of a group declaration at compile-time",
						field.name.unmangled_name().bold().cyan()
					),
					context = context,
				})?)
			} else {
				None
			};

			// Add the field
			fields.push(Field {
				name: field.name,
				value,
				field_type,
			});
		}

		// Store in memory and return a pointer
		context.scope_data.set_current_scope(previous);
		Ok(GroupDeclaration {
			fields,
			scope_id: self.scope_id,
			name: self.name,
			span: self.span,
		})
	}
}

impl TranspileToC for GroupDeclaration {
	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		let mut builder = "{".to_owned();

		// Anything fields
		if self.name != "Anything".into() {
			let anything = GroupDeclaration::from_literal(&context.scope_data.expect_global_variable("Anything").clone().expect_literal(context).cloned().map_err(
				mapped_err! {
					while = format!("interpreting the value of the global variable {} as a literal", "Anything".bold().yellow()),
					context = context,
				},
			)?)?;
			for field in &anything.fields {
				builder += &format!(
					"\n\t{}* {};",
					field
						.value
						.as_ref()
						.unwrap_or(&Expression::Void(()))
						.get_type(context)?
						.virtual_deref(context)
						.clone()
						.to_c_type(context)?,
					field.name.to_c(context)?
				);
			}
		}

		for field in &self.fields {
			builder += &format!(
				"\n\t{}* {};",
				if let Some(field_type) = &field.field_type {
					field_type.try_as_literal_or_name(context)?.clone().to_c_type(context)?
				} else {
					field
						.value
						.as_ref()
						.unwrap_or(&Expression::Void(()))
						.get_type(context)?
						.virtual_deref(context)
						.clone()
						.to_c_type(context)?
				},
				field.name.to_c(context)?
			);
		}

		match self.name.unmangled_name().as_str() {
			"Text" => builder += "\n\tchar* internal_value;",
			"Number" => builder += "\n\tfloat internal_value;",
			"Function" => builder += "\n\tvoid* call;",
			"List" => builder += "\n\tvoid* elements;\n\tint size;\n\tint capacity;",
			_ => {},
		}

		builder += "\n}";
		Ok(builder)
	}
}

impl LiteralConvertible for GroupDeclaration {
	fn to_literal(self) -> LiteralObject {
		LiteralObject {
			address: None,
			fields: HashMap::from([]),
			internal_fields: HashMap::from([("fields".to_owned(), InternalFieldValue::FieldList(self.fields))]),
			name: self.name,
			object_type: ObjectType::Group,
			scope_id: self.scope_id,
			span: self.span,
			type_name: "Group".into(),
			tags: TagList::default(),
		}
	}

	fn from_literal(literal: &LiteralObject) -> anyhow::Result<Self> {
		Ok(GroupDeclaration {
			fields: literal.get_internal_field::<Vec<Field>>("fields")?.to_owned(),
			scope_id: literal.declared_scope_id(),
			name: literal.name.clone(),
			span: literal.span.clone(),
		})
	}
}

impl Spanned for GroupDeclaration {
	fn span(&self, _context: &Context) -> crate::lexer::Span {
		self.span.clone()
	}
}
