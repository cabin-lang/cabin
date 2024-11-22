use std::hash::Hash;

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
	span: Option<Span>,
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
			span: None,
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

	pub fn position(&self) -> Option<Span> {
		self.span.clone()
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
			span: Some(token.span),
			should_mangle: true,
		})
	}
}

impl CompileTime for Name {
	type Output = Expression;

	fn evaluate_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self::Output> {
		let (value, scope_id) = context.scope_data.pop_variable(&self).map_err(mapped_err! {
			while = format!("attempting to get the original value of the name \"{}\" to evaluate it at compile-time", self.unmangled_name().bold().cyan()),
			context = context,
		})?;
		let evaluated = value.evaluate_at_compile_time(context).map_err(mapped_err! {
			while = format!("evaluating the value of the name \"{}\" at compile-time", self.unmangled_name().bold().cyan()),
			context = context,
		})?;
		let result = evaluated.try_clone_pointer(context).unwrap_or(Expression::Name(self.clone()));
		context.scope_data.declare_new_variable_from_id(self.clone(), evaluated, scope_id)?;
		Ok(result)
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
			span: None,
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
	fn span(&self) -> Span {
		self.span.as_ref().unwrap().to_owned()
	}
}
