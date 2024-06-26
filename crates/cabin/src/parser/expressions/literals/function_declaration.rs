use crate::{
	compile_time::{builtin::builtin_to_c, CompileTime, CompileTimeStatement, TranspileToC},
	context::Context,
	formatter::{ColoredCabin, ToCabin},
	lexer::{Token, TokenType},
	parse_list,
	parser::{
		expressions::{
			literals::{Literal, LiteralValue},
			run::ParentExpression,
			util::{name::Name, tags::TagList, types::Typed},
			Expression,
		},
		statements::Statement,
		Parse, TokenQueue,
	},
	scopes::ScopeType,
	util::IntegerSuffix,
	var_literal, void,
};

// Brings the `write!()` and `writeln!()` macros into scope, which allows appending to a string. This is more efficient than using
// `string = format!("{string}...")`, because it avoids an extra allocation. We have a clippy warning turned on for this very
// purpose. We assign this to `_` to indicate clearly that it's just a trait and not used explicitly anywhere outside of bringing its
// methods into scope.
use std::{fmt::Write as _, sync::atomic::AtomicUsize};

use colored::Colorize as _;

/// A function declaration
#[derive(Clone, Debug)]
pub struct FunctionDeclaration {
	/// The body of the function. This may be `None` in the case of a builtin function, which is called from Rust.
	pub body: Option<Vec<Statement>>,
	/// The parameters of the function. This is a `Vec` of tuples containing the name of the parameter and the type tag of the parameter.
	pub parameters: Vec<(Name, Expression)>,
	/// The return type of the function.
	pub return_type: Expression,
	/// The tags on the function.
	pub tags: TagList,
	/// The ID of the inner scope of the function. This is referenced later during compile_time to evaluate the function.
	pub inner_scope_id: Option<usize>,

	/// A name for the function. This does not actually do anything besides make the transpiled C code more readable. This also doesn't have to be unique,
	/// as transpilation will append this functions unique ID to the end of the function name. Other than that, this is just the name that the transpiled
	/// C function will have.
	pub name: Option<String>,

	/// The unique ID of the function.
	pub id: usize,

	/// Whether this function is a "non-void" function. This should be used to check if this function returns a non-void value, as opposed to checking its return type,
	/// because *all* functions in Cabin get converted into void functions, and the return value is passed as a pointer parameter.
	pub is_non_void: bool,

	/// Whether this function declaration has already been evaluated at compile-time. This prevents double compile-time evaluation, which causes performance overhead and
	/// can cause unexpected issues and bugs.
	has_been_compile_time_evaluated: bool,
}

/// The next unique ID for a function declaration.
static FUNCTION_ID: AtomicUsize = AtomicUsize::new(0);

impl Parse for FunctionDeclaration {
	type Output = Self;

	fn parse(tokens: &mut std::collections::VecDeque<Token>, context: &mut Context) -> anyhow::Result<Self::Output> {
		tokens.pop(TokenType::KeywordAction, context)?;

		// Parameters
		let mut parameters = Vec::new();
		if tokens.next_is(TokenType::LeftParenthesis) {
			tokens.pop(TokenType::LeftParenthesis, context)?;
			if !tokens.next_is(TokenType::RightParenthesis) {
				parse_list!(tokens, context, {
					let name = Name(tokens.pop(TokenType::Identifier, context)?);
					tokens
						.pop(TokenType::Colon, context)
						.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while attempting to parse the colon before a function parameter's type".dimmed()))?;
					let type_annotation = Expression::parse(tokens, context)?;
					parameters.push((name, type_annotation));
				});
			}
			tokens.pop(TokenType::RightParenthesis, context)?;
		}

		// Return type
		tokens
			.pop(TokenType::Colon, context)
			.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while attempting to parse the colon before a function's return type".dimmed()))?;
		context.is_parsing_type = true;
		let return_type = Expression::parse(tokens, context)?;
		context.is_parsing_type = false;

		// Body
		let (body, inner_scope_id) = if tokens.next_is(TokenType::LeftBrace) {
			context.scope_data.enter_new_scope(ScopeType::FunctionDeclaration);
			let inner_scope_id = context.scope_data.unique_id();

			tokens.pop(TokenType::LeftBrace, context).unwrap_or_else(|_| unreachable!());
			let mut body = Vec::new();
			while !tokens.next_is(TokenType::RightBrace) {
				let statement = Statement::parse(tokens, context)?;
				body.push(statement);
			}
			tokens.pop(TokenType::RightBrace, context)?;
			context.scope_data.exit_scope()?;

			(Some(body), Some(inner_scope_id))
		} else {
			(None, None)
		};

		Ok(Self {
			body,
			parameters,
			return_type,
			tags: TagList::default(),
			inner_scope_id,
			id: FUNCTION_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
			name: None,
			is_non_void: false,
			has_been_compile_time_evaluated: false,
		})
	}
}

