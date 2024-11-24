use std::collections::HashMap;

use crate::{
	api::{
		builtin::transpile_builtin_to_c,
		context::Context,
		scope::{ScopeId, ScopeType},
		traits::{TryAs as _, TupleOption},
	},
	bail_err,
	comptime::{memory::VirtualPointer, CompileTime},
	if_then_else_default, if_then_some,
	lexer::{Span, TokenType},
	mapped_err, parse_list,
	parser::{
		expressions::{
			block::Block,
			literal::{LiteralConvertible, LiteralObject},
			name::Name,
			object::{InternalFieldValue, ObjectType},
			Expression, Parse, Spanned,
		},
		statements::tag::TagList,
		ListType, TokenQueue, TokenQueueFunctionality as _,
	},
	transpiler::TranspileToC,
};

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
	tags: TagList,
	compile_time_parameters: Vec<(Name, Expression)>,
	parameters: Vec<(Name, Expression)>,
	return_type: Option<Expression>,
	body: Option<Expression>,
	outer_scope_id: ScopeId,
	inner_scope_id: Option<ScopeId>,
	this_object: Option<Expression>,
	name: Name,
	span: Span,
}

impl Parse for FunctionDeclaration {
	type Output = VirtualPointer;

	fn parse(tokens: &mut TokenQueue, context: &mut Context) -> anyhow::Result<Self::Output> {
		// "function" keyword
		let start = tokens.pop(TokenType::KeywordAction)?.span;
		let mut end = start.clone();

		// Compile-time parameters
		let compile_time_parameters = if_then_else_default!(tokens.next_is(TokenType::LeftAngleBracket), {
			let mut compile_time_parameters = Vec::new();
			end = parse_list!(tokens, ListType::AngleBracketed, {
				let name = Name::parse(tokens, context)?;
				tokens.pop(TokenType::Colon)?;
				let parameter_type = Expression::parse(tokens, context)?;
				compile_time_parameters.push((name, parameter_type));
			})
			.span;
			compile_time_parameters
		});

		// Parameters
		let parameters = if_then_else_default!(tokens.next_is(TokenType::LeftParenthesis), {
			let mut parameters = Vec::new();
			end = parse_list!(tokens, ListType::Parenthesized, {
				let name = Name::parse(tokens, context)?;
				tokens.pop(TokenType::Colon)?;
				let parameter_type = Expression::parse(tokens, context)?;
				parameters.push((name, parameter_type));
			})
			.span;
			parameters
		});

		// Return Type
		let return_type = if_then_some!(tokens.next_is(TokenType::Colon), {
			tokens.pop(TokenType::Colon)?;
			let expression = Expression::parse(tokens, context)?;
			end = expression.span(context);
			expression
		});

		// Body
		let (body, inner_scope_id) = if_then_some!(tokens.next_is(TokenType::LeftBrace), {
			let block = Block::parse_type(tokens, context, ScopeType::Function)?;
			let inner_scope_id = block.inner_scope_id;
			for (parameter_name, _parameter_type) in &compile_time_parameters {
				context
					.scope_data
					.declare_new_variable_from_id(parameter_name.clone(), Expression::Void(()), block.inner_scope_id)?;
			}
			end = block.span(context);
			(Expression::Block(block), inner_scope_id)
		})
		.deconstruct();

		// Return
		Ok(Self {
			tags: TagList::default(),
			parameters,
			compile_time_parameters,
			return_type,
			body,
			outer_scope_id: context.scope_data.unique_id(),
			inner_scope_id,
			this_object: None,
			name: Name::non_mangled("anonymous_function"),
			span: start.to(&end),
		}
		.to_literal()
		.store_in_memory(context))
	}
}

impl CompileTime for FunctionDeclaration {
	type Output = FunctionDeclaration;

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
			let mut parameters = Vec::new();
			for (parameter_name, parameter_type) in self.parameters {
				let parameter_type = parameter_type.evaluate_at_compile_time(context).map_err(mapped_err! {
					while = format!("evaluating the type of the parameter \"{}\" of a function at compile-time", parameter_name.unmangled_name().bold().cyan()),
					context = context,
				})?;
				if !parameter_type.is_pointer() {
					bail_err!(
						base = "A value that's not fully known at compile-time was used as a function parameter type",
						while = format!("checking the type of the parameter \"{}\"", parameter_name.unmangled_name().bold().cyan()),
						context = context,
					);
				}
				parameters.push((parameter_name, parameter_type));
			}
			parameters
		};

		// Return type
		let return_type = self.return_type.map(|return_type| anyhow::Ok(return_type.evaluate_at_compile_time(context)?)).transpose()?;

		// Body
		let body = self.body.map(|body| body.evaluate_at_compile_time(context)).transpose().map_err(mapped_err! {
			while = "evaluating the body of a function declaration at compile-time",
			context = context,
		})?;

		let this_object = self.this_object.map(|this| this.evaluate_at_compile_time(context)).transpose()?;

		// Return
		let function = FunctionDeclaration {
			compile_time_parameters,
			parameters,
			body,
			return_type,
			tags: self.tags.evaluate_at_compile_time(context)?,
			this_object,
			name: self.name,
			span: self.span,
			outer_scope_id: self.outer_scope_id,
			inner_scope_id: self.inner_scope_id,
		};

		// Return as a pointer
		Ok(function)
	}
}

