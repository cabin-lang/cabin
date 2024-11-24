use crate::{
	api::{context::Context, scope::ScopeId, traits::TryAsRefMut},
	comptime::{memory::VirtualPointer, CompileTime},
	err,
	lexer::TokenType,
	mapped_err,
	parser::{
		expressions::{name::Name, represent_as::RepresentAs, Expression},
		statements::tag::TagList,
		Parse, TokenQueue, TokenQueueFunctionality as _,
	},
	transpiler::TranspileToC,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclarationType {
	Normal,
	RepresentAs,
}

#[derive(Debug, Clone)]
pub struct Declaration {
	name: Name,
	scope_id: ScopeId,
	declaration_type: DeclarationType,
}

impl Declaration {
	pub fn value<'a>(&'a self, context: &'a Context) -> anyhow::Result<&'a Expression> {
		context.scope_data.get_variable_from_id(self.name.clone(), self.scope_id).ok_or_else(|| {
			err! {
				base = format!("Attempted to get the value for the declaration of \"{}\", but it has no value stored.", self.name.unmangled_name().bold().cyan()),
				context = context,
			}
		})
	}

	pub fn declaration_type(&self) -> &DeclarationType {
		&self.declaration_type
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

		// Represent-As declarations
		if tokens.next_is_one_of(&[TokenType::KeywordRepresent, TokenType::KeywordDefault]) {
			let represent_as = RepresentAs::parse(tokens, context).map_err(mapped_err! {
				while = "parsing a represent-as declaration",
				context = context,
			})?;
			context.scope_data.add_represent_as_declaration(name.clone(), represent_as);
			tokens.pop(TokenType::Semicolon)?;
			return Ok(Declaration {
				name,
				scope_id: context.scope_data.unique_id(),
				declaration_type: DeclarationType::RepresentAs,
			});
		}

		let mut value = Expression::parse(tokens, context)?;

		// Tags
		if let Some(tags) = tags.clone() {
			value.set_tags(tags, context);
		}

		// Set name
		if let Ok(literal) = value.try_as_ref_mut::<VirtualPointer>().map(|pointer| pointer.virtual_deref_mut(context)) {
			literal.name = name.clone();
		}

		// Add the name declaration to the scope
		context.scope_data.declare_new_variable(name.clone(), value).map_err(mapped_err! {
			while = format!("attempting to add the variable \"{}\" to its scope", name.unmangled_name().bold().cyan()),
			context = context,
		})?;

		tokens.pop(TokenType::Semicolon)?;

		// Return the declaration
		Ok(Declaration {
			name,
			scope_id: context.scope_data.unique_id(),
			declaration_type: DeclarationType::Normal,
		})
	}
}

impl CompileTime for Declaration {
	type Output = Declaration;

	fn evaluate_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self::Output> {
		// Represent as
		if self.declaration_type == DeclarationType::RepresentAs {
			let represent = context.scope_data.get_represent_from_id(self.name.clone(), self.scope_id).unwrap().clone();
			let evaluated = represent.evaluate_at_compile_time(context).map_err(mapped_err! {
				while = "evaluating a represent-as declaration at compile-time",
				context = context,
			})?;
			context.scope_data.reassign_represent_from_id(&self.name, evaluated, self.scope_id)?;
			return Ok(self);
		}

		let evaluated = self
			.value(context)
			.map_err(mapped_err! {
				while = format!("getting the value of the declaration of \"{}\"", self.name.unmangled_name().bold().cyan()),
				context = context,
			})?
			.clone()
			.evaluate_at_compile_time(context)?;
		context.scope_data.reassign_variable_from_id(&self.name, evaluated, self.scope_id)?;

		// Return the declaration
		Ok(self)
	}
}

impl TranspileToC for Declaration {
	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		Ok(format!(
			"void* {} = {};",
			self.name.to_c(context)?,
			self.value(context)?.clone().to_c(context).map_err(mapped_err! {
				while = format!("transpiling the value of the initial declaration for the variable \"{}\" to C", self.name.unmangled_name()),
				context = context,
			})?
		))
	}
}

impl Declaration {
	pub fn name(&self) -> &Name {
		&self.name
	}
}
