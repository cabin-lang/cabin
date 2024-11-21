use std::hash::Hash;

use colored::Colorize as _;

use crate::{
	api::context::Context,
	comptime::CompileTime,
	lexer::{Position, TokenType},
	mapped_err,
	parser::{expressions::Expression, Parse, ToCabin, TokenQueue, TokenQueueFunctionality as _},
};

#[derive(Debug, Clone, Eq)]
pub struct Name {
	name: String,
	position: Option<Position>,
}

impl PartialEq for Name {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
	}
}

impl Hash for Name {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.name.hash(state);
	}
}

impl Name {
	pub fn unmangled_name(&self) -> String {
		self.name.clone()
	}

	pub fn position(&self) -> Option<Position> {
		self.position.clone()
	}
}

impl Parse for Name {
	type Output = Self;

	fn parse(tokens: &mut TokenQueue, context: &mut Context) -> anyhow::Result<Self::Output> {
		let position = tokens.current_position();

		let token = tokens.pop(TokenType::Identifier).map_err(mapped_err! {
			while = "attempting to parse a variable name",
			context = context,
			position = position.unwrap_or_else(Position::zero),
		})?;

		Ok(Name {
			name: token.value,
			position: Some(token.position),
		})
	}
}

impl CompileTime for Name {
	type Output = Expression;

	fn evaluate_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self::Output> {
		let value = &context.scope_data.get_variable(self.clone()).ok_or_else(|| {
			anyhow::anyhow!(
				"Attempted to reference a variable named \"{}\", but no variable with that name exists where its referenced.\n\t{}",
				self.unmangled_name().bold().cyan(),
				format!("while evaluating a the name \"{}\" at compile-time", self.unmangled_name().bold().cyan()).dimmed()
			)
		})?;

		Ok(value.try_clone_pointer().unwrap_or(Expression::Name(self)))
	}
}

impl<T: AsRef<str>> From<T> for Name {
	fn from(value: T) -> Self {
		Name {
			name: value.as_ref().to_owned(),
			position: None,
		}
	}
}

impl AsRef<Name> for Name {
	fn as_ref(&self) -> &Name {
		self
	}
}

impl ToCabin for Name {
	fn to_cabin(&self) -> String {
		self.unmangled_name()
	}
}
