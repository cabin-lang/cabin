use super::tag::TagList;
use crate::{
	api::context::Context,
	comptime::CompileTime,
	lexer::TokenType,
	mapped_err,
	parser::{
		expressions::{name::Name, Expression},
		Parse, TokenQueue, TokenQueueFunctionality as _,
	},
};

#[derive(Debug, Clone)]
pub struct Declaration {
	name: Name,
	scope_id: usize,
}

impl Declaration {
	pub fn value<'a>(&'a self, context: &'a Context) -> &'a Expression {
		context.scope_data.get_variable_from_id(self.name.clone(), self.scope_id).unwrap()
	}
}

impl Parse for Declaration {
	type Output = Declaration;

	fn parse(tokens: &mut TokenQueue, context: &mut Context) -> anyhow::Result<Self::Output> {
		// Tags
		let tags = if tokens.next_is(TokenType::TagOpening) {
			Some(TagList::parse(tokens, context)?)
		} else {
			None
		};

		// Name
		tokens.pop(TokenType::KeywordLet)?;
		let name = Name::parse(tokens, context)?;

		// Value
		tokens.pop(TokenType::Equal)?;
		let mut value = Expression::parse(tokens, context)?;

		// Tags
		if let Some(expression_tags) = value.tags_mut() {
			if let Some(declaration_tags) = &tags {
				*expression_tags = declaration_tags.clone();
			}
		}

		// Add the name declaration to the scope
		context.scope_data.declare_new_variable(name.clone(), value).map_err(mapped_err! {
			while = format!("while attempting to add the variable \"{}\" to its scope", name.unmangled_name().bold().cyan()),
			context = context,
		})?;

		// Return the declaration
		Ok(Declaration {
			name,
			scope_id: context.scope_data.unique_id(),
		})
	}
}

impl CompileTime for Declaration {
	type Output = Declaration;

	fn evaluate_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self::Output> {
		let value = context.scope_data.get_variable_from_id(self.name.clone(), self.scope_id).unwrap().clone();
		let evaluated = value.evaluate_at_compile_time(context).map_err(mapped_err! {
			while = format!(
				"while evaluating value of the initial declaration for the variable \"{}\" at compile-time",
				self.name.unmangled_name().bold().cyan()
			),
			context = context,
		})?;

		context.scope_data.reassign_variable_from_id(&self.name, evaluated, self.scope_id).map_err(mapped_err! {
			while = format!(
				"while attempting to reassign the variable \"{}\" to its evaluated value",
				self.name.unmangled_name().bold().cyan()
			),
			context = context,
		})?;

		Ok(Declaration {
			name: self.name,
			scope_id: self.scope_id,
		})
	}
}
