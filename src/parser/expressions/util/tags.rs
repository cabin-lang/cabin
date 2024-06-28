use crate::{
	context::Context,
	lexer::{Token, TokenType},
	parse_list,
	parser::{expressions::Expression, Parse, TokenQueue},
};

use std::{
	collections::VecDeque,
	ops::{Deref, DerefMut},
};

/// A list of tags. Tags are values that can be present on declarations, such as variable declarations, group fields, etc. Tags do
/// not modify the value in any way, they are simply markers that can be checked at compile-time and runtime.
#[derive(Debug, Clone, Default)]
pub struct TagList {
	/// The tags in this tag list as a vector of expressions
	pub tags: Vec<Expression>,
}

impl IntoIterator for TagList {
	type Item = Expression;
	type IntoIter = std::vec::IntoIter<Expression>;

	fn into_iter(self) -> Self::IntoIter {
		self.tags.into_iter()
	}
}

impl TagList {
	/// Creates a new `TagList` with the given tags.
	///
	/// # Parameters
	/// - `tags` - The tags in this tag list
	pub fn new(tags: Vec<Expression>) -> Self {
		Self { tags }
	}
}

impl Deref for TagList {
	type Target = Vec<Expression>;

	fn deref(&self) -> &Self::Target {
		&self.tags
	}
}

impl DerefMut for TagList {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.tags
	}
}

impl Parse for TagList {
	type Output = Self;

	fn parse(tokens: &mut VecDeque<Token>, context: &mut Context) -> anyhow::Result<Self::Output> {
		let mut tags = Vec::new();
		if tokens.next_is(TokenType::TagOpening) {
			tokens.pop(TokenType::TagOpening, context).unwrap_or_else(|_error| unreachable!());
			parse_list!(tokens, context, {
				tags.push(Expression::parse(tokens, context).map_err(|error| anyhow::anyhow!("{error}\n\twhile parsing the value of an tag"))?);
			});
			tokens
				.pop(TokenType::RightBracket, context)
				.map_err(|error| anyhow::anyhow!("{error}: Tags, which are opened with \"#[\", must be closed with a right bracket \"]\". If you intended to add another tag here, you must separate your tags with a comma (\",\")"))?;
		}
		Ok(Self { tags })
	}
}
