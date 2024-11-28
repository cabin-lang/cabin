use std::collections::HashMap;

use crate::{
	api::{context::context, scope::ScopeId},
	comptime::{memory::VirtualPointer, CompileTime},
	lexer::{Span, TokenType},
	parse_list,
	parser::{
		expressions::{
			literal::{LiteralConvertible, LiteralObject},
			name::Name,
			object::InternalFieldValue,
			Spanned,
		},
		statements::tag::TagList,
		ListType, Parse, TokenQueue, TokenQueueFunctionality,
	},
	transpiler::TranspileToC,
};

use super::field_access::FieldAccessType;

#[derive(Debug, Clone)]
pub struct Either {
	variants: Vec<(Name, VirtualPointer)>,
	scope_id: ScopeId,
	inner_scope_id: ScopeId,
	name: Name,
	span: Span,
}

impl Parse for Either {
	type Output = VirtualPointer;

	fn parse(tokens: &mut TokenQueue) -> anyhow::Result<Self::Output> {
		let start = tokens.pop(TokenType::KeywordEither)?.span;
		let mut variants = Vec::new();
		let end = parse_list!(tokens, ListType::Braced, {
			let name = Name::parse(tokens)?;
			let span = name.span();
			variants.push((name, LiteralObject::empty(span).store_in_memory()));
		})
		.span;

		Ok(Either {
			variants,
			scope_id: context().scope_data.unique_id(),
			inner_scope_id: context().scope_data.unique_id(),
			name: "anonymous_either".into(),
			span: start.to(&end),
		}
		.to_literal()
		.store_in_memory())
	}
}

impl CompileTime for Either {
	type Output = Either;

	fn evaluate_at_compile_time(self) -> anyhow::Result<Self::Output> {
		Ok(self)
	}
}

impl LiteralConvertible for Either {
	fn to_literal(self) -> LiteralObject {
		LiteralObject {
			address: None,
			fields: HashMap::from([]),
			internal_fields: HashMap::from([("variants".to_owned(), InternalFieldValue::LiteralMap(self.variants))]),
			name: self.name,
			field_access_type: FieldAccessType::Either,
			outer_scope_id: self.scope_id,
			inner_scope_id: Some(self.inner_scope_id),
			span: self.span,
			type_name: "Either".into(),
			tags: TagList::default(),
		}
	}

	fn from_literal(literal: &LiteralObject) -> anyhow::Result<Self> {
		Ok(Either {
			variants: literal.get_internal_field::<Vec<(Name, VirtualPointer)>>("variants")?.to_owned(),
			scope_id: literal.outer_scope_id(),
			inner_scope_id: literal.inner_scope_id.unwrap(),
			name: literal.name.clone(),
			span: literal.span.clone(),
		})
	}
}

impl TranspileToC for Either {
	fn to_c(&self) -> anyhow::Result<String> {
		let mut builder = "{\n".to_owned();
		for (variant_name, _variant_value) in &self.variants {
			builder += &format!("\n\t{},", variant_name.to_c()?);
		}

		builder += "\n}";

		Ok(builder)
	}
}

impl Spanned for Either {
	fn span(&self) -> Span {
		self.span.to_owned()
	}
}

impl Either {
	pub fn variants(&self) -> &[(Name, VirtualPointer)] {
		&self.variants
	}

	pub fn variant_names(&self) -> Vec<&Name> {
		self.variants.iter().map(|variant| &variant.0).collect()
	}
}
