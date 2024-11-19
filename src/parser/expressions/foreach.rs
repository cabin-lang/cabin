use crate::{
	comptime::CompileTime,
	context::Context,
	lexer::TokenType,
	parser::{statements::tag::TagList, Parse, TokenQueue, TokenQueueFunctionality as _},
};

use super::{block::Block, name::Name, Expression};

#[derive(Debug, Clone)]
pub struct ForEachLoop {
	binding_name: Name,
	iterable: Box<Expression>,
	body: Box<Expression>,
	inner_scope_id: usize,
}

impl Parse for ForEachLoop {
	type Output = ForEachLoop;

	fn parse(tokens: &mut TokenQueue, context: &mut Context) -> anyhow::Result<Self::Output> {
		tokens.pop(TokenType::KeywordForEach)?;
		let binding_name = Name::parse(tokens, context)?;
		tokens.pop(TokenType::KeywordIn)?;
		let iterable = Box::new(Expression::parse(tokens, context)?);
		let body = Block::parse(tokens, context)?;
		let inner_scope_id = body.inner_scope_id;
		context
			.scope_data
			.declare_new_variable_from_id(binding_name.clone(), Expression::Void, TagList::default(), inner_scope_id)?;

		Ok(ForEachLoop {
			binding_name,
			body: Box::new(Expression::Block(body)),
			iterable,
			inner_scope_id,
		})
	}
}

impl CompileTime for ForEachLoop {
	type Output = Expression;

	fn evaluate_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self::Output> {
		if let Ok(literal) = self.iterable.as_literal(context) {
			let elements = literal.list_elements()?.to_owned();
			for element in elements {
				context.scope_data.reassign_variable_from_id(&self.binding_name, element.clone(), self.inner_scope_id)?; // TODO: sneaky clone
				let value = self.body.clone().evaluate_at_compile_time(context)?;
				if value.is_literal() {
					return Ok(value);
				}
			}
		}

		Ok(Expression::ForEachLoop(self))
	}
}
