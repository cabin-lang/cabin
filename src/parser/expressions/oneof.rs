use std::collections::HashMap;

use crate::{
	api::{context::Context, scope::ScopeType},
	comptime::{memory::VirtualPointer, CompileTime},
	lexer::{Span, TokenType},
	parse_list,
	parser::{
		expressions::{
			literal::{LiteralConvertible, LiteralObject},
			name::Name,
			object::ObjectType,
			Expression,
		},
		statements::tag::TagList,
		ListType, Parse, TokenQueue, TokenQueueFunctionality,
	},
};

use super::{object::InternalFieldValue, Spanned};

#[derive(Debug, Clone)]
pub struct OneOf {
	compile_time_parameters: Vec<Name>,
	choices: Vec<Expression>,
	scope_id: usize,
	span: Span,
	name: Name,
}

impl Parse for OneOf {
	type Output = VirtualPointer;

	fn parse(tokens: &mut TokenQueue, context: &mut Context) -> anyhow::Result<Self::Output> {
		let start = tokens.pop(TokenType::KeywordOneOf)?.span;
		context.scope_data.enter_new_unlabeled_scope(ScopeType::OneOf);

		// Compile-time parameters
		let compile_time_parameters = if tokens.next_is(TokenType::LeftAngleBracket) {
			let mut compile_time_parameters = Vec::new();
			parse_list!(tokens, ListType::AngleBracketed, {
				let name = Name::parse(tokens, context)?;
				context.scope_data.declare_new_variable(name.clone(), Expression::Void(()))?;
				compile_time_parameters.push(name);
			});
			compile_time_parameters
		} else {
			Vec::new()
		};

		// Choices
		let mut choices = Vec::new();
		let end = parse_list!(tokens, ListType::Braced, {
			choices.push(Expression::parse(tokens, context)?);
		})
		.span;

		context.scope_data.exit_scope()?;

		// Return
		Ok(OneOf {
			choices,
			compile_time_parameters,
			scope_id: context.scope_data.unique_id(),
			span: start.to(&end),
			name: "anonymous_one_of".into(),
		}
		.to_literal()
		.store_in_memory(context))
	}
}

impl CompileTime for OneOf {
	type Output = OneOf;

	fn evaluate_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self::Output> {
		let mut choices = Vec::new();
		for choice in self.choices {
			if let Expression::Name(choice_name) = &choice {
				if self.compile_time_parameters.contains(choice_name) {
					choices.push(choice);
					continue;
				}
			}

			let choice_value = choice.evaluate_at_compile_time(context)?;
			choices.push(choice_value);
		}

		Ok(OneOf {
			choices,
			scope_id: self.scope_id,
			compile_time_parameters: self.compile_time_parameters,
			span: self.span,
			name: self.name,
		})
	}
}

impl LiteralConvertible for OneOf {
	fn to_literal(self) -> LiteralObject {
		LiteralObject {
			address: None,
			fields: HashMap::from([]),
			internal_fields: HashMap::from([
				("choices".to_owned(), InternalFieldValue::ExpressionList(self.choices)),
				("compile_time_parameters".to_owned(), InternalFieldValue::NameList(self.compile_time_parameters)),
			]),
			name: self.name,
			object_type: ObjectType::OneOf,
			scope_id: self.scope_id,
			span: self.span,
			type_name: "OneOf".into(),
			tags: TagList::default(),
		}
	}

	fn from_literal(literal: &LiteralObject) -> anyhow::Result<Self> {
		Ok(OneOf {
			choices: literal.get_internal_field::<Vec<Expression>>("choices")?.to_owned(),
			compile_time_parameters: literal.get_internal_field::<Vec<Name>>("compile_time_parameters")?.to_owned(),
			scope_id: literal.declared_scope_id(),
			span: literal.span.clone(),
			name: literal.name().to_owned(),
		})
	}
}

impl Spanned for OneOf {
	fn span(&self, _context: &Context) -> Span {
		self.span.clone()
	}
}
