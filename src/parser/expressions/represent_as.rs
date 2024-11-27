use crate::{
	api::{
		context::context,
		scope::{ScopeId, ScopeType},
		traits::TryAs,
	},
	comptime::{memory::VirtualPointer, CompileTime},
	if_then_else_default, if_then_some,
	lexer::{Span, TokenType},
	mapped_err, parse_list,
	parser::{
		expressions::{
			name::Name,
			object::{Field, Fields as _},
			parameter::Parameter,
			Expression, Spanned,
		},
		statements::tag::TagList,
		ListType, Parse, TokenQueue, TokenQueueFunctionality as _,
	},
};

use super::Typed;

#[derive(Debug, Clone)]
pub struct RepresentAs {
	type_to_represent: Expression,
	type_to_represent_as: Expression,
	fields: Vec<Field>,
	name: Name,
	span: Span,
	compile_time_parameters: Vec<Parameter>,
	inner_scope_id: ScopeId,
	is_default: bool,
}

impl Parse for RepresentAs {
	type Output = RepresentAs;

	fn parse(tokens: &mut TokenQueue) -> anyhow::Result<Self::Output> {
		// Default
		let (is_default, start) = if tokens.next_is(TokenType::KeywordDefault) {
			(true, tokens.pop(TokenType::KeywordDefault)?.span)
		} else {
			(false, tokens.pop(TokenType::KeywordRepresent)?.span)
		};

		context().scope_data.enter_new_unlabeled_scope(ScopeType::RepresentAs);
		let inner_scope_id = context().scope_data.unique_id();

		let compile_time_parameters = if_then_else_default!(tokens.next_is(TokenType::LeftAngleBracket), {
			let mut parameters = Vec::new();
			parse_list!(tokens, ListType::AngleBracketed, {
				let parameter = Parameter::parse(tokens)?;
				context().scope_data.declare_new_variable(parameter.name().to_owned(), parameter.parameter_type().clone())?;
				parameters.push(parameter);
			});
			parameters
		});

		let type_to_represent = Expression::parse(tokens)?;
		tokens.pop(TokenType::KeywordAs)?;
		let type_to_represent_as = Expression::parse(tokens)?;

		let mut fields = Vec::new();
		let end = parse_list!(tokens, ListType::Braced, {
			// Parse tags
			let tags = if_then_some!(tokens.next_is(TokenType::TagOpening), TagList::parse(tokens)?);

			// Name
			let name = Name::parse(tokens).map_err(mapped_err! {
				while = "attempting to parse an object constructor",
			})?;

			// Value
			tokens.pop(TokenType::Equal)?;
			let mut value = Expression::parse(tokens)?;

			// Set tags
			if let Some(tags) = tags.clone() {
				value.set_tags(tags);
			}

			// Add field
			fields.add_or_overwrite_field(Field {
				name,
				value: Some(value),
				field_type: None,
			});
		})
		.span;

		context().scope_data.exit_scope()?;

		Ok(RepresentAs {
			type_to_represent,
			type_to_represent_as,
			fields,
			span: start.to(&end),
			name: "anonymous_represent_as".into(),
			compile_time_parameters,
			inner_scope_id,
			is_default,
		})
	}
}

impl CompileTime for RepresentAs {
	type Output = RepresentAs;

	fn evaluate_at_compile_time(self) -> anyhow::Result<Self::Output> {
		let previous = context().scope_data.set_current_scope(self.inner_scope_id);

		let type_to_represent = self.type_to_represent.evaluate_at_compile_time().map_err(mapped_err! {
			while = "evaluating the type to represent in a represent-as declaration at compile-time",
		})?;
		let type_to_represent_as = self.type_to_represent_as.evaluate_at_compile_time().map_err(mapped_err! {
			while = "evaluating the type to represent as in a represent-as declaration at compile-time",
		})?;

		let mut fields = Vec::new();

		for field in self.fields {
			let field_value = field.value.unwrap().evaluate_at_compile_time().map_err(mapped_err! {
				while = format!(
					"evaluating the value of the field \"{}\" of a represent-as declaration at compile-time",
					field.name.unmangled_name().bold().cyan()
				),
			})?;

			fields.add_or_overwrite_field(Field {
				name: field.name,
				value: Some(field_value),
				field_type: None,
			});
		}

		// Evaluate compile-time parameters
		let compile_time_parameters = self
			.compile_time_parameters
			.into_iter()
			.map(|parameter| parameter.evaluate_at_compile_time())
			.collect::<anyhow::Result<Vec<_>>>()
			.map_err(mapped_err! {
				while = "evaluating the compile-time parameters of a represent-as declaration at compile-time",
			})?;

		// Exit the scope
		context().scope_data.set_current_scope(previous);

		Ok(RepresentAs {
			type_to_represent,
			type_to_represent_as,
			name: self.name,
			span: self.span,
			fields,
			inner_scope_id: self.inner_scope_id,
			compile_time_parameters,
			is_default: self.is_default,
		})
	}
}

impl RepresentAs {
	pub fn type_to_represent(&self) -> &Expression {
		&self.type_to_represent
	}

	pub fn type_to_represent_as(&self) -> &Expression {
		&self.type_to_represent_as
	}

	pub fn fields(&self) -> &[Field] {
		&self.fields
	}

	pub fn can_represent(&self, object: &Expression) -> anyhow::Result<bool> {
		let previous = context().scope_data.set_current_scope(self.inner_scope_id);

		if let Expression::Name(name) = &self.type_to_represent {
			if let Expression::Parameter(parameter) = context().scope_data.get_variable(name).unwrap() {
				let anything = *context().scope_data.get_variable("Anything").unwrap().expect_as::<VirtualPointer>()?;
				let parameter_type = parameter.clone().get_type()?;
				if parameter_type == anything || object.is_assignable_to_type(parameter_type)? {
					return Ok(true);
				}
			}
		}

		context().scope_data.set_current_scope(previous);

		Ok(false)
	}

	pub fn can_represent_string(&self) -> anyhow::Result<String> {
		let previous = context().scope_data.set_current_scope(self.inner_scope_id);

		if let Expression::Name(name) = &self.type_to_represent {
			if let Expression::Parameter(parameter) = context().scope_data.get_variable(name).unwrap() {
				let parameter_type = parameter.clone().get_type()?;
				return Ok(parameter_type.virtual_deref().name().unmangled_name());
			}
		}

		context().scope_data.set_current_scope(previous);

		Ok("unknown".to_string())
	}
}

impl Spanned for RepresentAs {
	fn span(&self) -> Span {
		self.span.clone()
	}
}