impl Typed for FunctionDeclaration {
	fn get_type(&self, _context: &mut Context) -> anyhow::Result<Literal> {
		Ok(var_literal!("Function", 0))
	}
}

impl CompileTime for FunctionDeclaration {
	fn compile_time_evaluate(&self, context: &mut Context, with_side_effects: bool) -> anyhow::Result<Expression> {
		if self.has_been_compile_time_evaluated {
			return Ok(Expression::Literal(Literal::new(LiteralValue::FunctionDeclaration(Box::new(self.clone())))));
		}

		let parameters = self
			.parameters
			.iter()
			.map(|(name, type_annotation)| {
				let evaluated = type_annotation
					.compile_time_evaluate(context, with_side_effects)
					.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating a declared function's parameter types at compile-time".dimmed()))?;
				evaluated.require_literal(context)?;
				Ok((name.clone(), evaluated))
			})
			.collect::<anyhow::Result<Vec<_>>>()?;

		context.parameter_names = self
			.parameters
			.iter()
			.map(|parameter| (parameter.0.clone(), parameter.1.as_literal(context).unwrap().clone()))
			.collect();
		if let Expression::Literal(Literal(LiteralValue::VariableReference(variable_reference), ..)) = &self.return_type {
			if variable_reference.name() != &Name("Void".to_owned()) {
				let return_parameter = (Name("return_address".to_owned()), self.return_type.as_literal(context).unwrap().clone());
				context.parameter_names.push(return_parameter);
			}
		} else {
			let return_parameter = (Name("return_address".to_owned()), self.return_type.as_literal(context).unwrap().clone());
			context.parameter_names.push(return_parameter);
		}

		let return_type: Expression = self
			.return_type
			.compile_time_evaluate(context, with_side_effects)
			.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating the return type of a function at compile-time".dimmed()))?;

		// Tags
		let tags = TagList::new(
			self.tags
				.iter()
				.enumerate()
				.map(|(index, tag)| {
					tag.compile_time_evaluate(context, with_side_effects).map_err(|error| {
						anyhow::anyhow!(
							"{error}\n\t{}",
							format!(
								"while evaluating the {} {} at compile-time",
								format!("{} tag", (index + 1).suffixed()).bold().cyan(),
								"of a function declaration".bold().white(),
							)
							.dimmed()
						)
					})
				})
				.collect::<anyhow::Result<Vec<_>>>()?,
		);

		let body = self
			.body
			.as_ref()
			.map(|body| {
				body.iter()
					.map(|statement| statement.compile_time_evaluate_statement(context, false))
					.collect::<anyhow::Result<Vec<_>>>()
			})
			.transpose()
			.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating the body of a function at compile-time".dimmed()))?;

		let mut function = Self {
			body,
			parameters,
			return_type,
			tags,
			inner_scope_id: self.inner_scope_id,
			name: self.name.clone(),
			id: self.id,
			is_non_void: false,
			has_been_compile_time_evaluated: true,
		};

		function.make_void();

		if !context.function_declarations.iter().any(|forward| forward.id == self.id) {
			let mut cloned_function = function.clone();
			cloned_function.parameters = cloned_function
				.parameters
				.into_iter()
				.map(|(parameter_name, parameter_type)| {
					(
						parameter_name,
						if let Expression::Literal(Literal(LiteralValue::VariableReference(type_name, ..), ..)) = &parameter_type {
							if context.generics_stack.last().cloned().unwrap_or_else(Vec::new).contains(type_name.name()) {
								void!()
							} else {
								parameter_type
							}
						} else {
							parameter_type
						},
					)
				})
				.collect();

			context.function_declarations.push(cloned_function);
		}

		context.parameter_names = Vec::new();

		Ok(Expression::Literal(Literal::new(LiteralValue::FunctionDeclaration(Box::new(function)))))
	}
}

