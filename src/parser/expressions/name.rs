use std::hash::Hash;

use colored::Colorize;

use crate::{
	api::context::Context,
	comptime::CompileTime,
	lexer::{Span, TokenType},
	mapped_err,
	parser::{expressions::Expression, Parse, ToCabin, TokenQueue, TokenQueueFunctionality as _},
	transpiler::TranspileToC,
};

use super::Spanned;

#[derive(Debug, Clone, Eq)]
pub struct Name {
	name: String,
	span: Span,
	should_mangle: bool,
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
	pub fn non_mangled<T: AsRef<str>>(name: T) -> Name {
		Name {
			name: name.as_ref().to_owned(),
			span: Span::zero(),
			should_mangle: false,
		}
	}

	pub fn unmangled_name(&self) -> String {
		self.name.clone()
	}

	pub fn mangled_name(&self) -> String {
		if self.should_mangle {
			format!("u_{}", self.name)
		} else {
			self.unmangled_name()
		}
	}
}

impl Parse for Name {
	type Output = Self;

	fn parse(tokens: &mut TokenQueue, context: &mut Context) -> anyhow::Result<Self::Output> {
		let position = tokens.current_position();

		let token = tokens.pop(TokenType::Identifier).map_err(mapped_err! {
			while = "attempting to parse a variable name",
			context = context,
			position = position.unwrap_or_else(Span::zero),
		})?;

		Ok(Name {
			name: token.value,
			span: token.span,
			should_mangle: true,
		})
	}
}

impl CompileTime for Name {
	type Output = Expression;

	fn evaluate_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self::Output> {
		let value = context
			.scope_data
			.get_variable(self.clone())
			.ok_or_else(|| anyhow::anyhow!("No variable found with the name {}", self.unmangled_name().bold().cyan()))
			.map_err(mapped_err! {
				while = format!("attempting to get the original value of the name \"{}\" to evaluate it at compile-time", self.unmangled_name().bold().cyan()),
				context = context,
				position = self.span(context),
				details = unindent::unindent(&format!(
					"
					Here you reference a variable called \"{name}\", but no variable called \"{name}\" exists at this
					part of the program. If this is a typo and you don't expect a variable with this name to exist, you
					may be trying to refer to one of these variables, which are the ones with the closest names that are
					present here:

					{closest}
					", 
					name = self.unmangled_name().bold().red(),
					closest = context
						.scope_data
						.get_closest_variables(&self, 3)
						.iter()
						.map(|(name, _)| format!("    - {}", name.unmangled_name().bold().green()))
						.collect::<Vec<_>>()
						.join("\n")
						.trim_start()
				))
			})?;

		Ok(value.try_clone_pointer(context).unwrap_or(Expression::Name(self.clone())))
	}
}

impl TranspileToC for Name {
	fn to_c(&self, _context: &mut Context) -> anyhow::Result<String> {
		Ok(self.mangled_name())
	}
}

impl<T: AsRef<str>> From<T> for Name {
	fn from(value: T) -> Self {
		Name {
			name: value.as_ref().to_owned(),
			span: Span::zero(),
			should_mangle: true,
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

impl Spanned for Name {
	fn span(&self, _context: &Context) -> Span {
		self.span.clone()
	}
}
