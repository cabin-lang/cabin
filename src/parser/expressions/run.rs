use crate::{
	api::context::Context,
	comptime::CompileTime,
	lexer::{Span, TokenType},
	parser::{expressions::Expression, Parse, TokenQueue, TokenQueueFunctionality as _},
	transpiler::TranspileToC,
};

use super::{Spanned, Typed};

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
	span: Span,
}

impl Parse for RunExpression {
	type Output = RunExpression;

	fn parse(tokens: &mut TokenQueue, context: &mut Context) -> anyhow::Result<Self::Output> {
		let mut span = tokens.pop(TokenType::KeywordRuntime)?.span;
		let expression = Box::new(Expression::parse(tokens, context)?);
		span = span.to(&expression.span());
		Ok(RunExpression { span, expression })
	}
}

impl CompileTime for RunExpression {
	type Output = RunExpression;

	fn evaluate_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self::Output> {
		Ok(RunExpression {
			expression: Box::new(self.expression.evaluate_subexpressions_at_compile_time(context)?),
			span: self.span,
		})
	}
}

impl TranspileToC for RunExpression {
	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		self.expression.to_c(context)
	}
}

impl Typed for RunExpression {
	fn get_type(&self, context: &mut Context) -> anyhow::Result<crate::comptime::memory::VirtualPointer> {
		self.expression.get_type(context)
	}
}

pub trait ParentExpression: Sized {
	fn evaluate_subexpressions_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self>;
}

impl Spanned for RunExpression {
	fn span(&self) -> Span {
		self.span.to_owned()
	}
}
