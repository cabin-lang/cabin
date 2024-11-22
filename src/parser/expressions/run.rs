use crate::{
	api::context::Context,
	comptime::CompileTime,
	lexer::TokenType,
	parser::{expressions::Expression, Parse, TokenQueue, TokenQueueFunctionality as _},
	transpiler::TranspileToC,
};

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

	fn evaluate_at_compile_time(self, _context: &mut Context) -> anyhow::Result<Self::Output> {
		// TODO: rah
		Ok(self)
	}
}

impl TranspileToC for RunExpression {
	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		self.expression.to_c(context)
	}
}