impl ParentExpression for FunctionDeclaration {
	fn evaluate_children_at_compile_time(&self, _context: &mut Context) -> anyhow::Result<Expression> {
		anyhow::bail!("Attempted to run a function declaration at compile-time: Using a \"run\" expression on a function declaration has no effect");
	}
}

impl FunctionDeclaration {
	/// Converts this function into a void function. This changes the return type to `void`, and adds a new parameter that's a pointer to the return value address.
	///
	/// Whether this function is void or not can be retrieved with the `is_non_void` field.
	///
	/// # Parameters
	/// - `context`
	pub fn make_void(&mut self) {
		if self.is_non_void {
			return;
		}

		self.is_non_void = if let Expression::Literal(Literal(LiteralValue::VariableReference(return_type_name, ..), ..)) = &self.return_type {
			if return_type_name.name() == &Name("Void".to_owned()) {
				false
			} else {
				self.parameters.push((Name("return_address".to_owned()), self.return_type.clone()));
				self.return_type = void!();
				true
			}
		} else {
			self.parameters.push((Name("return_address".to_owned()), self.return_type.clone()));
			self.return_type = void!();
			true
		};
	}
}

impl TranspileToC for FunctionDeclaration {
	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		if let Some(name) = context.function_type_name.clone() {
			Ok(format!(
				"{return_type}* (*{name})({parameters})",
				name = name.c_name(),
				return_type = self.return_type.to_c(context)?,
				parameters = self
					.parameters
					.iter()
					.map(|(parameter_name, type_annotation)| Ok(format!(
						"{}* {parameter_name}",
						{
							let c_type = type_annotation.to_c(context)?;
							if context.generics_stack.last().cloned().unwrap_or_else(Vec::new).contains(&Name::from_c(&c_type)) {
								"void".to_owned()
							} else {
								c_type
							}
						},
						parameter_name = parameter_name.c_name()
					)))
					.collect::<anyhow::Result<Vec<_>>>()?
					.join(", ")
			)
			.lines()
			.map(|line| format!("\t{line}"))
			.collect::<Vec<_>>()
			.join("\n") + "\n")
		} else {
			Ok(format!("{}_{}", self.name.as_ref().unwrap_or(&"unnamed_function".to_owned()), self.id))
		}
	}

	fn c_prelude(&self, context: &mut Context) -> anyhow::Result<String> {
		let parameter_prelude = self
			.parameters
			.iter()
			.map(|(_name, type_annotation)| type_annotation.c_prelude(context))
			.collect::<anyhow::Result<Vec<_>>>()?
			.join("\n");

		let return_type_prelude = self.return_type.c_prelude(context)?;

		let body_prelude = self
			.body
			.as_ref()
			.map(|body| body.iter().map(|statement| statement.c_prelude(context)).collect::<anyhow::Result<Vec<_>>>())
			.transpose()?
			.unwrap_or(Vec::new())
			.join("\n");

		let annotation_prelude = self.tags.iter().map(|tag| tag.c_prelude(context)).collect::<anyhow::Result<Vec<_>>>()?.join("\n");

		let parameters = self
			.parameters
			.iter()
			.map(|(name, type_annotation)| {
				Ok(format!(
					"{}* {}",
					{
						let c_type = type_annotation.to_c(context)?;
						if context.generics_stack.last().cloned().unwrap_or_else(Vec::new).contains(&Name::from_c(&c_type)) {
							"void".to_owned()
						} else {
							c_type
						}
					},
					name.c_name()
				))
			})
			.collect::<anyhow::Result<Vec<_>>>()?;

		let mut body = self
			.body
			.as_ref()
			.map(|body| body.iter().map(|statement| statement.to_c(context)).collect::<anyhow::Result<Vec<_>>>())
			.transpose()?
			.unwrap_or_else(|| vec![String::new()]);

		for tag in self.tags.iter() {
			if let Expression::Literal(Literal(LiteralValue::Object(table), ..)) = tag {
				// Builtin function
				if table.name.cabin_name() == "BuiltinTag" {
					let internal_name_value = table.get_field(&Name("internal_name".to_owned())).unwrap_or_else(|| unreachable!());
					let internal_name = internal_name_value.as_string().map_err(|error| {
						anyhow::anyhow!(
							"{error}\n\t{}",
							"while getting the internal name stored in a built-in tag on a function declaration\n\twhile generating the C prelude for a function declaration"
								.dimmed()
						)
					})?;
					let parameter_names = self.parameters.iter().map(|parameter| parameter.0.c_name()).collect::<Vec<_>>();
					body = builtin_to_c(&internal_name, parameter_names.as_slice())?
						.lines()
						.map(|line| line.to_owned())
						.collect::<Vec<_>>();
					break;
				}
			}
		}

		let _annotations = self.tags.iter().map(|tag| tag.to_c(context)).collect::<anyhow::Result<Vec<_>>>()?;

		let self_prelude = format!(
			"void {name}_{id}({parameters}) {{\n{body}\n}}",
			name = self.name.as_ref().unwrap_or(&"unnamed_function".to_owned()),
			id = self.id,
			parameters = parameters.join(", "),
			body = body.join("\n").lines().map(|line| format!("\t{line}")).collect::<Vec<_>>().join("\n"),
		);
		Ok([parameter_prelude, return_type_prelude, body_prelude, annotation_prelude, self_prelude].join("\n"))
	}
}

