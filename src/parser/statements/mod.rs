use crate::{
	api::context::Context,
	comptime::CompileTime,
	lexer::TokenType,
	mapped_err,
	parser::{
		expressions::Expression,
		statements::{declaration::Declaration, tail::TailStatement},
		Parse, TokenQueue, TokenQueueFunctionality as _,
	},
	transpiler::TranspileToC,
};

pub mod declaration;
pub mod tag;
pub mod tail;

#[derive(Debug, Clone)]
pub enum Statement {
	Declaration(Declaration),
	Tail(TailStatement),
	Expression(Expression),
}

impl Parse for Statement {
	type Output = Statement;

	fn parse(tokens: &mut TokenQueue, context: &mut Context) -> anyhow::Result<Self::Output> {
		let statement = match tokens.peek_type()? {
			TokenType::KeywordLet | TokenType::TagOpening => Statement::Declaration(Declaration::parse(tokens, context)?),
			TokenType::Identifier => {
				if tokens.peek_type2()? == &TokenType::KeywordIs {
					Statement::Tail(TailStatement::parse(tokens, context)?)
				} else {
					Statement::Expression(Expression::parse(tokens, context)?)
				}
			},
			_ => Statement::Expression(Expression::parse(tokens, context)?),
		};
		tokens.pop(TokenType::Semicolon)?;
		Ok(statement)
	}
}

impl CompileTime for Statement {
	type Output = Statement;

	fn evaluate_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self::Output> {
		Ok(match self {
			Statement::Declaration(declaration) => Statement::Declaration(declaration.evaluate_at_compile_time(context).map_err(mapped_err! {
				while = "evaluating a name declaration at compile-time",
				context = context,
			})?),
			Statement::Expression(expression) => Statement::Expression(expression.evaluate_at_compile_time(context).map_err(mapped_err! {
				while = "evaluating an expression statement at compile-time",
				context = context,
			})?),
			Statement::Tail(tail) => Statement::Tail(tail.evaluate_at_compile_time(context).map_err(mapped_err! {
				while = format!("evaluating a {} at compile-time", "tail statement".bold().cyan()),
				context = context,
			})?),
		})
	}
}

impl TranspileToC for Statement {
	fn to_c(&self, context: &Context) -> anyhow::Result<String> {
		Ok(match self {
			Statement::Declaration(declaration) => declaration.to_c(context)?,
			Statement::Tail(tail_statement) => tail_statement.to_c(context)?,
			Statement::Expression(expression) => expression.to_c(context)? + ";",
		})
	}
}
