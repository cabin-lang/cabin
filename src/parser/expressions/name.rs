use colored::Colorize as _;

use crate::{
	comptime::CompileTime,
	context::Context,
	lexer::TokenType,
	parser::{expressions::Expression, Parse, TokenQueue, TokenQueueFunctionality as _},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Name {
	name: String,
}

impl Name {
	pub fn unmangled_name(&self) -> String {
		self.name.clone()
	}
}

impl Parse for Name {
	type Output = Self;

	fn parse(tokens: &mut TokenQueue, _context: &mut Context) -> anyhow::Result<Self::Output> {
		Ok(Name {
			name: tokens
				.pop(TokenType::Identifier)
				.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while attempting to parse a variable name".dimmed()))?,
		})
	}
}

impl CompileTime for Name {
	type Output = Expression;

	fn evaluate_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self::Output> {
		let value = &context
			.scope_data
			.get_variable(&self)
			.ok_or_else(|| {
				anyhow::anyhow!(
					"Attempted to reference a variable named \"{}\", but no variable with that name exists where its referenced.\n\t{}",
					self.unmangled_name().bold().cyan(),
					format!("while evaluating a the name \"{}\" at compile-time", self.unmangled_name().bold().cyan()).dimmed()
				)
			})?
			.value;

		if let Expression::Pointer(address) = value {
			Ok(Expression::Pointer(*address))
		} else {
			Ok(Expression::Name(self))
		}
	}
}

impl<T: AsRef<str>> From<T> for Name {
	fn from(value: T) -> Self {
		Name { name: value.as_ref().to_owned() }
	}
}
