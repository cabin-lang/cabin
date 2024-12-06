use std::collections::VecDeque;

use try_as::traits as try_as_traits;

use crate::{
	api::{context::context, traits::TryAs as _},
	bail_err,
	lexer::{tokenize_string, Span, Token, TokenType},
	parser::{
		expressions::{field_access::FieldAccess, function_call::FunctionCall, object::ObjectConstructor, Expression},
		Parse,
		TokenQueue,
		TokenQueueFunctionality as _,
	},
};

#[derive(Debug, try_as::macros::TryAsRef)]
pub enum StringPart {
	Literal(String),
	Expression(Expression),
}

impl Into<Expression> for StringPart {
	fn into(self) -> Expression {
		match self {
			StringPart::Expression(expression) => expression,
			StringPart::Literal(literal) => Expression::ObjectConstructor(ObjectConstructor::string(&literal, Span::unknown())),
		}
	}
}

pub struct CabinString;

impl Parse for CabinString {
	type Output = Expression;

	fn parse(tokens: &mut TokenQueue) -> anyhow::Result<Self::Output> {
		let token = tokens.pop(TokenType::String)?;
		let with_quotes = token.value;
		let mut without_quotes = with_quotes.get(1..with_quotes.len() - 1).unwrap().to_owned();

		let mut parts = Vec::new();
		let mut builder = String::new();
		while !without_quotes.is_empty() {
			match without_quotes.chars().next().unwrap() {
				'{' => {
					if !builder.is_empty() {
						parts.push(StringPart::Literal(builder));
						builder = String::new();
					}
					// Pop the opening brace
					without_quotes = without_quotes.get(1..without_quotes.len()).unwrap().to_owned();

					// Parse an expression
					let mut tokens = tokenize_string(&without_quotes);
					let expression = Expression::parse(&mut tokens)?;
					parts.push(StringPart::Expression(expression));

					// Recollect tokens into string
					without_quotes = tokens.into_iter().map(|token| token.value).collect();

					// Pop closing brace
					if without_quotes.chars().next().unwrap() != '}' {
						bail_err! {
							base = "Expected closing brace after format expression in string",
						};
					}
					without_quotes = without_quotes.get(1..without_quotes.len()).unwrap().to_owned();
				},
				normal_character => {
					without_quotes = without_quotes.get(1..without_quotes.len()).unwrap().to_owned();
					builder.push(normal_character);
				},
			}
		}
		if !builder.is_empty() {
			parts.push(StringPart::Literal(builder));
		}

		if parts.iter().all(|part| matches!(part, StringPart::Literal(_))) {
			return Ok(Expression::ObjectConstructor(ObjectConstructor::string(
				&parts.into_iter().map(|part| part.try_as::<String>().unwrap().to_owned()).collect::<String>(),
				token.span,
			)));
		}

		// Composite into function call, i.e., "hello {name}!" becomes
		// "hello ".plus(name.to_text()).plus("!")
		let mut parts = VecDeque::from(parts);
		let mut left = parts.pop_front().unwrap().into();
		for part in parts {
			let mut right: Expression = part.into();
			right = Expression::FunctionCall(FunctionCall::basic(Expression::FieldAccess(FieldAccess::new(
				right,
				"to_text".into(),
				context().scope_data.unique_id(),
				Span::unknown(),
			))));
			left = Expression::FunctionCall(FunctionCall::from_binary_operation(left, right, Token {
				token_type: TokenType::Plus,
				value: "+".to_owned(),
				span: Span::unknown(),
			})?);
		}

		Ok(left)
	}
}
