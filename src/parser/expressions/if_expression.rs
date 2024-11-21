use crate::{
	api::context::Context,
	comptime::CompileTime,
	lexer::TokenType,
	mapped_err,
	parser::{
		expressions::{block::Block, Expression},
		Parse, TokenQueue, TokenQueueFunctionality,
	},
	transpiler::TranspileToC,
};

#[derive(Debug, Clone)]
pub struct IfExpression {
	pub condition: Box<Expression>,
	pub body: Box<Expression>,
	pub else_body: Option<Box<Expression>>,
}

impl Parse for IfExpression {
	type Output = IfExpression;

	fn parse(tokens: &mut TokenQueue, context: &mut Context) -> anyhow::Result<Self::Output> {
		tokens.pop(TokenType::KeywordIf)?;
		let condition = Box::new(Expression::parse(tokens, context)?);
		let body = Box::new(Expression::Block(Block::parse(tokens, context)?));
		let else_body = if tokens.next_is(TokenType::KeywordOtherwise) {
			tokens.pop(TokenType::KeywordOtherwise).unwrap_or_else(|_| unreachable!());
			Some(Box::new(Expression::Block(Block::parse(tokens, context)?)))
		} else {
			None
		};
		Ok(IfExpression { condition, body, else_body })
	}
}

impl CompileTime for IfExpression {
	type Output = Expression;

	fn evaluate_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self::Output> {
		// Check condition
		let condition = self.condition.evaluate_at_compile_time(context).map_err(mapped_err! {
			while = "evaluating the condition of an if-expression at compile-time",
			context = context,
		})?;
		let condition_is_true = condition.is_true(context);

		// Evaluate body
		context.toggle_side_effects(condition_is_true);
		let body = self
			.body
			.evaluate_at_compile_time(context)
			.map_err(mapped_err! { while = "evaluating the body of an if-expression at compile-time", context = context, })?;
		context.untoggle_side_effects();

		// Evaluate else body
		context.toggle_side_effects(!condition_is_true);
		let else_body = self
			.else_body
			.map(|else_body| {
				anyhow::Ok(Box::new(else_body.evaluate_at_compile_time(context).map_err(mapped_err! {
					while = "evaluating the otherwise-body of an if-expression at compile-time",
					context = context,
				})?))
			})
			.transpose()?;
		context.untoggle_side_effects();

		// Fully evaluated: return the value (only if true)
		if condition_is_true {
			if let Ok(literal) = body.try_clone_pointer(context) {
				return Ok(literal);
			}
		} else if let Some(else_body) = &else_body {
			if let Ok(literal) = else_body.try_clone_pointer(context) {
				return Ok(literal);
			}
		}

		// Non-literal: Return as an if-expression
		Ok(Expression::If(IfExpression {
			condition: Box::new(condition),
			body: Box::new(body),
			else_body,
		}))
	}
}

impl TranspileToC for IfExpression {
	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		let mut builder = format!("({}) ? (", self.condition.to_c(context)?);
		for line in self.body.to_c(context)?.lines() {
			builder += &format!("\n\t{line}");
		}
		builder += "\n) : (";

		if let Some(else_body) = &self.else_body {
			for line in else_body.to_c(context)?.lines() {
				builder += &format!("\n\t{line}");
			}
		} else {
			builder += "\nNULL"
		}

		builder += "\n) ";

		Ok(builder)
	}
}
