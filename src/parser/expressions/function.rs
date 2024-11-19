use std::collections::HashMap;

use colored::Colorize as _;

use crate::{
	comptime::CompileTime,
	context::Context,
	lexer::TokenType,
	literal, literal_list, parse_list,
	parser::{
		expressions::{
			block::Block,
			name::Name,
			object::{Field, InternalFieldValue, LiteralConvertible, LiteralObject, ObjectConstructor, ObjectType},
			Expression, Parse,
		},
		statements::tag::TagList,
		ListType, TokenQueue, TokenQueueFunctionality as _,
	},
	string_literal,
};

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
	pub return_type: Option<Box<Expression>>,
	pub compile_time_parameters: Vec<(Name, Expression)>,
	pub parameters: Vec<(Name, Expression)>,
	pub body: Option<Box<Expression>>,
	pub scope_id: usize,
	pub tags: TagList,
}

impl Parse for FunctionDeclaration {
	type Output = FunctionDeclaration;

	fn parse(tokens: &mut TokenQueue, context: &mut Context) -> anyhow::Result<Self::Output> {
		// "function" keyword
		tokens.pop(TokenType::KeywordAction)?;

		// Compile-time parameters
		let compile_time_parameters = if tokens.next_is(TokenType::LeftAngleBracket) {
			let mut compile_time_parameters = Vec::new();
			parse_list!(tokens, ListType::AngleBracketed, {
				let name = Name::parse(tokens, context)?;
				tokens.pop(TokenType::Colon)?;
				let parameter_type = Expression::parse(tokens, context)?;
				compile_time_parameters.push((name, parameter_type));
			});
			compile_time_parameters
		} else {
			Vec::new()
		};

		// Parameters
		let parameters = if tokens.next_is(TokenType::LeftParenthesis) {
			let mut parameters = Vec::new();
			parse_list!(tokens, ListType::Parenthesized, {
				let name = Name::parse(tokens, context)?;
				tokens.pop(TokenType::Colon)?;
				let parameter_type = Expression::parse(tokens, context)?;
				parameters.push((name, parameter_type));
			});
			parameters
		} else {
			Vec::new()
		};

		// Return Type
		let return_type = if tokens.next_is(TokenType::Colon) {
			tokens.pop(TokenType::Colon)?;
			Some(Box::new(Expression::parse(tokens, context)?))
		} else {
			None
		};

		// Body
		let body = if tokens.next_is(TokenType::LeftBrace) {
			let block = Block::parse(tokens, context)?;
			for (parameter_name, _parameter_type) in &compile_time_parameters {
				context
					.scope_data
					.declare_new_variable_from_id(parameter_name.clone(), Expression::Void, TagList::default(), block.inner_scope_id)?;
			}
			Some(Box::new(Expression::Block(block)))
		} else {
			None
		};

		// Return
		Ok(Self {
			tags: TagList::default(),
			parameters,
			compile_time_parameters,
			return_type,
			body,
			scope_id: context.scope_data.unique_id(),
		})
	}
}

impl CompileTime for FunctionDeclaration {
	type Output = Expression;

	fn evaluate_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self::Output> {
		// Compile-time parameters
		let compile_time_parameters = {
			let mut compile_time_parameters = Vec::new();
			for (parameter_name, parameter_type) in self.compile_time_parameters {
				let parameter_type = parameter_type.evaluate_at_compile_time(context)?;
				compile_time_parameters.push((parameter_name, parameter_type));
			}
			compile_time_parameters
		};

		// Parameters
		let parameters = {
			let mut compile_time_parameters = Vec::new();
			for (parameter_name, parameter_type) in self.parameters {
				let parameter_type = parameter_type.evaluate_at_compile_time(context)?;
				compile_time_parameters.push((parameter_name, parameter_type));
			}
			compile_time_parameters
		};

		// Return type
		let return_type = self
			.return_type
			.map(|return_type| anyhow::Ok(Box::new(return_type.evaluate_at_compile_time(context)?)))
			.transpose()?;

		// Body
		let body = self
			.body
			.map(|body| anyhow::Ok(Box::new(body.evaluate_at_compile_time(context)?)))
			.transpose()
			.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating the body of a function declaration at compile-time".dimmed()))?;

		// Return
		let function = FunctionDeclaration {
			compile_time_parameters,
			parameters,
			body,
			return_type,
			scope_id: self.scope_id,
			tags: self.tags.evaluate_at_compile_time(context)?,
		};

		Ok(Expression::Pointer(
			function
				.to_literal(context)
				.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while converting a function declaration into an object at compile-time".dimmed()))?
				.store_in_memory(context),
		))
	}
}

