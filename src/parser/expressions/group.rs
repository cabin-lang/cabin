use std::collections::{HashMap, VecDeque};

use colored::Colorize;

use crate::{
	comptime::CompileTime,
	context::Context,
	lexer::{Token, TokenType},
	literal,
	literal_list,
	parse_list,
	parser::{
		expressions::{
			name::Name,
			object::{Field, LiteralConvertible, LiteralObject, ObjectConstructor, ObjectType},
			Expression,
			Parse,
		},
		statements::tag::TagList,
		ListType,
		TokenQueueFunctionality,
	},
	string_literal,
};

#[derive(Debug, Clone)]
pub struct GroupDeclaration {
	pub fields: Vec<Field>,
	pub scope_id: usize,
}

impl Parse for GroupDeclaration {
	type Output = Expression;

	fn parse(tokens: &mut VecDeque<Token>, context: &mut Context) -> anyhow::Result<Self::Output> {
		tokens.pop(TokenType::KeywordGroup)?;

		// Fields
		let mut fields = Vec::new();
		parse_list!(tokens, ListType::Braced, {
			// Parse tags
			let tags = if tokens.next_is(TokenType::TagOpening) {
				Some(TagList::parse(tokens, context)?)
			} else {
				None
			};

			// Name
			let name =
				Name::parse(tokens, context).map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while attempting to parse an the type name of object constructor".dimmed()))?;

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
				if let Some(expression_tags) = value.tags() {
					if let Some(declaration_tags) = &tags {
						*expression_tags = declaration_tags.clone();
					}
				}

				Some(value)
			} else {
				None
			};

			// Add field
			fields.push(Field { name, value, field_type });
		});

		Ok(Expression::Group(GroupDeclaration {
			fields,
			scope_id: context.scope_data.unique_id(),
		}))
	}
}

impl CompileTime for GroupDeclaration {
	type Output = Expression;

	fn evaluate_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self::Output> {
		let mut fields = Vec::new();

		for field in self.fields {
			// Field value
			let value = if let Some(value) = field.value {
				let evaluated = value.evaluate_at_compile_time(context).map_err(|error| {
					anyhow::anyhow!(
						"{error}\n\t{}",
						format!(
							"while evaluating the default value of the field \"{}\" of a group declaration at compile-time",
							field.name.unmangled_name().bold().cyan()
						)
						.dimmed()
					)
				})?;

				if !evaluated.is_literal() {
					anyhow::bail!(
						"Attempted to assign a default value to a group field that's not known at compile-time\n\t{}",
						format!("while checking the default value of the field \"{}\"", field.name.unmangled_name().bold().cyan()).dimmed()
					);
				}

				Some(evaluated)
			} else {
				None
			};

			// Field type
			let field_type = if let Some(field_type) = field.field_type {
				Some(field_type.evaluate_at_compile_time(context).map_err(|error| {
					anyhow::anyhow!(
						"{error}\n\t{}",
						format!(
							"while evaluating the value of the field \"{}\" of a group declaration at compile-time",
							field.name.unmangled_name().bold().cyan()
						)
						.dimmed()
					)
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
		Ok(Expression::Pointer(
			GroupDeclaration { fields, scope_id: self.scope_id }.to_literal(context)?.store_in_memory(context),
		))
	}
}

impl LiteralConvertible for GroupDeclaration {
	fn to_literal(self, context: &mut Context) -> anyhow::Result<LiteralObject> {
		let fields = self
			.fields
			.into_iter()
			.filter_map(|field| {
				field.value.and_then(|value| {
					Some(literal! {
						context,
						Field {
							name = string_literal!(&field.name.unmangled_name(), context),
							value = value
						},
						self.scope_id
					})
				})
			})
			.collect();

		let constructor = ObjectConstructor {
			fields: vec![Field {
				name: "fields".into(),
				value: Some(literal_list!(context, self.scope_id, fields)),
				field_type: None,
			}],
			scope_id: self.scope_id,
			internal_fields: HashMap::new(),
			type_name: "Group".into(),
			object_type: ObjectType::Group,
		};

		LiteralObject::try_from_object_constructor(constructor, context)
	}

	fn from_literal(literal: &LiteralObject, context: &Context) -> anyhow::Result<Self> {
		let fields = literal
			.get_field_literal(&"fields".into(), context)
			.unwrap()
			.list_elements()
			.unwrap()
			.iter()
			.map(|field_object| {
				let name = Name::from(
					field_object
						.as_literal(context)
						.unwrap()
						.get_field_literal(&"name".into(), context)
						.unwrap()
						.as_string()
						.unwrap(),
				);
				let value = field_object.as_literal(context).unwrap().get_field(&"value".into());
				Field { name, value, field_type: None }
			})
			.collect();

		Ok(GroupDeclaration {
			fields,
			scope_id: literal.scope_id,
		})
	}
}
