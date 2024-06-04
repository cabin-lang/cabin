use crate::{
	cli::theme::Styled,
	compile_time::{CompileTime, CompileTimeStatement, TranspileToC},
	context::Context,
	formatter::{ColoredCabin, ToCabin},
	lexer::{Token, TokenType},
	parser::{expressions::Expression, statements::Statement, Parse, TokenQueue},
};

use std::collections::VecDeque;

use colored::Colorize as _;

/// A tail statement is a statement that is similar to a return statement, but it returns a value from the current scope instead of the enclosing function.
#[derive(Debug, Clone)]
pub struct TailStatement {
	/// The expression that's being returned from this tail statement.
	pub expression: Expression,
}

impl CompileTimeStatement for TailStatement {
	fn compile_time_evaluate_statement(&self, context: &mut Context, with_side_effects: bool) -> anyhow::Result<Statement> {
		Ok(Statement::Tail(Self {
			expression: self.expression.compile_time_evaluate(context, with_side_effects)?,
		}))
	}
}

impl Parse for TailStatement {
	type Output = Self;

	fn parse(tokens: &mut VecDeque<Token>, context: &mut Context) -> anyhow::Result<Self::Output> {
		tokens.pop(TokenType::KeywordTail, context).map_err(|error| {
			anyhow::anyhow!(anyhow::anyhow!(
				"{error}\n\t{}",
				format!("while attempting to parse the keyword \"{}\" for a tail statement.", "its".bold().cyan()).dimmed()
			))
		})?;

		Ok(Self {
			expression: Expression::parse(tokens, context).map_err(|error| {
				anyhow::anyhow!(
					"{error}\n\t{}",
					format!("while attempting to parse the expression in an \"{}\" statement", "its".bold().cyan()).dimmed()
				)
			})?,
		})
	}
}

impl TranspileToC for TailStatement {
	fn c_prelude(&self, context: &mut Context) -> anyhow::Result<String> {
		self.expression.c_prelude(context)
	}

	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		Ok(format!("{};", self.expression.to_c(context)?))
	}
}

impl ToCabin for TailStatement {
	fn to_cabin(&self) -> String {
		format!("its {};", self.expression.to_cabin())
	}
}

impl ColoredCabin for TailStatement {
	fn to_colored_cabin(&self, context: &mut Context) -> String {
		format!("{} {};", "its".style(context.theme().keyword()), self.expression.to_colored_cabin(context))
	}
}
