use std::collections::VecDeque;

use crate::{
	api::context::Context,
	lexer::{Token, TokenType},
	mapped_err,
	parser::{
		expressions::{
			block::Block,
			either::Either,
			foreach::ForEachLoop,
			function_call::{FunctionCall, PostfixOperators},
			function_declaration::FunctionDeclaration,
			group::GroupDeclaration,
			if_expression::IfExpression,
			name::Name,
			object::ObjectConstructor,
			oneof::OneOf,
			run::RunExpression,
			sugar::list::List,
			Expression,
		},
		Parse, TokenQueueFunctionality,
	},
};

use super::literal::LiteralObject;

/// A binary operation. More specifically, this represents not one operation, but a group of operations that share the same precedence.
/// For example, the `+` and `-` operators share the same precedence, so they are grouped together in the `ADDITIVE` constant.
///
/// # Parameters
/// `<'this>` - The lifetime of this operation, to ensure that the contained reference to the precedent operation lives at least that long.
pub struct BinaryOperation {
	/// The operation that has the next highest precedence, or `None` if this operation has the highest precedence.
	precedent: Option<&'static BinaryOperation>,
	/// The token types that represent this operation, used to parse a binary expression.
	token_types: &'static [TokenType],
}

impl BinaryOperation {
	/// Parses the precedent operation of this one if it exists, otherwise, parses a function call (which has higher precedence than any binary operation)
	///
	/// # Parameters
	/// - `tokens` - The token stream to parse
	/// - `current_scope` - The current scope
	/// - `debug_info` - The debug information
	fn parse_precedent(&self, tokens: &mut VecDeque<Token>, context: &mut Context) -> anyhow::Result<Expression> {
		if let Some(precedent) = self.precedent {
			parse_binary_expression(precedent, tokens, context)
		} else {
			PostfixOperators::parse(tokens, context)
		}
	}
}

/// A binary expression node in the abstract syntax tree. This represents an operation that takes two operands in infix notation.
#[derive(Clone, Debug)]
pub struct BinaryExpression;

fn parse_binary_expression(operation: &BinaryOperation, tokens: &mut VecDeque<Token>, context: &mut Context) -> anyhow::Result<Expression> {
	let mut expression = operation.parse_precedent(tokens, context)?;

	while tokens.next_is_one_of(operation.token_types) {
		let operator_token = tokens.pop(tokens.peek_type()?.clone())?;
		let right = operation.parse_precedent(tokens, context)?;
		expression = Expression::FunctionCall(FunctionCall::from_binary_operation(expression, right, operator_token, context).map_err(mapped_err! {
			while = "converting a binary operation into a function call expression",
			context = context,
		})?);
	}

	Ok(expression)
}

impl Parse for BinaryExpression {
	type Output = Expression;

	fn parse(tokens: &mut VecDeque<Token>, context: &mut Context) -> anyhow::Result<Self::Output> {
		parse_binary_expression(&ASSIGNMENT, tokens, context)
	}
}

pub struct PrimaryExpression;

impl Parse for PrimaryExpression {
	type Output = Expression;

