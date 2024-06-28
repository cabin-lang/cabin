use std::collections::VecDeque;

use colored::Colorize;

use crate::{
	compile_time::{CompileTimeStatement, TranspileToC},
	context::Context,
	formatter::{ColoredCabin, ToCabin},
	lexer::{Token, TokenType},
	parser::{
		expressions::{block::Block, Expression},
		statements::Statement,
		Parse, TokenQueue,
	},
};

/// A while loop, which runs while the given condition is true
#[derive(Debug, Clone)]
pub struct WhileLoop {
	/// The condition of the while loop
	condition: Expression,
	/// The body of the while loop
	body: Block,
}

impl Parse for WhileLoop {
	type Output = Self;

	fn parse(tokens: &mut VecDeque<Token>, context: &mut Context) -> anyhow::Result<Self::Output> {
		tokens.pop(TokenType::KeywordWhile, context)?;
		let condition = Expression::parse(tokens, context)?;
		let body = Block::parse(tokens, context)?;
		Ok(Self { condition, body })
	}
}

impl CompileTimeStatement for WhileLoop {
	fn compile_time_evaluate_statement(&self, _context: &mut Context, _with_side_effects: bool) -> anyhow::Result<Statement> {
		Ok(Statement::WhileLoop(self.clone()))
	}
}

impl TranspileToC for WhileLoop {
	fn c_prelude(&self, context: &mut Context) -> anyhow::Result<String> {
		Ok([self.condition.c_prelude(context)?, self.body.c_prelude(context)?].join("\n"))
	}

	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		Ok(format!("while {} {}", self.condition.to_c(context)?, {
			let body = self.body.to_c(context)?;
			body.get(1..body.len() - 1).unwrap().to_owned()
		}))
	}
}

impl ToCabin for WhileLoop {
	fn to_cabin(&self) -> String {
		format!("while {} {}", self.condition.to_cabin(), self.body.to_cabin())
	}
}

impl ColoredCabin for WhileLoop {
	fn to_colored_cabin(&self, context: &mut Context) -> String {
		format!("{} {} {}", "while".purple(), self.condition.to_colored_cabin(context), self.body.to_colored_cabin(context))
	}
}
