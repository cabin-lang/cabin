use std::{
	collections::HashMap,
	sync::atomic::{AtomicUsize, Ordering},
};

use colored::Colorize as _;

use crate::{
	api::{builtin::transpile_builtin_to_c, context::Context, macros::string, traits::TryAs as _},
	comptime::{memory::Pointer, CompileTime},
	lexer::TokenType,
	literal, literal_list, mapped_err, parse_list,
	parser::{
		expressions::{
			block::Block,
			literal::{LiteralConvertible, LiteralObject},
			name::{Name, NameOption},
			object::{Field, InternalFieldValue, ObjectConstructor, ObjectType},
			Expression, Parse,
		},
		statements::tag::TagList,
		ListType, TokenQueue, TokenQueueFunctionality as _,
	},
	transpiler::TranspileToC,
};

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
	pub return_type: Option<Box<Expression>>,
	pub compile_time_parameters: Vec<(Name, Expression)>,
	pub parameters: Vec<(Name, Expression)>,
	pub body: Option<Box<Expression>>,
	pub scope_id: usize,
	pub tags: TagList,
	pub this_object: Option<Box<Expression>>,
	pub name: Option<Name>,
}

static ANONYMOUS_FUNCTION_COUNT: AtomicUsize = AtomicUsize::new(0);

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
					.declare_new_variable_from_id(parameter_name.clone(), Expression::Void(()), block.inner_scope_id)?;
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
			this_object: None,
			name: Some(Name::non_mangled(format!(
				"anonymous_function_{}",
				ANONYMOUS_FUNCTION_COUNT.fetch_add(1, Ordering::Relaxed)
			))),
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
				if !parameter_type.is_pointer() {
					anyhow::bail!("A value that's not fully known at compile-time was used as a function parameter type");
				}
				compile_time_parameters.push((parameter_name, parameter_type));
			}
			compile_time_parameters
		};

		// Parameters
		let parameters = {
			let mut compile_time_parameters = Vec::new();
			for (parameter_name, parameter_type) in self.parameters {
				let parameter_type = parameter_type.evaluate_at_compile_time(context)?;
				if !parameter_type.is_pointer() {
					anyhow::bail!(
						"A value that's not fully known at compile-time was used as a function parameter type\n\t{}",
						format!("while checking the type of the parameter \"{}\"", parameter_name.unmangled_name().bold().cyan()).dimmed()
					);
				}
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
			.map_err(mapped_err! {
				while = "evaluating the body of a function declaration at compile-time",
				context = context,
			})?;

		let this_object = self.this_object.map(|this| anyhow::Ok(Box::new(this.evaluate_at_compile_time(context)?))).transpose()?;

		// Return
		let function = FunctionDeclaration {
			compile_time_parameters,
			parameters,
			body,
			return_type,
			scope_id: self.scope_id,
			tags: self.tags.evaluate_at_compile_time(context)?,
			this_object,
			name: self.name,
		};

		// Return as a pointer
		Ok(Expression::Pointer(
			function
				.to_literal(context)
				.map_err(mapped_err! {
					while = "converting a function declaration into an object at compile-time",
					context = context,
				})?
				.store_in_memory(context),
		))
	}
}

impl TranspileToC for FunctionDeclaration {
	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		let mut body = None;

		// Get builtin and side effect tags
		for tag in &self.tags.values {
			if let Ok(object) = tag.try_as_literal(context) {
				if object.type_name == Name::from("BuiltinTag") {
					let builtin_name = object.get_field_literal("internal_name", context).unwrap().expect_as::<String>().to_owned();
					let mut parameters = self.parameters.iter().map(|(parameter_name, _)| parameter_name.to_c(context).unwrap()).collect::<Vec<_>>();
					parameters.push("return_address".to_string());
					body = Some(transpile_builtin_to_c(&builtin_name, &parameters)?);
				}
			}
		}

		Ok(format!(
			"({}) {{\n{}\n}}",
			self.parameters
				.iter()
				.map(|(name, parameter_type)| Ok(format!("{}* {}, ", parameter_type.to_c(context)?, name.to_c(context)?)))
				.collect::<anyhow::Result<String>>()?
				+ "void* return_address",
			if let Some(body) = body { body } else { self.body.as_ref().unwrap().to_c(context)? }
				.lines()
				.map(|line| format!("\t{line}"))
				.collect::<Vec<_>>()
				.join("\n")
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
					name = self.name.with_field(&parameter_name),
					context = context,
					Parameter {
						name = string(&parameter_name.unmangled_name(), context),
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
					name = self.name.with_field(&parameter_name),
					context = context,
					Parameter {
						name = string(&parameter_name.unmangled_name(), context),
						type = parameter_type
					},
					self.scope_id
				}
			})
			.collect();

		// Create the object
		let constructor = ObjectConstructor {
			name: self.name,
			fields: vec![
				Field {
					name: "return_type".into(),
					value: Some(match self.return_type {
						Some(return_type) => *return_type,
						None => context.nothing(),
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
				Field {
					name: "this_object".into(),
					value: Some(match self.this_object {
						Some(this_object) => *this_object,
						None => context.nothing(),
					}),
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
		let tags_field = literal.get_field("tags").unwrap();
		let tag_refs = tags_field.expect_literal(context)?.expect_as::<Vec<Expression>>();
		let tags = tag_refs
			.iter()
			.map(|element| {
				element.try_clone_pointer(context).map_err(mapped_err! {
					while = format!("attempting to interpret a {} tag expression as a literal", element.kind_name().bold().cyan()),
					context = context,
				})
			})
			.collect::<anyhow::Result<Vec<_>>>()
			.map_err(mapped_err! {
				while = "interpreting the function declaration's tags as literals",
				context = context,
			})?;

		// Compile-time parameters
		let compile_time_parameters = literal.get_field_literal("compile_time_parameters", context).unwrap().try_as::<Vec<Expression>>()?;
		let compile_time_parameters = compile_time_parameters
			.iter()
			.map(|element| {
				let parameter_object = element.expect_literal(context)?;
				let name = parameter_object.get_field_literal("name", context).unwrap().expect_as::<String>();
				anyhow::Ok((Name::from(name), parameter_object.get_field("type").unwrap()))
			})
			.collect::<anyhow::Result<Vec<_>>>()?;

		// Parameters
		let parameters = literal.get_field_literal("parameters", context).unwrap().try_as::<Vec<Expression>>()?;
		let parameters = parameters
			.iter()
			.map(|element| {
				let parameter_object = element.try_as_literal(context).unwrap();
				let name = parameter_object.get_field_literal("name", context).unwrap().expect_as::<String>();
				(Name::from(name), parameter_object.get_field("type").unwrap())
			})
			.collect();

		// Return type
		let return_type_optional = literal.get_field("return_type").unwrap().try_into().unwrap();
		let nothing = context.nothing().try_into().unwrap();
		let return_type = if return_type_optional == nothing {
			None
		} else {
			Some(Box::new(Expression::Pointer(return_type_optional)))
		};

		// Body
		let body = literal.get_internal_field::<Option<Expression>>("body").cloned().unwrap().map(Box::new);

		// This object
		let this_object_optional = literal.get_field("this_object").unwrap().try_into().unwrap();
		let this_object = if this_object_optional == nothing {
			None
		} else {
			Some(Box::new(Expression::Pointer(this_object_optional)))
		};

		// Return the value
		Ok(FunctionDeclaration {
			compile_time_parameters,
			parameters,
			body,
			return_type,
			scope_id: literal.declared_scope_id(),
			tags: tags.into(),
			this_object,
			name: literal.name.clone(),
		})
	}
}
