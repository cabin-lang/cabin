use std::collections::VecDeque;

// Required because of a bug in `try_as`
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

/// A part of a formatted string literal. Each part is either just a regular string value, or an
/// expression that's inserted into the formatted string. The parts are chained together as
/// function calls at parse time, i.e.:
///
/// ```cabin
/// print("Hello {name}!");
/// ```
///
/// becomes:
///
/// ```cabin
/// print("Hello ".plus(name.to_text()).plus("!"));
/// ```
///
/// A formatted string is stored as a `Vec<StringPart>` before being converted into a function call
/// chain such as the one shown above, so the above might be something like:
///
/// ```rust
/// vec![
///     StringPart::Literal("Hello "),
///     StringPart::Expression(name.to_text()),
///     StringPart::Literal("!")
/// ]
/// ```
#[derive(Debug, try_as::macros::TryAsRef)]
pub enum StringPart {
	/// A literal string part.
	Literal(String),

	/// An interpolated expression string part.
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

/// A wrapper for implementing `Parse` for parsing string literals. In Cabin, all strings are
/// formatted strings by default, so they require special logic for parsing.
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

					// Recollect remaining tokens into string
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
