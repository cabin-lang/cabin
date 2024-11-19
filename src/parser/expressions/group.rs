use std::collections::VecDeque;

use crate::{
	context::Context,
	lexer::{Token, TokenType},
	parse_list,
	parser::{expressions::object::Field, ListType, TokenQueueFunctionality},
};

use super::{name::Name, object::ObjectConstructor, Expression, Parse};

#[derive(Debug, Clone)]
pub struct GroupDeclaration;

impl Parse for GroupDeclaration {
	type Output = Expression;

	fn parse(tokens: &mut VecDeque<Token>, context: &mut Context) -> anyhow::Result<Self::Output> {
		tokens.pop(TokenType::KeywordGroup)?;

		// Fields
		let mut fields = Vec::new();
		parse_list!(tokens, ListType::Braced, {
			let name = Name::parse(tokens, context)?;
			tokens.pop(TokenType::Colon)?;
			let field_type = Some(Expression::parse(tokens, context)?);
			fields.push(Field { name, field_type, value: None });
		});

		Ok(Expression::Pointer(ObjectConstructor::group(fields, context.scope_data.unique_id(), context)))
	}
}
