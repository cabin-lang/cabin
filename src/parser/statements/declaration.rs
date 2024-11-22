use crate::{
	api::{context::Context, traits::TryAs},
	comptime::{memory::VirtualPointer, CompileTime},
	lexer::TokenType,
	mapped_err,
	parser::{
		expressions::{group::GroupDeclaration, literal::LiteralConvertible, name::Name, Expression},
		statements::tag::TagList,
		Parse, TokenQueue, TokenQueueFunctionality as _,
	},
	transpiler::TranspileToC,
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
		context.scope_label = Some(name.clone());

		// Value
		tokens.pop(TokenType::Equal)?;
		let mut value = Expression::parse(tokens, context)?;

		// Tags
		if let Some(expression_tags) = value.tags_mut() {
			if let Some(declaration_tags) = &tags {
				*expression_tags = declaration_tags.clone();
			}
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
		})
	}
}

impl CompileTime for Declaration {
	type Output = Declaration;

	fn evaluate_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self::Output> {
		let value = context.scope_data.get_variable_from_id(self.name.clone(), self.scope_id).unwrap().clone();

		// Groups need special handling. There was an issue where if you refer to a group inside itself, such as when
		// using it as a function return type or parameter type, the compiler would crash. This is because things like
		// function declarations are evaluating their parameter types and return types in their respective
		// evaluate_at_compile_time() methods, and when it tries to evaluate the name of the group it's inside, it can't
		// find it as a literal, because the group value doesn't actually get stored in memory as a literal until after
		// it's compile-time evaluated, and evaluating those functions is part of evaluating the group.
		//
		// The fix to this is below. First, we assign a temporary literal group value to the group's name. This means
		// that when something inside a group refers to that group, they'll get a pointer to the temporary group literal.
		// This way all of their checks that the group exists and is a literal will pass. Then, later down in this function,
		// after the group is done being evaluated, we overwrite the temporary group in memory with the actual evaluated
		// group.
		//
		// This variable represents the pointer to the stored temporary group. If this declaration doesn't refer to a group,
		// it's None.
		let pointer_to_temporary_group = if let Expression::Group(group) = &value {
			// Create a temporary group literal and store it in memory, saving it's address as a pointer.
			let pointer = GroupDeclaration {
				fields: Vec::new(),
				name: "temporary_group".into(),
				scope_id: group.scope_id,
			}
			.to_literal(context)?
			.clone()
			.store_in_memory(context);

			// Reassign the variable name to the temporary group.
			context
				.scope_data
				.reassign_variable_from_id(&self.name, Expression::Pointer(pointer), self.scope_id)
				.map_err(mapped_err! {
					while = format!(
						"attempting to reassign the variable \"{}\" to its evaluated value",
						self.name.unmangled_name().bold().cyan()
					),
					context = context,
				})?;

			Some(pointer)
		} else {
			None
		};

		// Evaluate the value of the declaration at compile-time as much as possible
		let mut evaluated = value.evaluate_at_compile_time(context).map_err(mapped_err! {
			while = format!(
				"evaluating value of the initial declaration for the variable \"{}\" at compile-time",
				self.name.unmangled_name().bold().cyan()
			),
			context = context,
		})?;

		// Rewrite the location of the temporary group in memory to the group that was just evaluated. This means that
		// any pointers that were made to the temporary group now correctly point to the actual group that was evaluated.
		if let Some(temporary_group_pointer) = pointer_to_temporary_group {
			let group_address = evaluated.expect_as::<VirtualPointer>()?.to_owned();
			context.virtual_memory.move_overwrite(group_address, temporary_group_pointer);
			evaluated = Expression::Pointer(temporary_group_pointer);
		}

		// Reassign the variable in it's scope to the new evaluated value
		context.scope_data.reassign_variable_from_id(&self.name, evaluated, self.scope_id).map_err(mapped_err! {
			while = format!(
				"attempting to reassign the variable \"{}\" to its evaluated value",
				self.name.unmangled_name().bold().cyan()
			),
			context = context,
		})?;

		// Return the declaration
		Ok(Declaration {
			name: self.name,
			scope_id: self.scope_id,
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
