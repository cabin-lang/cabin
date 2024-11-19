use std::collections::HashMap;

use crate::{
	comptime::CompileTime,
	context::Context,
	lexer::TokenType,
	literal_list, parse_list,
	parser::{
		expressions::{
			name::Name,
			object::{Field, LiteralConvertible, LiteralObject, ObjectConstructor, ObjectType},
			Expression,
		},
		scope::ScopeType,
		statements::tag::TagList,
		ListType, Parse, TokenQueue, TokenQueueFunctionality,
	},
	string_literal,
};

#[derive(Debug, Clone)]
pub struct OneOf {
	compile_time_parameters: Vec<Name>,
	choices: Vec<Expression>,
	scope_id: usize,
}

impl Parse for OneOf {
	type Output = OneOf;

	fn parse(tokens: &mut TokenQueue, context: &mut Context) -> anyhow::Result<Self::Output> {
		tokens.pop(TokenType::KeywordOneOf)?;
		context.scope_data.enter_new_unlabeled_scope(ScopeType::OneOf);

		// Compile-time parameters
		let compile_time_parameters = if tokens.next_is(TokenType::LeftAngleBracket) {
			let mut compile_time_parameters = Vec::new();
			parse_list!(tokens, ListType::AngleBracketed, {
				let name = Name::parse(tokens, context)?;
				context.scope_data.declare_new_variable(name.clone(), Expression::Void, TagList::default())?;
				compile_time_parameters.push(name);
			});
			compile_time_parameters
		} else {
			Vec::new()
		};

		// Choices
		let mut choices = Vec::new();
		parse_list!(tokens, ListType::Braced, {
			choices.push(Expression::parse(tokens, context)?);
		});

		context.scope_data.exit_scope()?;

		// Return
		Ok(OneOf {
			choices,
			compile_time_parameters,
			scope_id: context.scope_data.unique_id(),
		})
	}
}

impl CompileTime for OneOf {
	type Output = Expression;

	fn evaluate_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self::Output> {
		let mut choices = Vec::new();
		for choice in self.choices {
			if let Expression::Name(choice_name) = &choice {
				if self.compile_time_parameters.contains(choice_name) {
					choices.push(choice);
					continue;
				}
			}

			let choice_value = choice.evaluate_at_compile_time(context)?;
			choices.push(choice_value);
		}

		Ok(Expression::Pointer(
			OneOf {
				choices,
				scope_id: self.scope_id,
				compile_time_parameters: self.compile_time_parameters,
			}
			.to_literal(context)
			.unwrap()
			.store_in_memory(context),
		))
	}
}

impl LiteralConvertible for OneOf {
	fn to_literal(self, context: &mut Context) -> anyhow::Result<LiteralObject> {
		let constructor = ObjectConstructor {
			fields: vec![
				Field {
					name: "variants".into(),
					value: Some(literal_list!(context, self.scope_id, self.choices)),
					field_type: None,
				},
				Field {
					name: "compile_time_parameters".into(),
					value: Some(literal_list!(
						context,
						self.scope_id,
						self.compile_time_parameters.iter().map(|name| string_literal!(&name.unmangled_name(), context)).collect()
					)),
					field_type: None,
				},
			],
			scope_id: self.scope_id,
			internal_fields: HashMap::new(),
			type_name: "OneOf".into(),
			object_type: ObjectType::OneOf,
		};

		LiteralObject::try_from_object_constructor(constructor, context)
	}

	fn from_literal(literal: &LiteralObject, context: &Context) -> anyhow::Result<Self> {
		if literal.object_type() != &ObjectType::OneOf {
			anyhow::bail!("Attempted to convert a non-oneof object into a oneof");
		}

		let compile_time_parameters = literal
			.get_field_literal(&"compile_time_parameters".into(), context)
			.unwrap()
			.list_elements()
			.unwrap()
			.iter()
			.map(|name_string| Name::from(name_string.as_literal(context).unwrap().as_string().unwrap()))
			.collect();

		let choices = literal
			.get_field_literal(&"variants".into(), context)
			.unwrap()
			.list_elements()
			.unwrap()
			.iter()
			.map(|choice| choice.to_owned_literal().unwrap())
			.collect();

		Ok(OneOf {
			compile_time_parameters,
			choices,
			scope_id: literal.scope_id,
		})
	}
}
