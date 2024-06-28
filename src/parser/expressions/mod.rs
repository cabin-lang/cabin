/// The `binary` module, which handles binary expressions, such as arithmetic operations.
pub mod binary;

/// The `if_expression` module, which handles `if` expressions/statements.
pub mod if_expression;

/// The `runtime` module, which handles `run` expressions
pub mod run;

/// The `tags` module, which handles tag parsing.
pub mod util;

/// The `block` module, which handles block expressions.
pub mod block;

/// The `function_call` module, which handles function calls.
pub mod function_call;

/// The `literals` module, which handles literal values.
pub mod literals;

use colored::Colorize as _;

use crate::{
	compile_time::{CompileTime, CompileTimeStatement, TranspileToC},
	context::Context,
	lexer::{Token, TokenType},
	object,
	parser::{
		expressions::{
			binary::BinaryExpression,
			block::Block,
			function_call::FunctionCall,
			if_expression::IfExpression,
			literals::{object::InternalValue, Literal, LiteralValue},
			run::{ParentExpression, RunExpression},
			util::{name::Name, types::Typed},
		},
		statements::Statement,
		Parse, TokenQueue,
	},
};

/// An expression in the language.
#[enum_dispatch::enum_dispatch(CompileTime)]
#[enum_dispatch::enum_dispatch(TranspileToC)]
#[enum_dispatch::enum_dispatch(Typed)]
#[enum_dispatch::enum_dispatch(ToCabin)]
#[enum_dispatch::enum_dispatch(ColoredCabin)]
#[enum_dispatch::enum_dispatch(ParentExpression)]
#[derive(Clone, Debug)]
pub enum Expression {
	Literal(Literal),
	FunctionCall(Box<FunctionCall>),
	BinaryExpression(Box<BinaryExpression>),
	IfStatement(Box<IfExpression>),
	Run(Box<RunExpression>),
	Block(Block),
}

impl Parse for Expression {
	type Output = Self;

	fn parse(tokens: &mut std::collections::VecDeque<Token>, context: &mut Context) -> anyhow::Result<Self::Output> {
		match tokens.peek().ok_or_else(|| anyhow::anyhow!("Unexpected EOF"))?.token_type {
			// If expressions
			TokenType::KeywordIf => Ok(Self::IfStatement(Box::new(IfExpression::parse(tokens, context)?))),

			// Run expression
			TokenType::KeywordRuntime => Ok(Self::Run(Box::new(RunExpression::parse(tokens, context)?))),

			// Block
			TokenType::LeftBrace => Ok(Self::Block(Block::parse(tokens, context)?)),

			// Binary expression
			_ => BinaryExpression::parse(tokens, context),
		}
	}
}

impl CompileTimeStatement for Expression {
	fn compile_time_evaluate_statement(&self, context: &mut Context, with_side_effects: bool) -> anyhow::Result<Statement> {
		Ok(Statement::Expression(self.compile_time_evaluate(context, with_side_effects)?))
	}
}

#[allow(clippy::as_conversions)]
impl From<usize> for Expression {
	fn from(value: usize) -> Self {
		object! {
			Number {
				internal_fields = {
					internal_value = InternalValue::Number(value as f64)
				}
			}
		}
	}
}

impl Expression {
	/// Returns the internal string value of this expression, if the expression represents a `Text` object in the
	/// language. If this expression is not an object or the object is not of type `Text`, an error is returned.
	///
	/// This is useful for things that require values to be strings, like some built-in functions. They can just
	/// call this, and if it's an error, they return an error because they need a string. A similar function exists
	/// for numbers, see `as_number()`.
	pub fn as_string(&self) -> anyhow::Result<String> {
		let Self::Literal(Literal(LiteralValue::Object(table), ..)) = self else {
			anyhow::bail!("Attempted to get an expression as a string, but it's not a string, it's a {self:?}\n");
		};

		if table.name != Name("Text".to_owned()) {
			anyhow::bail!(
				"Attempted to get an expression as Text, and it is an object, but the object is not an instance of Text, it's an instance of {}",
				table.name.cabin_name().bold().cyan()
			);
		}

		let Some(internal_value) = table.get_internal_field("internal_value") else {
			anyhow::bail!("Attempted to get an expression as a string, and it is a Text table, but it has no internal_value: {table:?}\n");
		};

		let InternalValue::String(internal_value_string) = internal_value else {
			anyhow::bail!(
				"Attempted to get an expression as a string, and it is a Text table, and it has an internal_value field, but that field is not a string, it's {internal_value:?}\n"
			);
		};

		Ok(internal_value_string.to_owned())
	}

	/// Returns the internal number value of this expression, if the expression represents a `Number` object in the
	/// language. If this expression is not an object or the object is not of type `Number`, an error is returned.
	///
	/// This is useful for things that require values to be numbers, like some built-in functions. They can just
	/// call this, and if it's an error, they return an error because they need a number. A similar function exists
	/// for strings, see `as_number()`.
	pub fn as_number(&self) -> anyhow::Result<f64> {
		let Self::Literal(Literal(LiteralValue::Object(table), ..)) = self else {
			anyhow::bail!("Attempted to get an expression as a number, but it's not a number, it's a {self:?}");
		};

		if table.name != Name("Number".to_owned()) {
			anyhow::bail!("Attempted to get an expression as a number, and it is a table, but the table is not Number: {self:?}");
		}

		let Some(internal_value) = table.get_internal_field("internal_value") else {
			anyhow::bail!("Attempted to get an expression as a number, and it is a Number table, but it has no internal_value: {table:?}");
		};

		let InternalValue::Number(internal_value_string) = internal_value else {
			anyhow::bail!(
				"Attempted to get an expression as a number, and it is a Number table, and it has an internal_value field, but that field is not a number, it's {internal_value:?}"
			);
		};

		Ok(internal_value_string.to_owned())
	}

	/// Returns the internal list value of this expression, if the expression represents a `List` object in the
	/// language. If this expression is not an object or the object is not of type `List`, an error is returned.
	pub fn as_list(&mut self) -> anyhow::Result<&mut Vec<Self>> {
		let Self::Literal(Literal(LiteralValue::Object(object), ..)) = self else {
			anyhow::bail!("Attempted to get an expression as a list, but it's not a list, it's a {self:?}");
		};

		if object.name != Name("List".to_owned()) {
			anyhow::bail!("Attempted to get an expression as a list, and it is a table, but the table is not List.");
		}

		let Some(internal_value) = object.get_internal_field_mut("internal_list") else {
			anyhow::bail!("Attempted to get an expression as a list, and it is a Number table, but it has no internal_list");
		};

		let InternalValue::List(internal_value_string) = internal_value else {
			anyhow::bail!(
				"Attempted to get an expression as a number, and it is a Number table, and it has an internal_value field, but that field is not a number, it's {internal_value:?}"
			);
		};

		Ok(internal_value_string)
	}

	/// Asserts that this expression is a literal. This is used to ensure that types are can be resolved into literals at compile-time.
	pub fn require_literal(&self, context: &mut Context) -> anyhow::Result<()> {
		self.as_literal(context)?;
		Ok(())
	}

	pub fn as_literal(&self, context: &mut Context) -> anyhow::Result<&Literal> {
		match self {
			Self::Literal(literal) => Ok(literal),
			_ => {
				context.add_error_details(format!("Although Cabin allows arbitrary expressions as types, it is still a statically typed language, so the expressions you use as types must be able to be evaluated at compile-time into a literal value."));
				anyhow::bail!("An expression was used as a type, but the expression couldn't be fully evaluated at compile-time into a literal value.")
			},
		}
	}
}