	fn parse(tokens: &mut VecDeque<Token>, context: &mut Context) -> anyhow::Result<Self::Output> {
		Ok(match tokens.peek_type()? {
			TokenType::LeftParenthesis => {
				tokens.pop(TokenType::LeftParenthesis).unwrap_or_else(|_| unreachable!());
				let expression = Expression::parse(tokens, context)?;
				tokens.pop(TokenType::RightParenthesis)?;
				expression
				// TODO: this needs to be its own expression type
			},

			// Parse function declaration expression
			TokenType::KeywordAction => Expression::Pointer(FunctionDeclaration::parse(tokens, context).map_err(mapped_err! {
				while = "attempting to parse a function declaration expression",
				context = context,
			})?),

			// Parse block expression
			TokenType::LeftBrace => Expression::Block(Block::parse(tokens, context)?),

			// Parse variable name expression
			TokenType::Identifier => Expression::Name(Name::parse(tokens, context).map_err(mapped_err! {
				while = "attempting to parse a variable name expression",
				context = context,
			})?),

			// Parse object constructor
			TokenType::KeywordNew => Expression::ObjectConstructor(ObjectConstructor::parse(tokens, context).map_err(mapped_err! {
				while = "attempting to parse an object constructor expression",
				context = context,
			})?),

			// Parse group declaration expression
			TokenType::KeywordGroup => Expression::Pointer(GroupDeclaration::parse(tokens, context).map_err(mapped_err! {
				while = "attempting to parse a group declaration expression",
				context = context,
			})?),

			// Parse one-of declaration expression
			TokenType::KeywordOneOf => Expression::Pointer(OneOf::parse(tokens, context).map_err(mapped_err! {
				while = "attempting to parse a one-of declaration expression",
				context = context,
			})?),

			TokenType::KeywordEither => Expression::Pointer(Either::parse(tokens, context)?),
			TokenType::KeywordIf => Expression::If(IfExpression::parse(tokens, context)?),
			TokenType::KeywordForEach => Expression::ForEachLoop(ForEachLoop::parse(tokens, context)?),

			// Parse run expression
			TokenType::KeywordRuntime => Expression::Run(RunExpression::parse(tokens, context).map_err(mapped_err! {
				while = "attempting to parse a run-expression",
				context = context,
			})?),

			// Syntactic sugar: These below handle cases where syntactic sugar exists for initializing objects of certain types, such as
			// strings, numbers, lists, etc.:

			// Parse list literal into a list object
			TokenType::LeftBracket => List::parse(tokens, context).map_err(mapped_err! {
				while = "attempting to parse a list literal",
				context = context,
			})?,

			// Parse string literal into a string object
			TokenType::String => {
				let token = tokens.pop(TokenType::String)?;
				let with_quotes = token.value;
				let without_quotes = with_quotes.get(1..with_quotes.len() - 1).unwrap().to_owned();
				Expression::Pointer(
					LiteralObject::try_from_object_constructor(ObjectConstructor::from_string(&without_quotes, token.span, context), context)
						.unwrap()
						.store_in_memory(context),
				)
			},

			// Parse number literal into a number object
			TokenType::Number => {
				let number_token = tokens.pop(TokenType::Number).unwrap();
				Expression::Pointer(
					LiteralObject::try_from_object_constructor(ObjectConstructor::from_number(number_token.value.parse().unwrap(), number_token.span, context), context)
						.unwrap()
						.store_in_memory(context),
				)
			},

			// bad :<
			_ => anyhow::bail!("Expected primary expression but found {}", tokens.peek_type()?),
		})
	}
}

// TODO: make this right-associative
/// The exponentiation operation, which has the highest precedence. This covers the `^` operator.
static EXPONENTIATION: BinaryOperation = BinaryOperation {
	precedent: None,
	token_types: &[TokenType::Caret],
};

// TODO: Add modulo
/// The multiplicative operations, which have the second highest precedence. This covers the `*` and `/` operators.
static MULTIPLICATIVE: BinaryOperation = BinaryOperation {
	precedent: Some(&EXPONENTIATION),
	token_types: &[TokenType::Asterisk, TokenType::ForwardSlash],
};

/// The additive operations, which have the third precedence. This covers the `+` and `-` operators.
static ADDITIVE: BinaryOperation = BinaryOperation {
	precedent: Some(&MULTIPLICATIVE),
	token_types: &[TokenType::Plus, TokenType::Minus],
};

/// The comparison operations, such as "==", "<=", etc.
static COMPARISON: BinaryOperation = BinaryOperation {
	precedent: Some(&ADDITIVE),
	token_types: &[TokenType::DoubleEquals, TokenType::LessThan, TokenType::GreaterThan],
};

/// The assignment operations, which have the lowest precedence. This covers the `=` operator.
static ASSIGNMENT: BinaryOperation = BinaryOperation {
	precedent: Some(&COMPARISON),
	token_types: &[TokenType::Equal],
};
