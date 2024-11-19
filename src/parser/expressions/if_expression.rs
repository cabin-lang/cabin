use crate::{
	comptime::CompileTime,
	context::Context,
	lexer::TokenType,
	parser::{
		expressions::{block::Block, Expression},
		Parse, TokenQueue, TokenQueueFunctionality,
	},
};

#[derive(Debug, Clone)]
pub struct IfExpression {
	pub condition: Box<Expression>,
	pub body: Box<Expression>,
}

impl Parse for IfExpression {
	type Output = IfExpression;

	fn parse(tokens: &mut TokenQueue, context: &mut Context) -> anyhow::Result<Self::Output> {
		tokens.pop(TokenType::KeywordIf)?;
		let condition = Box::new(Expression::parse(tokens, context)?);
		let body = Box::new(Expression::Block(Block::parse(tokens, context)?));
		Ok(IfExpression { condition, body })
	}
}

impl CompileTime for IfExpression {
	type Output = Expression;

	fn evaluate_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self::Output> {
		// Check condition
		let condition = self.condition.evaluate_at_compile_time(context)?;
		let with_side_effects = condition.is_true(context);

		// Evaluate body
		context.toggle_side_effects(with_side_effects);
		let body = self.body.evaluate_at_compile_time(context)?;
		context.untoggle_side_effects();

		// Fully evaluated: return the value
		if with_side_effects {
			if let Ok(literal) = body.to_owned_literal() {
				return Ok(literal);
			}
		}

		// Non-literal: Return as an if-expression
		Ok(Expression::If(IfExpression {
			condition: Box::new(condition),
			body: Box::new(body),
		}))
	}
}