impl LiteralConvertible for FunctionDeclaration {
	fn to_literal(self, context: &mut Context) -> anyhow::Result<LiteralObject> {
		// Compile-time parameters
		let compile_time_parameters = self
			.compile_time_parameters
			.into_iter()
			.map(|(parameter_name, parameter_type)| {
				literal! {
					context,
					Parameter {
						name = string_literal!(&parameter_name.unmangled_name(), context),
						type = parameter_type
					},
					self.scope_id
				}
			})
			.collect();

		// Parameters
		let parameters = self
			.parameters
			.into_iter()
			.map(|(parameter_name, parameter_type)| {
				literal! {
					context,
					Parameter {
						name = string_literal!(&parameter_name.unmangled_name(), context),
						type = parameter_type
					},
					self.scope_id
				}
			})
			.collect();

		// Create the object
		let constructor = ObjectConstructor {
			fields: vec![
				Field {
					name: "return_type".into(),
					value: Some(match self.return_type {
						Some(return_type) => *return_type,
						None => context.scope_data.get_global_variable(&"nothing".into()).unwrap().value.to_owned_literal()?,
					}),
					field_type: None,
				},
				Field {
					name: "compile_time_parameters".into(),
					value: Some(literal_list!(context, self.scope_id, compile_time_parameters)),
					field_type: None,
				},
				Field {
					name: "parameters".into(),
					value: Some(literal_list!(context, self.scope_id, parameters)),
					field_type: None,
				},
				Field {
					name: "tags".into(),
					value: Some(literal_list!(context, self.scope_id, self.tags.values)),
					field_type: None,
				},
			],
			scope_id: self.scope_id,
			internal_fields: HashMap::from([("body".to_owned(), InternalFieldValue::OptionalExpression(self.body.map(|body| *body)))]),
			type_name: "Function".into(),
			object_type: ObjectType::Function,
		};

		// Convert to literal
		LiteralObject::try_from_object_constructor(constructor, context)
	}

	fn from_literal(literal: &LiteralObject, context: &Context) -> anyhow::Result<Self> {
		// Check if it's a function
		if literal.object_type() != &ObjectType::Function {
			anyhow::bail!("Attempted to convert a non-function literal into a function");
		}

		// Tags
		let tags = literal
			.get_field(&"tags".into())
			.unwrap()
			.as_literal(context)
			.unwrap()
			.list_elements()
			.unwrap()
			.iter()
			.map(|element| element.to_owned_literal().unwrap())
			.collect::<Vec<_>>();

		// Compile-time parameters
		let compile_time_parameters = literal.get_field_literal(&"compile_time_parameters".into(), context).unwrap().list_elements()?;
		let compile_time_parameters = compile_time_parameters
			.iter()
			.map(|element| {
				let parameter_object = element.as_literal(context).unwrap();
				let name = parameter_object.get_field_literal(&"name".into(), context).unwrap().as_string().unwrap();
				(Name::from(name), parameter_object.get_field(&"type".into()).unwrap())
			})
			.collect();

		// Parameters
		let parameters = literal.get_field_literal(&"parameters".into(), context).unwrap().list_elements()?;
		let parameters = parameters
			.iter()
			.map(|element| {
				let parameter_object = element.as_literal(context).unwrap();
				let name = parameter_object.get_field_literal(&"name".into(), context).unwrap().as_string().unwrap();
				(Name::from(name), parameter_object.get_field(&"type".into()).unwrap())
			})
			.collect();

		// Return type
		let return_type_optional = literal.get_field(&"return_type".into()).unwrap().as_literal_address().unwrap();
		let nothing = context.scope_data.get_global_variable(&"nothing".into()).unwrap().value.as_literal_address().unwrap();
		let return_type = if return_type_optional == nothing {
			None
		} else {
			Some(Box::new(Expression::Pointer(return_type_optional)))
		};

		// Body
		let body = literal.get_internal_field("body").unwrap().to_owned().as_optional_expression().unwrap().map(Box::new);

		// Return the value
		Ok(FunctionDeclaration {
			compile_time_parameters,
			parameters,
			body,
			return_type,
			scope_id: literal.scope_id,
			tags: tags.into(),
		})
	}
}
