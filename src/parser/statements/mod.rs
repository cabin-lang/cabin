use crate::{
	compile_time::{CompileTimeStatement, TranspileToC},
	context::Context,
	formatter::{ColoredCabin, ToCabin},
	lexer::{Token, TokenType},
	parser::{
		expressions::{run::ParentExpression, run::ParentStatement, Expression},
		statements::{declaration::Declaration, return_statement::ReturnStatement, tail::TailStatement},
		Parse, TokenQueue,
	},
};

use colored::Colorize as _;

use self::{foreach::ForEachLoop, while_loop::WhileLoop};

/// The declaration module, which handles variable declarations.
pub mod declaration;
/// The return statement module, which handles return statements.
pub mod return_statement;

/// The tail module, which handles tail statements, which allow returning a value from a block.
pub mod tail;

/// The `foreach` module, which handles for loops.
mod foreach;

/// The `while_loop` module, which handles while loops.
mod while_loop;

/// A statement in the language. A statement can only occur at the top level of a program or within a function body.
#[derive(Clone, Debug)]
#[enum_dispatch::enum_dispatch(CompileTimeStatement)]
pub enum Statement {
	Declaration(Declaration),
	ReturnStatement(ReturnStatement),
	Expression(Expression),
	Tail(TailStatement),
	ForEachLoop(ForEachLoop),
	WhileLoop(WhileLoop),
}

impl Parse for Statement {
	type Output = Self;

	fn parse(tokens: &mut std::collections::VecDeque<Token>, context: &mut Context) -> anyhow::Result<Self::Output> {
		let parsed = Ok(match tokens.peek().ok_or_else(|| anyhow::anyhow!("Unexpected EOF"))?.token_type {
			TokenType::KeywordLet | TokenType::TagOpening => Self::Declaration(Declaration::parse(tokens, context)?),
			TokenType::KeywordReturn => Self::ReturnStatement(ReturnStatement::parse(tokens, context)?),
			TokenType::KeywordTail => Self::Tail(TailStatement::parse(tokens, context)?),
			TokenType::KeywordForEach => Self::ForEachLoop(ForEachLoop::parse(tokens, context)?),
			TokenType::KeywordWhile => Self::WhileLoop(WhileLoop::parse(tokens, context)?),
			_ => Self::Expression(Expression::parse(tokens, context)?),
		});
		tokens.pop(crate::lexer::TokenType::Semicolon, context)?;
		parsed
	}
}

impl ParentStatement for Statement {
	fn evaluate_statement_children_at_compile_time(&self, context: &mut Context) -> anyhow::Result<Statement> {
		Ok(match self {
			Self::Declaration(declaration) => declaration.evaluate_statement_children_at_compile_time(context).map_err(|error| {
				anyhow::anyhow!(
					"{error}\n\t{}",
					format!(
						"while evaluating the sub-expressions of the declaration of the variable {} at compile-time",
						declaration.name.cabin_name().bold().cyan()
					)
					.dimmed()
				)
			})?,
			Self::Expression(expression) => {
				let evaluated = match expression {
					Expression::Run(run_expression) => run_expression.expression.evaluate_children_at_compile_time(context)?,
					_ => expression
						.evaluate_children_at_compile_time(context)
						.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating the sub-expressions of an expression at compile-time".dimmed()))?,
				};
				Self::Expression(evaluated)
			},
			Self::Tail(tail) => Self::Tail(TailStatement {
				expression: tail.expression.evaluate_children_at_compile_time(context)?,
			}),
			Self::ForEachLoop(for_each_loop) => for_each_loop.compile_time_evaluate_statement(context, false)?,
			Self::WhileLoop(while_loop) => while_loop.compile_time_evaluate_statement(context, false)?,
			_ => unreachable!(),
		})
	}
}

impl TranspileToC for Statement {
	fn c_prelude(&self, context: &mut Context) -> anyhow::Result<String> {
		match self {
			Self::Declaration(declaration) => declaration.c_prelude(context),
			Self::ReturnStatement(return_statement) => return_statement.c_prelude(context),
			Self::Expression(expression) => expression.c_prelude(context),
			Self::Tail(tail) => tail.c_prelude(context),
			Self::ForEachLoop(foreach) => foreach.c_prelude(context),
			Self::WhileLoop(while_loop) => while_loop.c_prelude(context),
		}
	}

	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		Ok(match self {
			Self::Declaration(declaration) => declaration.to_c(context)?,
			Self::ReturnStatement(return_statement) => return_statement.to_c(context)?,
			Self::Expression(expression) => format!("{};", expression.to_c(context)?),
			Self::Tail(tail) => tail.to_c(context)?,
			Self::ForEachLoop(foreach) => foreach.to_c(context)?,
			Self::WhileLoop(while_loop) => while_loop.to_c(context)?,
		})
	}
}

impl ToCabin for Statement {
	fn to_cabin(&self) -> String {
		match self {
			Self::Declaration(declaration) => declaration.to_cabin(),
			Self::ReturnStatement(return_statement) => return_statement.to_cabin(),
			Self::Expression(expression) => format!("{};", expression.to_cabin()),
			Self::Tail(tail) => tail.to_cabin(),
			Self::ForEachLoop(foreach) => foreach.to_cabin(),
			Self::WhileLoop(while_loop) => while_loop.to_cabin(),
		}
	}
}

impl ColoredCabin for Statement {
	fn to_colored_cabin(&self, context: &mut Context) -> String {
		match self {
			Self::Declaration(declaration) => declaration.to_colored_cabin(context),
			Self::ReturnStatement(return_statement) => return_statement.to_colored_cabin(context),
			Self::Expression(expression) => format!("{};", expression.to_colored_cabin(context)),
			Self::Tail(tail) => tail.to_colored_cabin(context),
			Self::ForEachLoop(foreach) => foreach.to_colored_cabin(context),
			Self::WhileLoop(while_loop) => while_loop.to_colored_cabin(context),
		}
	}
}