impl TranspileToC for FunctionDeclaration {
	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		if !self.compile_time_parameters.is_empty() {
			return Ok(String::new());
		}

		// Get builtin and side effect tags
		let mut builtin_body = None;
		for tag in &self.tags.values {
			if let Ok(object) = tag.try_as_literal_or_name(context).cloned() {
				if object.type_name() == &Name::from("BuiltinTag") {
					let builtin_name = object.get_field_literal("internal_name", context).unwrap().expect_as::<String>()?.to_owned();
					let mut parameters = self.parameters.iter().map(|(parameter_name, _)| parameter_name.to_c(context).unwrap()).collect::<Vec<_>>();
					parameters.push("return_address".to_string());
					builtin_body = Some(transpile_builtin_to_c(&builtin_name, context, &parameters).map_err(mapped_err! {
						while = format!("transpiling the body of the built-in function {}()", builtin_name.bold().blue()),
						context = context,
					})?);
				}
			}
		}

		// Generate the C code for the return address declaration
		let return_type_c = if let Some(return_type) = self.return_type.as_ref() {
			format!(
				"{}{}* return_address",
				if self.parameters.is_empty() { "" } else { ", " },
				return_type.try_as_literal_or_name(context)?.clone().to_c_type(context)?
			)
		} else {
			String::new()
		};

		Ok(format!(
			"({}{}) {{\n{}\n}}",
			self.parameters
				.iter()
				.map(|(name, parameter_type)| Ok(format!(
					"{}* {}",
					parameter_type.try_as_literal_or_name(context)?.clone().to_c_type(context)?,
					name.to_c(context)?
				)))
				.collect::<anyhow::Result<Vec<_>>>()?
				.join(", "),
			return_type_c,
			if let Some(builtin_body) = builtin_body {
				builtin_body
			} else {
				let body = self.body.as_ref().unwrap().to_c(context)?;
				let body = body.strip_prefix("({").unwrap().strip_suffix("})").unwrap().to_owned();
				body
			}
			.lines()
			.map(|line| format!("\t{line}"))
			.collect::<Vec<_>>()
			.join("\n")
		))
	}
}

impl LiteralConvertible for FunctionDeclaration {
	fn to_literal(self) -> LiteralObject {
		LiteralObject {
			address: None,
			fields: HashMap::from([]),
			internal_fields: HashMap::from([
				("compile_time_parameters".to_owned(), InternalFieldValue::ParameterList(self.compile_time_parameters)),
				("parameters".to_owned(), InternalFieldValue::ParameterList(self.parameters)),
				("body".to_owned(), InternalFieldValue::OptionalExpression(self.body)),
				("return_type".to_owned(), InternalFieldValue::OptionalExpression(self.return_type)),
				("this_object".to_owned(), InternalFieldValue::OptionalExpression(self.this_object)),
			]),
			name: self.name,
			object_type: ObjectType::Function,
			outer_scope_id: self.outer_scope_id,
			inner_scope_id: self.inner_scope_id,
			span: self.span,
			type_name: "Function".into(),
			tags: self.tags,
		}
	}

	fn from_literal(literal: &LiteralObject) -> anyhow::Result<Self> {
		Ok(FunctionDeclaration {
			compile_time_parameters: literal.get_internal_field::<Vec<(Name, Expression)>>("compile_time_parameters")?.to_owned(),
			parameters: literal.get_internal_field::<Vec<(Name, Expression)>>("parameters")?.to_owned(),
			body: literal.get_internal_field::<Option<Expression>>("body")?.to_owned(),
			return_type: literal.get_internal_field::<Option<Expression>>("return_type")?.to_owned(),
			this_object: literal.get_internal_field::<Option<Expression>>("this_object")?.to_owned(),
			tags: literal.tags.clone(),
			outer_scope_id: literal.outer_scope_id(),
			inner_scope_id: literal.inner_scope_id,
			name: literal.name.clone(),
			span: literal.span.to_owned(),
		})
	}
}

impl Spanned for FunctionDeclaration {
	fn span(&self, _context: &Context) -> Span {
		self.span.clone()
	}
}

impl FunctionDeclaration {
	pub fn body(&self) -> Option<&Expression> {
		self.body.as_ref()
	}

	pub fn return_type(&self) -> Option<&Expression> {
		self.return_type.as_ref()
	}

	pub fn parameters(&self) -> &[(Name, Expression)] {
		&self.parameters
	}

	pub fn tags(&self) -> &TagList {
		&self.tags
	}

	pub fn name(&self) -> &Name {
		&self.name
	}

	pub fn this_object(&self) -> Option<&Expression> {
		self.this_object.as_ref()
	}

	pub fn set_this_object(&mut self, this_object: Expression) {
		self.this_object = Some(this_object);
	}

	pub fn compile_time_parameters(&self) -> &[(Name, Expression)] {
		&self.compile_time_parameters
	}
}
