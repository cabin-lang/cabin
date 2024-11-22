use std::collections::VecDeque;

use colored::Colorize as _;

use crate::{
	api::{context::Context, traits::TryAs as _},
	comptime::{memory::VirtualPointer, CompileTime},
	lexer::{Span, Token, TokenType},
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
			literal::LiteralConvertible,
			name::Name,
			object::{ObjectConstructor, ObjectType},
			oneof::OneOf,
			run::RunExpression,
			sugar::list::List,
			Expression,
		},
		Parse, TokenQueueFunctionality,
	},
	transpiler::TranspileToC,
};

use super::Spanned;

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
	let start = expression.span(context);
	while tokens.next_is_one_of(operation.token_types) {
		let operator_token = tokens.pop(tokens.peek_type()?.clone())?;
		let middle = operator_token.span;
		let operator = operator_token.token_type;
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
		let end = right.span(context);
		expression = Expression::FunctionCall(FunctionCall {
			function: Box::new(Expression::FieldAccess(FieldAccess {
				left: Box::new(expression),
				right: Name::from(function_name),
				scope_id: context.scope_data.unique_id(),
				span: start.to(&middle),
			})),
			arguments: Some(vec![right]),
			compile_time_arguments: Some(Vec::new()),
			scope_id: context.scope_data.unique_id(),
			span: start.to(&end),
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
	span: Span,
}

impl Parse for FieldAccess {
	type Output = Expression;

	fn parse(tokens: &mut VecDeque<Token>, context: &mut Context) -> anyhow::Result<Self::Output> {
		let mut expression = PrimaryExpression::parse(tokens, context)?; // There should be no map_err here
		let start = expression.span(context);
		while tokens.next_is(TokenType::Dot) {
			tokens.pop(TokenType::Dot)?;
			let right = Name::parse(tokens, context)?;
			let end = right.span(context);
			expression = Expression::FieldAccess(Self {
				left: Box::new(expression),
				right,
				scope_id: context.scope_data.unique_id(),
				span: start.to(&end),
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
		if let Ok(pointer) = left_evaluated.try_as::<VirtualPointer>() {
			let literal = pointer.virtual_deref(context);
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
					let elements = field.expect_literal(context)?.expect_as::<Vec<Expression>>()?;
					elements
						.iter()
						.map(|element| {
							let variant_object = element.expect_literal(context)?;
							let name = variant_object.get_field_literal("name", context).unwrap().expect_as::<String>()?;
							Ok(if name == &self.right.unmangled_name() {
								Some(variant_object.get_field("value").unwrap())
							} else {
								None
							})
						})
						.collect::<anyhow::Result<Vec<_>>>()?
						.into_iter()
						.find_map(|element| element)
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
				span: self.span,
			}))
		}
	}
}

impl TranspileToC for FieldAccess {
	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		let left = if let Ok(name) = self.left.as_ref().try_as::<Name>() {
			format!(
				"{}_{}",
				self.left.to_c(context)?,
				name.clone().evaluate_at_compile_time(context)?.try_as_literal(context)?.address.unwrap()
			)
		} else {
			self.left.to_c(context)?
		};
		Ok(format!("{}->{}", left, self.right.mangled_name()))
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
			TokenType::KeywordNew => Expression::ObjectConstructor(ObjectConstructor::parse(tokens, context).map_err(mapped_err! {
				while = "attempting to parse an object constructor",
				context = context,
			})?),
			TokenType::KeywordGroup => GroupDeclaration::parse(tokens, context)?,
			TokenType::KeywordOneOf => Expression::OneOf(OneOf::parse(tokens, context)?),
			TokenType::KeywordEither => Expression::Either(Either::parse(tokens, context)?),
			TokenType::KeywordIf => Expression::If(IfExpression::parse(tokens, context)?),
			TokenType::KeywordForEach => Expression::ForEachLoop(ForEachLoop::parse(tokens, context)?),
			TokenType::LeftBracket => List::parse(tokens, context)?,
			TokenType::KeywordRuntime => Expression::Run(RunExpression::parse(tokens, context)?),
			TokenType::String => {
				let with_quotes = tokens.pop(TokenType::String)?.value;
				let without_quotes = with_quotes.get(1..with_quotes.len() - 1).unwrap().to_owned();
				Expression::Pointer(ObjectConstructor::from_string(&without_quotes, context))
			},
			_ => anyhow::bail!("Expected primary expression but found {}", tokens.peek_type()?),
		})
	}
}

impl Spanned for FieldAccess {
	fn span(&self, _context: &Context) -> Span {
		self.span.clone()
	}
}
