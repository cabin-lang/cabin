use crate::{
	compile_time::{CompileTime, CompileTimeStatement, TranspileToC},
	context::Context,
	formatter::{ColoredCabin, ToCabin},
	lexer::{Token, TokenType},
	parser::{
		expressions::{binary::BinaryExpression, Expression},
		statements::Statement,
		Parse, TokenQueue,
	},
	var,
};

use std::collections::VecDeque;

use colored::Colorize as _;

/// A return statement. This is a statement that returns a value from a function.
#[derive(Clone, Debug)]
pub struct ReturnStatement {
	/// The expression to return. This is `None` if the return statement has no expression.
	pub expression: Option<Expression>,

	/// The ID of the scope that the return statement appears in. This is used to convert the return statement into an assignment statement.
	pub scope_id: usize,
}

impl Parse for ReturnStatement {
	type Output = Self;

	fn parse(tokens: &mut VecDeque<Token>, context: &mut Context) -> anyhow::Result<Self::Output> {
		tokens.pop(TokenType::KeywordReturn, context)?;
		if tokens.next_is(TokenType::Semicolon) {
			Ok(Self {
				expression: None,
				scope_id: context.scope_data.unique_id(),
			})
		} else {
			let expression = Expression::parse(tokens, context)?;
			Ok(Self {
				expression: Some(expression),
				scope_id: context.scope_data.unique_id(),
			})
		}
	}
}

impl CompileTimeStatement for ReturnStatement {
	fn compile_time_evaluate_statement(&self, context: &mut Context, with_side_effects: bool) -> anyhow::Result<Statement> {
		let evaluated = self
			.expression
			.as_ref()
			.map(|expression| expression.compile_time_evaluate(context, with_side_effects))
			.transpose()?;

		// This is a false positive warning - `Option::map_or_else()` will cause an ownership error here
		#[allow(clippy::option_if_let_else)]
		if let Some(expression) = evaluated {
			Ok(Statement::Expression(Expression::BinaryExpression(Box::new(BinaryExpression {
				left: var!("return_address", self.scope_id),
				operator: TokenType::Equal,
				right: expression,
			}))))
		} else {
			Ok(Statement::ReturnStatement(Self {
				expression: evaluated,
				scope_id: self.scope_id,
			}))
		}
	}
}

impl TranspileToC for ReturnStatement {
	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		let mut c = String::new();
		c.push_str("return ");
		if let Some(expression) = &self.expression {
			c.push_str(&expression.to_c(context)?);
		}
		c.push(';');
		Ok(c)
	}

	fn c_prelude(&self, context: &mut Context) -> anyhow::Result<String> {
		self.expression.as_ref().map_or(Ok(String::new()), |expression| expression.c_prelude(context))
	}
}

impl ToCabin for ReturnStatement {
	fn to_cabin(&self) -> String {
		format!("return {}", self.expression.as_ref().map_or(String::new(), |expression| expression.to_cabin()))
	}
}

impl ColoredCabin for ReturnStatement {
	fn to_colored_cabin(&self, context: &mut Context) -> String {
		format!(
			"{} {}",
			"return".purple(),
			self.expression.as_ref().map_or(String::new(), |expression| expression.to_colored_cabin(context))
		)
	}
}
