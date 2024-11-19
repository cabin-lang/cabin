use std::collections::HashMap;

use crate::{
	comptime::CompileTime,
	context::Context,
	lexer::TokenType,
	literal, literal_list, parse_list,
	parser::{
		expressions::{
			name::Name,
			object::ObjectConstructor,
			object::{Field, LiteralConvertible, LiteralObject, ObjectType},
			Expression,
		},
		ListType, Parse, TokenQueue, TokenQueueFunctionality,
	},
	string_literal,
};

#[derive(Debug, Clone)]
pub struct Either {
	variants: Vec<Name>,
	scope_id: usize,
}

impl Parse for Either {
	type Output = Either;

	fn parse(tokens: &mut TokenQueue, context: &mut Context) -> anyhow::Result<Self::Output> {
		tokens.pop(TokenType::KeywordEither)?;
		let mut variants = Vec::new();
		parse_list!(tokens, ListType::Braced, {
			variants.push(Name::parse(tokens, context)?);
		});

		Ok(Either {
			variants,
			scope_id: context.scope_data.unique_id(),
		})
	}
}

impl CompileTime for Either {
	type Output = Expression;

	fn evaluate_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self::Output> {
		Ok(Expression::Pointer(self.to_literal(context)?.store_in_memory(context)))
	}
}

impl LiteralConvertible for Either {
	fn to_literal(self, context: &mut Context) -> anyhow::Result<LiteralObject> {
		let variants = self
			.variants
			.iter()
			.map(|field| {
				literal! {
					context,
					Field {
						name = string_literal!(&field.unmangled_name(), context),
						value = literal! {
							context,
							Object {},
							self.scope_id
						}
					},
					self.scope_id
				}
			})
			.collect();

		let constructor = ObjectConstructor {
			fields: vec![Field {
				name: "variants".into(),
				value: Some(literal_list!(context, self.scope_id, variants)),
				field_type: None,
			}],
			scope_id: self.scope_id,
			internal_fields: HashMap::new(),
			type_name: "Either".into(),
			object_type: ObjectType::Either,
		};

		LiteralObject::try_from_object_constructor(constructor, context)
	}

	fn from_literal(literal: &LiteralObject, context: &Context) -> anyhow::Result<Self> {
		if literal.object_type() != &ObjectType::Either {
			anyhow::bail!("Attempted to convert a non-either object into an either");
		}

		let variants = literal
			.get_field_literal(&"variants".into(), context)
			.unwrap()
			.list_elements()
			.unwrap()
			.iter()
			.map(|field_object| {
				Name::from(
					field_object
						.as_literal(context)
						.unwrap()
						.get_field_literal(&"value".into(), context)
						.unwrap()
						.as_string()
						.unwrap(),
				)
			})
			.collect();

		Ok(Either {
			variants,
			scope_id: literal.scope_id,
		})
	}
}
