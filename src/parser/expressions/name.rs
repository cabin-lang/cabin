use std::hash::Hash;

use crate::{
	api::context::Context,
	comptime::CompileTime,
	lexer::{Position, TokenType},
	mapped_err,
	parser::{expressions::Expression, Parse, ToCabin, TokenQueue, TokenQueueFunctionality as _},
	transpiler::TranspileToC,
};

#[derive(Debug, Clone, Eq)]
pub struct Name {
	name: String,
	position: Option<Position>,
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
			position: None,
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
			position: None,
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

pub trait NameOption {
	fn to_c_or_pointer(&self, address: usize) -> String;
	fn with_field(&self, field_name: &Name) -> Option<Name>;
}

impl NameOption for Option<Name> {
	fn to_c_or_pointer(&self, address: usize) -> String {
		self.clone().map(|name| name.mangled_name()).unwrap_or_else(|| format!("POINTER_{address}"))
	}

	fn with_field(&self, field_name: &Name) -> Option<Name> {
		self.clone().map(|name| format!("{}_{}", name.unmangled_name(), field_name.unmangled_name()).into())
	}
}
