use crate::{
	api::context::Context,
	comptime::CompileTime,
	lexer::TokenType,
	mapped_err,
	parser::{
		expressions::{name::Name, Expression},
		statements::tag::TagList,
		Parse, TokenQueue, TokenQueueFunctionality as _,
	},
	transpiler::TranspileToC,
};

#[derive(Debug, Clone)]
pub struct Declaration {
	name: Name,
	scope_id: usize,
	initial_value: Expression,
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
		context.scope_label = Some(name.clone());

		// Value
		tokens.pop(TokenType::Equal)?;
		let mut value = Expression::parse(tokens, context)?;
		let initial_value = value.clone();

		// Tags
		if let Some(tags) = tags.clone() {
			value.set_tags(tags, context);
		}

		// Set name
		if let Some(expression_name) = value.name_mut() {
			*expression_name = name.clone();
		}

		// Add the name declaration to the scope
		context.scope_data.declare_new_variable(name.clone(), value).map_err(mapped_err! {
			while = format!("attempting to add the variable \"{}\" to its scope", name.unmangled_name().bold().cyan()),
			context = context,
		})?;

		// Return the declaration
		Ok(Declaration {
			name,
			scope_id: context.scope_data.unique_id(),
			initial_value,
		})
	}
}

impl CompileTime for Declaration {
	type Output = Declaration;

	fn evaluate_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self::Output> {
		let evaluated = self.value(context).clone().evaluate_at_compile_time(context)?;
		context.scope_data.reassign_variable_from_id(&self.name, evaluated, self.scope_id)?;

		// Return the declaration
		Ok(Declaration {
			name: self.name,
			scope_id: self.scope_id,
			initial_value: self.initial_value,
		})
	}
}

impl TranspileToC for Declaration {
	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		Ok(format!(
			"void* {} = {};",
			self.name.to_c(context)?,
			self.value(context).clone().to_c(context).map_err(mapped_err! {
				while = format!("transpiling the value of the initial declaration for the variable \"{}\" to C", self.name.unmangled_name()),
				context = context,
			})?
		))
	}
}
