use std::collections::VecDeque;

use colored::Colorize;

use super::{
	super::Parse,
	block::Block,
	either::Either,
	foreach::ForEachLoop,
	function::FunctionDeclaration,
	function_call::{FunctionCall, PostfixOperators},
	group::GroupDeclaration,
	if_expression::IfExpression,
	list::List,
	name::Name,
	object::{LiteralConvertible, ObjectConstructor, ObjectType},
	oneof::OneOf,
	Expression,
};
use crate::{
	comptime::CompileTime,
	context::Context,
	lexer::{Token, TokenType},
	parser::{util::macros::TryAs as _, TokenQueueFunctionality},
};

/// A binary operation. More specifically, this represents not one operation, but a group of operations that share the same precedence.
/// For example, the `+` and `-` operators share the same precedence, so they are grouped together in the `ADDITIVE` constant.
///
/// # Parameters
/// `<'this>` - The lifetime of this operation, to ensure that the contained reference to the precedent operation lives at least that long.
struct BinaryOperation<'this> {
	/// The operation that has the next highest precedence, or `None` if this operation has the highest precedence.
	precedent: Option<&'this BinaryOperation<'this>>,
	/// The token types that represent this operation, used to parse a binary expression.
	token_types: &'this [TokenType],
}

// TODO: make this right-associative
/// The exponentiation operation, which has the highest precedence. This covers the `^` operator.
static EXPONENTIATION: BinaryOperation<'static> = BinaryOperation {
	precedent: None,
	token_types: &[TokenType::Caret],
};

// TODO: Add modulo
/// The multiplicative operations, which have the second highest precedence. This covers the `*` and `/` operators.
static MULTIPLICATIVE: BinaryOperation<'static> = BinaryOperation {
	precedent: Some(&EXPONENTIATION),
	token_types: &[TokenType::Asterisk, TokenType::ForwardSlash],
};

/// The additive operations, which have the third precedence. This covers the `+` and `-` operators.
static ADDITIVE: BinaryOperation<'static> = BinaryOperation {
	precedent: Some(&MULTIPLICATIVE),
	token_types: &[TokenType::Plus, TokenType::Minus],
};

/// The comparison operations, such as "==", "<=", etc.
static COMPARISON: BinaryOperation<'static> = BinaryOperation {
	precedent: Some(&ADDITIVE),
	token_types: &[TokenType::DoubleEquals, TokenType::LessThan, TokenType::GreaterThan],
};

/// The assignment operations, which have the lowest precedence. This covers the `=` operator.
static ASSIGNMENT: BinaryOperation<'static> = BinaryOperation {
	precedent: Some(&COMPARISON),
	token_types: &[TokenType::Equal],
};

impl BinaryOperation<'_> {
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

fn parse_binary_expression(operation: &BinaryOperation<'_>, tokens: &mut VecDeque<Token>, context: &mut Context) -> anyhow::Result<Expression> {
	let mut expression = operation.parse_precedent(tokens, context)?;
	while tokens.next_is_one_of(operation.token_types) {
		let operator = tokens.pop_type(tokens.peek_type()?.clone()).unwrap_or_else(|_error| unreachable!());
		let function_name = match operator {
			TokenType::Asterisk => "times",
			TokenType::DoubleEquals => "equals",
			TokenType::ForwardSlash => "divided_by",
			TokenType::LessThan => "is_less_than",
			TokenType::GreaterThan => "is_greater_than",
			TokenType::Minus => "minus",
			TokenType::Plus => "plus",
			_ => unreachable!(),
		};
		let right = operation.parse_precedent(tokens, context)?;
		expression = Expression::FunctionCall(FunctionCall {
			function: Box::new(Expression::FieldAccess(FieldAccess {
				left: Box::new(expression),
				right: Name::from(function_name),
				scope_id: context.scope_data.unique_id(),
			})),
			arguments: Some(vec![right]),
			compile_time_arguments: Some(Vec::new()),
			scope_id: context.scope_data.unique_id(),
		});
	}

	Ok(expression)
}

impl Parse for BinaryExpression {
	type Output = Expression;

	fn parse(tokens: &mut VecDeque<Token>, context: &mut Context) -> anyhow::Result<Self::Output> {
		parse_binary_expression(&ASSIGNMENT, tokens, context)
	}
}

