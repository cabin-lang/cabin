use crate::{
	comptime::CompileTime,
	lexer::TokenType,
	mapped_err,
	parser::{
		expressions::Expression,
		statements::{declaration::Declaration, tail::TailStatement},
		Parse,
		TokenQueue,
		TokenQueueFunctionality as _,
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

	fn parse(tokens: &mut TokenQueue) -> anyhow::Result<Self::Output> {
		let statement = match tokens.peek_type()? {
			TokenType::KeywordLet | TokenType::TagOpening => Declaration::parse(tokens)?,
			TokenType::Identifier => {
				if tokens.peek_type2()? == TokenType::KeywordIs {
					let tail = Statement::Tail(TailStatement::parse(tokens)?);
					let _ = tokens.pop(TokenType::Semicolon)?;
					tail
				} else {
					let expression = Statement::Expression(Expression::parse(tokens)?);
					let _ = tokens.pop(TokenType::Semicolon)?;
					expression
				}
			},
			_ => {
				let expression = Statement::Expression(Expression::parse(tokens)?);
				let _ = tokens.pop(TokenType::Semicolon)?;
				expression
			},
		};
		Ok(statement)
	}
}

impl CompileTime for Statement {
	type Output = Statement;

	fn evaluate_at_compile_time(self) -> anyhow::Result<Self::Output> {
		Ok(match self {
			Statement::Declaration(declaration) => Statement::Declaration(declaration.evaluate_at_compile_time().map_err(mapped_err! {
				while = "evaluating a name declaration at compile-time",
			})?),
			Statement::Expression(expression) => Statement::Expression(expression.evaluate_at_compile_time().map_err(mapped_err! {
				while = "evaluating an expression statement at compile-time",
			})?),
			Statement::Tail(tail) => Statement::Tail(tail.evaluate_at_compile_time().map_err(mapped_err! {
				while = format!("evaluating a {} at compile-time", "tail statement".bold().cyan()),
			})?),
		})
	}
}

impl TranspileToC for Statement {
	fn to_c(&self) -> anyhow::Result<String> {
		Ok(match self {
			Statement::Declaration(declaration) => declaration.to_c()?,
			Statement::Tail(tail_statement) => tail_statement.to_c()?,
			Statement::Expression(expression) => expression.to_c()? + ";",
		})
	}
}
