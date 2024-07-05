use crate::{
	compile_time::{CompileTime, TranspileToC},
	context::Context,
	formatter::{ColoredCabin, ToCabin},
	lexer::{Token, TokenType},
	parser::{
		expressions::{
			function_call::FunctionCall,
			literals::{Literal, LiteralValue},
			util::name::Name,
			Expression,
		},
		Parse, TokenQueue,
	},
};

use std::collections::VecDeque;

use super::{run::ParentExpression, util::types::Typed};

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
			FunctionCall::parse(tokens, context)
		}
	}
}

/// A binary expression node in the abstract syntax tree. This represents an operation that takes two operands in infix notation.
#[derive(Clone, Debug)]
pub struct BinaryExpression;

/// Parses a binary expression with the given operation.
///
/// # Parameters
/// - `operation` - The operation to parse
/// - `tokens` - The token stream to parse
/// - `context` - Global data about the compiler's state
///
/// # Returns
/// A `Result` containing either the parsed expression or an `Error`.
fn parse_binary_expression(operation: &BinaryOperation<'_>, tokens: &mut VecDeque<Token>, context: &mut Context) -> anyhow::Result<Expression> {
	let mut expression = operation.parse_precedent(tokens, context)?;
	while tokens.next_is_one_of(operation.token_types) {
		let operator = tokens.pop_type(tokens.peek().unwrap().token_type.clone()).unwrap_or_else(|_error| unreachable!());
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
		expression = Expression::FunctionCall(Box::new(FunctionCall {
			function: Expression::Access(Box::new(AccessExpression {
				left: expression,
				right: Name(function_name.to_owned()),
			})),
			arguments: vec![right],
			has_been_converted_to_block: false,
		}));
	}

	Ok(expression)
}

impl Parse for BinaryExpression {
	type Output = Expression;

	fn parse(tokens: &mut VecDeque<Token>, context: &mut Context) -> anyhow::Result<Self::Output> {
		parse_binary_expression(if context.is_parsing_type { &COMPARISON } else { &ASSIGNMENT }, tokens, context)
	}
}

#[derive(Debug, Clone)]
pub struct AccessExpression {
	left: Expression,
	right: Name,
}

impl Parse for AccessExpression {
	type Output = Expression;

	fn parse(tokens: &mut VecDeque<Token>, context: &mut Context) -> anyhow::Result<Self::Output> {
		let mut expression = Expression::Literal(Literal::new(LiteralValue::parse(tokens, context)?)); // There should be no map_err here
		while tokens.next_is(TokenType::Dot) {
			tokens.pop(TokenType::Dot, context)?;
			let right = Name(tokens.pop(TokenType::Identifier, context)?);
			expression = Expression::Access(Box::new(Self { left: expression, right }));
		}

		Ok(expression)
	}
}

impl CompileTime for AccessExpression {
	fn compile_time_evaluate(&self, context: &mut Context, with_side_effects: bool) -> anyhow::Result<Expression> {
		let left = self.left.compile_time_evaluate(context, with_side_effects)?;
		Ok(Expression::Access(Box::new(Self { left, right: self.right.clone() })))
	}
}

impl ParentExpression for AccessExpression {
	fn evaluate_children_at_compile_time(&self, context: &mut Context) -> anyhow::Result<Expression> {
		let left = self.left.compile_time_evaluate(context, true)?;
		Ok(Expression::Access(Box::new(Self { left, right: self.right.clone() })))
	}
}

impl TranspileToC for AccessExpression {
	fn c_prelude(&self, context: &mut Context) -> anyhow::Result<String> {
		self.left.c_prelude(context)
	}

	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		Ok(format!("{}.{}", self.left.to_c(context)?, self.right.mangled_name()))
	}
}

impl Typed for AccessExpression {
	fn get_type(&self, context: &mut Context) -> anyhow::Result<Literal> {
		let left = self.left.compile_time_evaluate(context, true)?;
		let left_type = left.get_type(context)?;

		let mut literal_value = left_type.0.clone();
		if let LiteralValue::VariableReference(variable_reference) = literal_value {
			literal_value = variable_reference.value(context)?.as_literal(context)?.clone().0;
		}

		if let LiteralValue::Either(_) = &literal_value {
			return Ok(left_type);
		}

		match literal_value {
			LiteralValue::Object(object) => object
				.get_field(&self.right)
				.ok_or_else(|| {
					anyhow::anyhow!("Attempted to access field \"{}\" on an object, but no field with that name exists", {
						self.right.unmangled_name()
					})
				})?
				.clone(),
			value => unreachable!("left type is {:?} in expression {}", value, self.to_cabin()),
		}
		.get_type(context)
	}
}

impl ToCabin for AccessExpression {
	fn to_cabin(&self) -> String {
		format!("{}.{}", self.left.to_cabin(), self.right.unmangled_name())
	}
}

impl ColoredCabin for AccessExpression {
	fn to_colored_cabin(&self, context: &mut Context) -> String {
		format!("{}.{}", self.left.to_colored_cabin(context), self.right.to_colored_cabin(context))
	}
}