impl ToCabin for FunctionDeclaration {
	fn to_cabin(&self) -> String {
		let mut cabin_code = "function(".to_owned();
		for (parameter_name, parameter_type) in &self.parameters {
			write!(cabin_code, "{}: {},", parameter_name.cabin_name(), parameter_type.to_cabin()).unwrap();
		}
		write!(cabin_code, "): {}", self.return_type.to_cabin()).unwrap();
		if let Some(body) = &self.body {
			cabin_code.push('{');
			for statement in body {
				for line in statement.to_cabin().lines() {
					write!(cabin_code, "\t{line}").unwrap();
				}
			}
			cabin_code.push('}');
		}
		cabin_code
	}
}

impl ColoredCabin for FunctionDeclaration {
	fn to_colored_cabin(&self, context: &mut Context) -> String {
		let mut cabin_code = format!("{}(", "function".purple());
		for (parameter_name, parameter_type) in &self.parameters {
			write!(cabin_code, "{}: {},", parameter_name.to_colored_cabin(context), parameter_type.to_colored_cabin(context)).unwrap();
		}
		if !self.parameters.is_empty() {
			cabin_code = cabin_code.get(0..cabin_code.len() - 1).unwrap().to_owned();
		}
		write!(cabin_code, "): {}", self.return_type.to_colored_cabin(context)).unwrap();
		if let Some(body) = &self.body {
			cabin_code.push_str(" {\n");
			for statement in body {
				for line in statement.to_colored_cabin(context).lines() {
					writeln!(cabin_code, "    {line}").unwrap();
				}
			}
			cabin_code.push('}');
		}
		cabin_code
	}
}
