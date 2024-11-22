use crate::{
	api::context::Context,
	comptime::CompileTime,
	lexer::TokenType,
	parser::{expressions::Expression, Parse, TokenQueue, TokenQueueFunctionality as _},
	transpiler::TranspileToC,
};

/// A `Run` expression in the language. Run-expressions forcibly run an expression at runtime instead of compile-time. Since
/// Cabin runs all code at compile-time by default, this is the only way to forcibly run an expression at runtime.
///
/// Note that an expressions sub-expressions are still run at compile-time. For example, consider the expression:
///
/// ```
/// run ((1 + 2) + (3 + 4))
/// ```
///
/// This evaluates at compile-time to:
///
/// ```
/// run (3 + 7)
/// ```
///
/// To fully run the entire expression at runtime, one would have to nest run expressions:
///
/// ```
/// run (run (1 + 2) + run (3 + 4))
/// ```
///
/// The syntax for this expression is:
///
/// `run <expression>`
#[derive(Debug, Clone)]
pub struct RunExpression {
	expression: Box<Expression>,
}

impl Parse for RunExpression {
	type Output = RunExpression;

	fn parse(tokens: &mut TokenQueue, context: &mut Context) -> anyhow::Result<Self::Output> {
		tokens.pop(TokenType::KeywordRuntime)?;
		Ok(RunExpression {
			expression: Box::new(Expression::parse(tokens, context)?),
		})
	}
}

impl CompileTime for RunExpression {
	type Output = RunExpression;

	fn evaluate_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self::Output> {
		Ok(RunExpression {
			expression: Box::new(self.expression.evaluate_subexpressions_at_compile_time(context)?),
		})
	}
}

impl TranspileToC for RunExpression {
	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		self.expression.to_c(context)
	}
}

pub trait ParentExpression: Sized {
	fn evaluate_subexpressions_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self>;
}
