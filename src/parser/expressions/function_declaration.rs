use std::{collections::HashMap, fmt::Debug};

use crate::{
	api::{
		builtin::transpile_builtin_to_c,
		context::context,
		scope::{ScopeId, ScopeType},
		traits::{TryAs as _, TupleOption},
	},
	cli::theme::Styled,
	comptime::{memory::VirtualPointer, CompileTime},
	debug_log, debug_start, if_then_else_default, if_then_some,
	lexer::{Span, TokenType},
	mapped_err, parse_list,
	parser::{
		expressions::{
			block::Block,
			literal::{LiteralConvertible, LiteralObject},
			name::Name,
			object::InternalFieldValue,
			Expression, Parse, Spanned,
		},
		statements::tag::TagList,
		ListType, TokenQueue, TokenQueueFunctionality as _,
	},
	transpiler::TranspileToC,
};

use super::{field_access::FieldAccessType, parameter::Parameter};

#[derive(Clone)]
pub struct FunctionDeclaration {
	tags: TagList,
	compile_time_parameters: Vec<Parameter>,
	parameters: Vec<Parameter>,
	return_type: Option<Expression>,
	body: Option<Expression>,
	outer_scope_id: ScopeId,
	inner_scope_id: Option<ScopeId>,
	this_object: Option<Expression>,
	name: Name,
	span: Span,
}

impl Debug for FunctionDeclaration {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}{}{}{}{}",
			if_then_else_default!(!self.tags().is_empty(), format!("{:?} ", self.tags())),
			"action".style(context().theme.keyword()),
			if_then_else_default!(!self.compile_time_parameters().is_empty(), {
				format!(
					"<{}>",
					self.compile_time_parameters()
						.iter()
						.map(|parameter| format!("{parameter:?}"))
						.collect::<Vec<_>>()
						.join(", ")
				)
			}),
			if_then_else_default!(!self.parameters().is_empty(), {
				format!("({})", self.parameters().iter().map(|parameter| format!("{parameter:?}")).collect::<Vec<_>>().join(", "))
			}),
			if let Some(return_type) = self.return_type() {
				format!(": {return_type:?}")
			} else {
				String::new()
			}
		)
	}
}

impl Parse for FunctionDeclaration {
	type Output = VirtualPointer;

	fn parse(tokens: &mut TokenQueue) -> anyhow::Result<Self::Output> {
		let debug_section = debug_start!("{} a {}", "Parsing".bold().green(), "function declaration".cyan());
		// "function" keyword
		let start = tokens.pop(TokenType::KeywordAction)?.span;
		let mut end = start;

		// Compile-time parameters
		debug_log!("Parsing the compile-time parameters of {}", "function declaration".cyan());
		let compile_time_parameters = if_then_else_default!(tokens.next_is(TokenType::LeftAngleBracket), {
			let mut compile_time_parameters = Vec::new();
			end = parse_list!(tokens, ListType::AngleBracketed, {
				let parameter = Parameter::from_literal(Parameter::parse(tokens)?.virtual_deref()).unwrap();
				debug_log!(
					"Parsed compile-time parameter {} of type {:?} in a function declaration",
					parameter.name().unmangled_name().red(),
					parameter.parameter_type()
				);
				compile_time_parameters.push(parameter);
			})
			.span;
			compile_time_parameters
		});

		// Parameters
		debug_log!("Parsing the parameters of {}", "function declaration".cyan());
		let parameters = if_then_else_default!(tokens.next_is(TokenType::LeftParenthesis), {
			let mut parameters = Vec::new();
			end = parse_list!(tokens, ListType::Parenthesized, {
				let parameter = Parameter::from_literal(Parameter::parse(tokens)?.virtual_deref()).unwrap();
				debug_log!(
					"Parsed parameter {} of type {:?} in a function declaration",
					parameter.name().unmangled_name().red(),
					parameter.parameter_type()
				);
				parameters.push(parameter);
			})
			.span;
			parameters
		});

		// Return Type
		let return_type = if_then_some!(tokens.next_is(TokenType::Colon), {
			tokens.pop(TokenType::Colon)?;
			let expression = Expression::parse(tokens)?;
			end = expression.span();
			expression
		});

		// Body
		debug_log!("Parsing the body of a {}", "function declaration".cyan());
		let (body, inner_scope_id) = if_then_some!(tokens.next_is(TokenType::LeftBrace), {
			let block = Block::parse_type(tokens, ScopeType::Function)?;
			let inner_scope_id = block.inner_scope_id;
			for parameter in &compile_time_parameters {
				context()
					.scope_data
					.declare_new_variable_from_id(parameter.name().clone(), Expression::Void(()), block.inner_scope_id)?;
			}
			for parameter in &parameters {
				context()
					.scope_data
					.declare_new_variable_from_id(parameter.name().clone(), Expression::Void(()), block.inner_scope_id)?;
			}
			end = block.span();
			(Expression::Block(block), inner_scope_id)
		})
		.deconstruct();

		// Return
		debug_section.finish();
		Ok(Self {
			tags: TagList::default(),
			parameters,
			compile_time_parameters,
			return_type,
			body,
			outer_scope_id: context().scope_data.unique_id(),
			inner_scope_id,
			this_object: None,
			name: Name::non_mangled("anonymous_function"),
			span: start.to(&end),
		}
		.to_literal()
		.store_in_memory())
	}
}