#[derive(Debug, Clone)]
pub struct FieldAccess {
	pub left: Box<Expression>,
	pub right: Name,
	pub scope_id: usize,
}

impl Parse for FieldAccess {
	type Output = Expression;

	fn parse(tokens: &mut VecDeque<Token>, context: &mut Context) -> anyhow::Result<Self::Output> {
		let mut expression = PrimaryExpression::parse(tokens, context)?; // There should be no map_err here
		while tokens.next_is(TokenType::Dot) {
			tokens.pop(TokenType::Dot)?;
			let right = Name::parse(tokens, context)?;
			expression = Expression::FieldAccess(Self {
				left: Box::new(expression),
				right,
				scope_id: context.scope_data.unique_id(),
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
		if let Ok(literal) = left_evaluated.try_as_literal(context) {
			Ok(match literal.object_type() {
				// Object fields
				ObjectType::Normal => {
					let field = literal.get_field(self.right.clone()).ok_or_else(|| {
						anyhow::anyhow!(
							"Attempted to access a the field \"{}\" on an object, but no field with that name exists on that object.",
							self.right.unmangled_name().bold().cyan()
						)
					})?;

					if let Ok(Ok(mut function_declaration)) = field.try_as_literal(context).map(|field| FunctionDeclaration::from_literal(field, context)) {
						function_declaration.this_object = Some(Box::new(left_evaluated));
						Expression::Pointer(function_declaration.to_literal(context).unwrap().store_in_memory(context))
					} else {
						field
					}
				},

				// Either fields
				ObjectType::Either => {
					let field = literal.get_field("variants").unwrap();
					let elements = field.try_as_literal(context).unwrap().try_as::<Vec<Expression>>().unwrap();
					elements
						.iter()
						.find_map(|element| {
							let variant_object = element.try_as_literal(context).unwrap();
							let name = variant_object.get_field_literal("name", context).unwrap().try_as::<String>().unwrap();
							if name == &self.right.unmangled_name() {
								Some(variant_object.get_field("value").unwrap())
							} else {
								None
							}
						})
						.ok_or_else(|| {
							anyhow::anyhow!(
								"Attempted to access a variant called \"{}\" on an either, but the either has no variant with that name.",
								self.right.unmangled_name().cyan().bold()
							)
						})?
						.clone()
				},
				value => todo!("{value:?}"),
			})
		}
		// Not resolvable at compile-time - return the original expression
		else {
			Ok(Expression::FieldAccess(FieldAccess {
				left: Box::new(left_evaluated),
				right: self.right,
				scope_id: self.scope_id,
			}))
		}
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
			},
			TokenType::Number => Expression::ObjectConstructor(ObjectConstructor::from_number(tokens.pop(TokenType::Number).unwrap().value.parse().unwrap())),
			TokenType::KeywordAction => Expression::FunctionDeclaration(FunctionDeclaration::parse(tokens, context)?),
			TokenType::LeftBrace => Expression::Block(Block::parse(tokens, context)?),
			TokenType::Identifier => Expression::Name(Name::parse(tokens, context)?),
			TokenType::KeywordNew => Expression::ObjectConstructor(
				ObjectConstructor::parse(tokens, context).map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while attempting to parse an expression".dimmed()))?,
			),
			TokenType::KeywordGroup => GroupDeclaration::parse(tokens, context)?,
			TokenType::KeywordOneOf => Expression::OneOf(OneOf::parse(tokens, context)?),
			TokenType::KeywordEither => Expression::Either(Either::parse(tokens, context)?),
			TokenType::KeywordIf => Expression::If(IfExpression::parse(tokens, context)?),
			TokenType::KeywordForEach => Expression::ForEachLoop(ForEachLoop::parse(tokens, context)?),
			TokenType::LeftBracket => List::parse(tokens, context)?,
			TokenType::String => {
				let with_quotes = tokens.pop(TokenType::String)?.value;
				let without_quotes = with_quotes.get(1..with_quotes.len() - 1).unwrap().to_owned();
				Expression::Pointer(ObjectConstructor::from_string(&without_quotes, context))
			},
			_ => anyhow::bail!("Expected primary expression but found {}", tokens.peek_type()?),
		})
	}
}