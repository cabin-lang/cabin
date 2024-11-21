use std::collections::HashMap;

use crate::{
	api::{context::Context, macros::string, traits::TryAs as _},
	comptime::CompileTime,
	lexer::TokenType,
	literal, literal_list, mapped_err, parse_list,
	parser::{
		expressions::{
			literal::{LiteralConvertible, LiteralObject},
			name::{Name, NameOption as _},
			object::{Field, ObjectConstructor, ObjectType},
			Expression,
		},
		ListType, Parse, TokenQueue, TokenQueueFunctionality,
	},
	transpiler::TranspileToC,
};

#[derive(Debug, Clone)]
pub struct Either {
	variants: Vec<Name>,
	scope_id: usize,
	pub name: Option<Name>,
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
			name: None,
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
					name = self.name.with_field(field),
					context = context,
					Field {
						name = string(&field.unmangled_name(), context),
						value = literal! {
							context = context,
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
			name: None,
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
			.expect_field_literal("variants", context)
			.try_as::<Vec<Expression>>()
			.map_err(mapped_err! {
				while = "interpreting the variants field of an either as a list",
				context = context,
			})?
			.iter()
			.map(|field_object| {
				anyhow::Ok(Name::from(
					field_object
						.expect_literal(context)?
						.expect_field_literal("name", context)
						.try_as::<String>()
						.map_err(mapped_err! {
							while = "interpreting the field \"name\" of an either variant object as a string",
							context = context,
						})?,
				))
			})
			.collect::<anyhow::Result<Vec<_>>>()?;

		Ok(Either {
			variants,
			scope_id: literal.declared_scope_id(),
			name: literal.name.clone(),
		})
	}
}

impl TranspileToC for Either {
	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		let mut builder = "{\n".to_owned();
		for variant in &self.variants {
			builder += &format!("\n\t{},", variant.to_c(context)?);
		}

		builder += "\n}";

		Ok(builder)
	}
}

impl Either {
	pub fn to_c_metadata(&self, context: &Context, address: usize) -> anyhow::Result<String> {
		Ok(String::new())
	}
}