impl CompileTime for FunctionDeclaration {
	type Output = FunctionDeclaration;

	fn evaluate_at_compile_time(self) -> anyhow::Result<Self::Output> {
		let debug_section = debug_start!(
			"{} a {} called {}",
			"Compile-Time Evaluating".bold().green(),
			"function declaration".cyan(),
			self.name.unmangled_name().blue()
		);

		// Compile-time parameters
		let compile_time_parameters = {
			let mut compile_time_parameters = Vec::new();
			for parameter in self.compile_time_parameters {
				compile_time_parameters.push(parameter.evaluate_at_compile_time()?);
			}
			compile_time_parameters
		};

		// Parameters
		let parameters = {
			let debug_section = debug_start!("{} the parameters of a {}", "Compile-Time Evaluating".green().bold(), "function declaration".cyan());
			let mut parameters = Vec::new();
			for parameter in self.parameters {
				let debug_section = debug_start!(
					"{} a parameter called {} of type {:?} on a {}",
					"Compile-Time Evaluating".bold().green(),
					parameter.name().unmangled_name().red(),
					parameter.parameter_type(),
					"function declaration".cyan()
				);
				parameters.push(parameter.evaluate_at_compile_time().map_err(mapped_err! {
					while = "evaluating a parameter at compile-time",
				})?);
				debug_section.finish();
			}
			debug_section.finish();
			parameters
		};

		// Return type
		debug_log!("Compile-Time Evaluating the return type of a {}", "function declaration".cyan());
		let return_type = self.return_type.map(|return_type| return_type.evaluate_as_type()).transpose()?;

		let tags = {
			let debug_section = debug_start!("Evaluating the tags on a {}", "function declaration".cyan());
			let evaluated = self.tags.evaluate_at_compile_time()?;
			debug_section.finish();
			evaluated
		};

		// Return
		debug_section.finish();
		let function = FunctionDeclaration {
			compile_time_parameters,
			parameters,
			body: self.body,
			return_type,
			tags,
			this_object: self.this_object,
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
	fn to_c(&self) -> anyhow::Result<String> {
		if !self.compile_time_parameters.is_empty() {
			return Ok(String::new());
		}

		// Get builtin and side effect tags
		let mut builtin_body = None;
		for tag in &self.tags.values {
			if let Ok(object) = tag.try_as_literal() {
				if object.type_name() == &Name::from("BuiltinTag") {
					let builtin_name = object.get_field_literal("internal_name").unwrap().expect_as::<String>()?.to_owned();
					let mut parameters = self.parameters.iter().map(|parameter| parameter.name().to_c().unwrap()).collect::<Vec<_>>();
					parameters.push("return_address".to_string());
					builtin_body = Some(transpile_builtin_to_c(&builtin_name, &parameters).map_err(mapped_err! {
						while = format!("transpiling the body of the built-in function {}()", builtin_name.bold().blue()),
					})?);
				}
			}
		}

		// Generate the C code for the return address declaration
		let return_type_c = if let Some(return_type) = self.return_type.as_ref() {
			format!(
				"{}{}* return_address",
				if self.parameters.is_empty() { "" } else { ", " },
				return_type.try_as_literal()?.to_c_type()?
			)
		} else {
			String::new()
		};

		Ok(format!(
			"({}{}) {{\n{}\n}}",
			self.parameters
				.iter()
				.map(|parameter| Ok(format!("{}* {}", parameter.parameter_type().try_as_literal()?.to_c_type()?, parameter.name().to_c()?)))
				.collect::<anyhow::Result<Vec<_>>>()?
				.join(", "),
			return_type_c,
			if let Some(builtin_body) = builtin_body {
				builtin_body
			} else {
				let body = self.body.as_ref().unwrap().to_c()?;
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
			field_access_type: FieldAccessType::Normal,
			outer_scope_id: self.outer_scope_id,
			inner_scope_id: self.inner_scope_id,
			span: self.span,
			type_name: "Function".into(),
			tags: self.tags,
		}
	}

	fn from_literal(literal: &LiteralObject) -> anyhow::Result<Self> {
		Ok(FunctionDeclaration {
			compile_time_parameters: literal.get_internal_field::<Vec<Parameter>>("compile_time_parameters")?.to_owned(),
			parameters: literal.get_internal_field::<Vec<Parameter>>("parameters")?.to_owned(),
			body: literal.get_internal_field::<Option<Expression>>("body")?.to_owned(),
			return_type: literal.get_internal_field::<Option<Expression>>("return_type")?.to_owned(),
			this_object: literal.get_internal_field::<Option<Expression>>("this_object")?.to_owned(),
			tags: literal.tags.clone(),
			outer_scope_id: literal.outer_scope_id(),
			inner_scope_id: literal.inner_scope_id,
			name: literal.name.clone(),
			span: literal.span,
		})
	}
}

impl Spanned for FunctionDeclaration {
	fn span(&self) -> Span {
		self.span
	}
}

impl FunctionDeclaration {
	pub fn body(&self) -> Option<&Expression> {
		self.body.as_ref()
	}

	pub fn return_type(&self) -> Option<&Expression> {
		self.return_type.as_ref()
	}

	pub fn parameters(&self) -> &[Parameter] {
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

	pub fn compile_time_parameters(&self) -> &[Parameter] {
		&self.compile_time_parameters
	}

	pub fn set_name(&mut self, name: Name) {
		self.name = name;
	}
}
