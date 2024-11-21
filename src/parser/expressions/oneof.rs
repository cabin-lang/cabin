use std::collections::HashMap;

use crate::{
	api::{context::Context, macros::string, scope::ScopeType, traits::TryAs as _},
	comptime::CompileTime,
	lexer::TokenType,
	literal_list, parse_list,
	parser::{
		expressions::{
			literal::{LiteralConvertible, LiteralObject},
			name::Name,
			object::{Field, ObjectConstructor, ObjectType},
			Expression,
		},
		ListType, Parse, TokenQueue, TokenQueueFunctionality,
	},
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
				context.scope_data.declare_new_variable(name.clone(), Expression::Void(()))?;
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
						self.compile_time_parameters.iter().map(|name| string(&name.unmangled_name(), context)).collect()
					)),
					field_type: None,
				},
			],
			name: None,
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
			.expect_field_literal_as::<Vec<Expression>>("compile_time_parameters", context)
			.iter()
			.map(|name_string| anyhow::Ok(Name::from(name_string.expect_literal(context)?.expect_as::<String>())))
			.collect::<anyhow::Result<Vec<_>>>()?;

		let choices = literal
			.expect_field_literal_as::<Vec<Expression>>("variants", context)
			.iter()
			.map(|choice| choice.try_clone_pointer(context).unwrap())
			.collect();

		Ok(OneOf {
			compile_time_parameters,
			choices,
			scope_id: literal.declared_scope_id(),
		})
	}
}
