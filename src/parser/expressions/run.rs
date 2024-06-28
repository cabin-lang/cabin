use crate::{
	compile_time::{CompileTime, TranspileToC},
	context::Context,
	formatter::{ColoredCabin, ToCabin},
	lexer::{Token, TokenType},
	parser::{
		expressions::{util::types::Typed, Expression},
		statements::Statement,
		Parse, TokenQueue,
	},
};

use std::collections::VecDeque;

use colored::Colorize as _;

use super::literals::Literal;

/// An expression that is forcibly run at runtime. By default, all expressions are evaluated at compile-time (if they are capable),
/// but this expression will be evaluated at runtime. Note that inner child expressions of this expression will still be run at
/// compile-time; It only forces the direct expression to be run at runtime.
#[derive(Debug, Clone)]
pub struct RunExpression {
	/// The inner expression to run at runtime.
	pub expression: Expression,
}

impl Parse for RunExpression {
	type Output = Self;

	fn parse(tokens: &mut VecDeque<Token>, context: &mut Context) -> anyhow::Result<Self::Output> {
		tokens.pop(TokenType::KeywordRuntime, context)?;
		Ok(Self {
			expression: Expression::parse(tokens, context)?,
		})
	}
}

impl Typed for RunExpression {
	fn get_type(&self, context: &mut Context) -> anyhow::Result<Literal> {
		self.expression.get_type(context)
	}
}

impl CompileTime for RunExpression {
	fn compile_time_evaluate(&self, context: &mut Context, _with_side_effects: bool) -> anyhow::Result<Expression> {
		self.expression.evaluate_children_at_compile_time(context).map_err(|error| {
			anyhow::anyhow!(
				"{error}\n\t{}",
				format!("while evaluating a \"{}\" expression at compile-time", "run".bold().cyan()).dimmed()
			)
		})
	}
}

impl TranspileToC for RunExpression {
	fn c_prelude(&self, context: &mut Context) -> anyhow::Result<String> {
		context.encountered_compiler_bug = true;
		anyhow::bail!("The compiler attempted to convert a \"{run}\" expression into C prelude code, which shouldn't happen;\n\"{run}\" expressions should be evaluated away during compile-time evaluation.\n", run = "run".bold().cyan());
	}

	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		context.encountered_compiler_bug = true;
		anyhow::bail!("The compiler attempted to convert a \"{run}\" expression into C code, which shouldn't happen;\n\"{run}\" expressions should be evaluated away during compile-time evaluation.\n", run = "run".bold().cyan());
	}
}

impl ToCabin for RunExpression {
	fn to_cabin(&self) -> String {
		format!("run {}", self.expression.to_cabin())
	}
}

impl ColoredCabin for RunExpression {
	fn to_colored_cabin(&self, context: &mut Context) -> String {
		format!("{} {}", "run".purple(), self.expression.to_colored_cabin(context))
	}
}

/// Indicates that an expression can evaluate it's sub expressions. This is used by `run` expressions, which should still
/// evaluate all children of the main expression at compile-time, but not evaluate the base expression itself. For
/// literals, which are the only expressions without sub-expressions, the literal can just be returned back.
#[enum_dispatch::enum_dispatch]
pub trait ParentExpression {
	/// Evaluates the child expressions of this expression, while leaving this expression untouched, and returns
	/// the resulting expression. This is used by `run` expressions to leave an expression un-evaluated at compile-time.
	fn evaluate_children_at_compile_time(&self, context: &mut Context) -> anyhow::Result<Expression>;
}

impl ParentExpression for RunExpression {
	fn evaluate_children_at_compile_time(&self, context: &mut Context) -> anyhow::Result<Expression> {
		Ok(Expression::Run(Box::new(Self {
			expression: self
				.expression
				.compile_time_evaluate(context, true)
				.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating the sub-expressions of a run expression at compile-time".dimmed()))?,
		})))
	}
}

impl<T: ParentExpression> ParentExpression for Box<T> {
	fn evaluate_children_at_compile_time(&self, context: &mut Context) -> anyhow::Result<Expression> {
		self.as_ref().evaluate_children_at_compile_time(context)
	}
}

/// A trait indicating that a statement can resolve it's child expressions at compile-time but not evaluate itself. This is currently only used for declaration statements,
/// which evaluate their type, and then evaluate the children of their expression (see `ParentExpression::evaluate_children_at_compile_time`).
#[enum_dispatch::enum_dispatch]
pub trait ParentStatement {
	/// Resolves a statement's child expressions at compile-time but does not evaluate itself. This is currently only used for declaration statements,
	/// which evaluate their type, and then evaluate the children of their expression (see `ParentExpression::evaluate_children_at_compile_time`).
	///
	/// # Parameters
	/// - `context` - Global data about the compiler, such as data about scopes and error messages.
	fn evaluate_statement_children_at_compile_time(&self, context: &mut Context) -> anyhow::Result<Statement>;
}

impl<T: ParentStatement> ParentStatement for Box<T> {
	fn evaluate_statement_children_at_compile_time(&self, context: &mut Context) -> anyhow::Result<Statement> {
		self.as_ref().evaluate_statement_children_at_compile_time(context)
	}
}
