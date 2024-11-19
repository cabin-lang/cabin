use super::object::ObjectConstructor;
use crate::{
	context::Context,
	list,
	parse_list,
	parser::{expressions::Expression, ListType, Parse, TokenQueue},
};

pub struct List;

impl Parse for List {
	type Output = Expression;

	fn parse(tokens: &mut TokenQueue, context: &mut Context) -> anyhow::Result<Self::Output> {
		let mut list = Vec::new();
		parse_list!(tokens, ListType::Bracketed, { list.push(Expression::parse(tokens, context)?) });
		let list = list!(context, context.scope_data.unique_id(), list);
		Ok(list)
	}
}
