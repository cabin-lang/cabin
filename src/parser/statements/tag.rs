use std::ops::Deref;

use crate::{
	comptime::CompileTime,
	mapped_err, parse_list,
	parser::{expressions::Expression, ListType, Parse, TokenQueue},
};

#[derive(Debug, Clone, Default)]
pub struct TagList {
	pub values: Vec<Expression>,
}

impl Parse for TagList {
	type Output = TagList;

	fn parse(tokens: &mut TokenQueue) -> anyhow::Result<Self::Output> {
		let mut tags = Vec::new();
		parse_list!(tokens, ListType::Tag, {
			tags.push(Expression::parse(tokens)?);
		});
		Ok(TagList { values: tags })
	}
}

impl CompileTime for TagList {
	type Output = TagList;

	fn evaluate_at_compile_time(self) -> anyhow::Result<Self::Output> {
		let mut values = Vec::new();
		for value in self.values {
			let evaluated = value.evaluate_at_compile_time().map_err(mapped_err! {
				while = "evaluating a tag at compile-time",
			})?;
			values.push(evaluated);
		}
		Ok(TagList { values })
	}
}

impl Deref for TagList {
	type Target = Vec<Expression>;

	fn deref(&self) -> &Self::Target {
		&self.values
	}
}

impl From<Vec<Expression>> for TagList {
	fn from(values: Vec<Expression>) -> Self {
		Self { values }
	}
}
