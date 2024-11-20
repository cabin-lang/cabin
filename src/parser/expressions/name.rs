use std::hash::Hash;

use colored::Colorize as _;

use crate::{
	comptime::CompileTime,
	context::Context,
	lexer::{Position, TokenType},
	parser::{expressions::Expression, Parse, TokenQueue, TokenQueueFunctionality as _},
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

	fn ne(&self, other: &Self) -> bool {
		self.name != other.name
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

	fn parse(tokens: &mut TokenQueue, _context: &mut Context) -> anyhow::Result<Self::Output> {
		let token = tokens
			.pop(TokenType::Identifier)
			.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while attempting to parse a variable name".dimmed()))?;
		Ok(Name {
			name: token.value,
			position: Some(token.position),
		})
	}
}

impl CompileTime for Name {
	type Output = Expression;

	fn evaluate_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self::Output> {
		let value = &context.scope_data.get_variable(&self).ok_or_else(|| {
			anyhow::anyhow!(
				"Attempted to reference a variable named \"{}\", but no variable with that name exists where its referenced.\n\t{}",
				self.unmangled_name().bold().cyan(),
				format!("while evaluating a the name \"{}\" at compile-time", self.unmangled_name().bold().cyan()).dimmed()
			)
		})?;

		if let Expression::Pointer(address) = value {
			Ok(Expression::Pointer(*address))
		} else {
			Ok(Expression::Name(self))
		}
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
