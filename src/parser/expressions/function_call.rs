use crate::{
	cli::theme::Styled,
	compile_time::{builtin::call_builtin_at_compile_time, CompileTime, CompileTimeStatement, TranspileToC},
	context::Context,
	formatter::{ColoredCabin, ToCabin},
	lexer::{Token, TokenType},
	parse_list,
	parser::{
		expressions::{
			binary::AccessExpression,
			block::Block,
			literals::{object::Object, LiteralValue},
			run::ParentExpression,
			util::{name::Name, tags::TagList, types::Typed},
			Expression,
		},
		statements::{declaration::Declaration, tail::TailStatement, Statement},
		Parse, TokenQueue,
	},
	scopes::ScopeType,
	var, void,
};

use std::collections::VecDeque;

use colored::Colorize as _;

use super::literals::Literal;

/// A function call expression. This represents a function call with a function and a list of arguments.
#[derive(Clone, Debug)]
pub struct FunctionCall {
	/// The function to call. This is something like an identifier, a member access, or another function call. Regardless,
	/// this should evaluate to a pointer to a function declaration.
	pub function: Expression,
	/// The arguments to pass to the function.
	pub arguments: Vec<Expression>,

	/// Whether this function call expression has already been evaluated at compile-time. This is used to prevent
	/// double compile-time evaluating this, which can cause unexpected issues.
	pub has_been_converted_to_block: bool,
}

impl Parse for FunctionCall {
	type Output = Expression;

	fn parse(tokens: &mut VecDeque<Token>, context: &mut Context) -> anyhow::Result<Self::Output> {
		let mut literal = AccessExpression::parse(tokens, context)?;

		while tokens.next_is(TokenType::LeftParenthesis) || tokens.next_is(TokenType::LeftAngleBracket) {
			// Runtime parameters
			if tokens.next_is(TokenType::LeftParenthesis) {
				tokens.pop(TokenType::LeftParenthesis, context)?;
				let mut arguments = Vec::new();
				if !tokens.next_is(TokenType::RightParenthesis) {
					parse_list!(tokens, context, {
						arguments.push(Expression::parse(tokens, context)?);
					});
				}
				tokens.pop(TokenType::RightParenthesis, context)?;
				literal = Expression::FunctionCall(Box::new(Self {
					function: literal,
					arguments,
					has_been_converted_to_block: false,
				}));
			}

			// Compile-time parameters
			if tokens.next_is(TokenType::LeftAngleBracket) {
				tokens.pop(TokenType::LeftAngleBracket, context)?;
				let mut arguments = Vec::new();
				if !tokens.next_is(TokenType::RightAngleBracket) {
					parse_list!(tokens, context, {
						arguments.push(Expression::parse(tokens, context)?);
					});
				}
				tokens.pop(TokenType::RightAngleBracket, context)?;
				literal = Expression::FunctionCall(Box::new(Self {
					function: literal,
					arguments,
					has_been_converted_to_block: false,
				}));
			}
		}

		Ok(literal)
	}
}

impl Typed for FunctionCall {
	fn get_type(&self, context: &mut Context) -> anyhow::Result<Literal> {
		let function = self.function.compile_time_evaluate(context, false)?;
		let Expression::Literal(Literal(LiteralValue::FunctionDeclaration(function_declaration), ..)) = function else {
			anyhow::bail!("attempted to call non-function value");
		};

		if function_declaration.is_non_void {
			return Ok(function_declaration.parameters.last().unwrap().1.as_literal(context).unwrap().clone());
		}

		anyhow::bail!("Attempted to get the return type of a void function");
	}
}

impl CompileTime for FunctionCall {
	fn compile_time_evaluate(&self, context: &mut Context, with_side_effects: bool) -> anyhow::Result<Expression> {
		if !self.has_been_converted_to_block {
			return self.evaluate_children_at_compile_time(context)?.compile_time_evaluate(context, with_side_effects);
		}

		// Evaluate function reference and arguments
		let mut function = self.function.compile_time_evaluate(context, with_side_effects)?;
		let mut arguments = self
			.arguments
			.iter()
			.map(|arg| arg.compile_time_evaluate(context, with_side_effects))
			.collect::<anyhow::Result<Vec<_>>>()?;

		if let Expression::Literal(Literal(LiteralValue::VariableReference(variable_reference), ..)) = function {
			function = context
				.scope_data
				.get_variable_from_id(variable_reference.name(), variable_reference.scope_id())
				.ok_or_else(|| anyhow::anyhow!("Variable {} not found", variable_reference.name().cabin_name()))?
				.value
				.clone()
				.unwrap();
		}

		let Expression::Literal(Literal(LiteralValue::FunctionDeclaration(function_declaration), ..)) = &function else {
			return Ok(Expression::FunctionCall(Box::new(Self {
				function,
				arguments,
				has_been_converted_to_block: true,
			})));
		};

		let mut has_system_side_effects = false;
		let mut is_runtime_only = None;
		for tag in function_declaration.tags.iter() {
			if let Expression::Literal(Literal(LiteralValue::VariableReference(identifier, ..), ..)) = tag {
				if identifier.name() == &Name("system_side_effects".to_owned()) {
					has_system_side_effects = true;
				}
			}

			if let Expression::Literal(Literal(LiteralValue::Object(table), ..)) = tag {
				if table.name.cabin_name() == "RuntimeOnlyTag" {
					let reason_value = table.get_field(&Name("reason".to_owned())).unwrap_or_else(|| unreachable!());
					let reason = reason_value
						.as_string()
						.map_err(|error| anyhow::anyhow!("{error}\n\twhile evaluating a runtime_only tag for a function call at compile-time"))?;
					is_runtime_only = Some(reason);
				}
			}
		}

		if function_declaration.name == Some("input".to_owned()) {
			anyhow::bail!("input");
		}

		if let Some(reason) = is_runtime_only {
			context.warnings.push(unindent::unindent(&format!(
				"
				{} {}
				
				This function is marked with the tag \"{runtime_only}\", which means it really shouldn't be called at compile-time.
				Calling at this function at compile-time is possible, but could cause confusing or unexpected behavior, so it's recommended
				to only call this function at runtime.

				If you'd like to call this function at runtime, consider adding the \"{}\" keyword before the function call to force it to
				be called at runtime instead of compile-time.

				The reason for this particular function being runtime-only is documented as the following:

				{}
				",
				"Warning:".bold().yellow().underline(),
				format!(
					"You called a function that's marked as \"{runtime_only}\" at compile-time:",
					runtime_only = "runtime_only".cyan()
				)
				.white()
				.bold(),
				"run".style(context.theme().keyword()).bold(),
				format!("\t{}", reason.lines().collect::<Vec<_>>().join("\n\t\t\t\t\t")),
				runtime_only = "runtime_only".bold().cyan()
			)));
		}

		// Builtin function
		if !has_system_side_effects || with_side_effects {
			for tag in function_declaration.tags.iter() {
				if let Expression::Literal(Literal(LiteralValue::Object(table), ..)) = tag {
					if table.name.cabin_name() == "BuiltinTag" {
						let internal_name_value = table.get_field(&Name("internal_name".to_owned())).unwrap_or_else(|| unreachable!());
						let internal_name = internal_name_value.as_string().map_err(|error| {
							context.encountered_compiler_bug = true;
							anyhow::anyhow!("{error}\n\t{}", "while evaluating a built-in tag for a function call at compile-time".dimmed())
						})?;
						return call_builtin_at_compile_time(&internal_name, &mut arguments);
					}
				}
			}
		} else {
			return Ok(Expression::Literal(context.unknown_at_compile_time().clone()));
		}

		// Not builtin
		if let Some(body) = &function_declaration.body {
			// Evaluate the statements
			for statement in body {
				statement
					.compile_time_evaluate_statement(context, with_side_effects)
					.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating the body of a function call at compile-time".dimmed()))?;
			}

			// Get the return value
			let return_value = context
				.scope_data
				.get_variable_from_id(&Name("return_address".to_owned()), function_declaration.inner_scope_id.unwrap())
				.map_or_else(|| void!(), |declaration| declaration.clone().value.unwrap());

			// Return the return value
			return Ok(return_value);
		}

		// No body on non-builtin
		anyhow::bail!(
			"Function \"{}\" has no body; All functions that are not built into the language must have a body: {:?}\n",
			function_declaration.name.as_ref().unwrap().bold().cyan(),
			function_declaration
		);
	}
}

impl ParentExpression for FunctionCall {
	fn evaluate_children_at_compile_time(&self, context: &mut Context) -> anyhow::Result<Expression> {
		if self.has_been_converted_to_block {
			return Ok(Expression::FunctionCall(Box::new(self.clone())));
		}

		let mut function = self.function.compile_time_evaluate(context, false).map_err(|error| {
			anyhow::anyhow!(
				"{error}\n\t{}",
				"while evaluating the function declaration that a function call calls at compile-time".dimmed()
			)
		})?;

		let mut arguments = self
			.arguments
			.iter()
			.map(|argument| argument.compile_time_evaluate(context, false))
			.collect::<anyhow::Result<Vec<_>>>()?;

		if let Expression::Literal(Literal(LiteralValue::VariableReference(variable_reference), ..)) = function {
			function = context
				.scope_data
				.get_variable_from_id(variable_reference.name(), variable_reference.scope_id())
				.ok_or_else(|| anyhow::anyhow!("Variable {} not found", variable_reference.name().cabin_name()))?
				.value
				.clone()
				.unwrap()
				.compile_time_evaluate(context, false)
				.map_err(|error| {
					anyhow::anyhow!(
						"{error}\n\t{}",
						"while evaluating the function declaration that a function call references at compile-time".dimmed()
					)
				})?;
		}

		let Expression::Literal(Literal(LiteralValue::FunctionDeclaration(function_declaration), ..)) = &mut function else {
			unreachable!("{function:?}")
		};

		context.scope_data.enter_new_scope(ScopeType::Block);

		let mut statements = Vec::new();

		// Parameters
		for (argument, (parameter_name, parameter_type)) in arguments.iter_mut().zip(&function_declaration.parameters) {
			statements.push(Statement::Declaration(Declaration {
				name: parameter_name.clone(),
				declared_scope_id: context.scope_data.unique_id(),
				tags: TagList::default(),
				type_annotation: Some(parameter_type.clone()),
				initial_value: argument.clone(),
				line_start: 0,
			}));

			context
				.scope_data
				.declare_new_variable(parameter_name.clone(), None, argument.clone(), TagList::default())?;

			*argument = var!(parameter_name.cabin_name(), context.scope_data.unique_id());
		}

		let block_scope_id = context.scope_data.unique_id();

		// Non-void functions
		if function_declaration.is_non_void {
			// Add the return address to the arguments
			arguments.push(var!("return_address", block_scope_id));

			let return_type = function_declaration
				.parameters
				.last()
				.ok_or_else(|| {
					context.encountered_compiler_bug = true;
					anyhow::anyhow!("Attempted to get the return type of a non-void function, but the function has no parameters: {function_declaration:?}")
				})?
				.1
				.clone();

			let Expression::Literal(Literal(LiteralValue::VariableReference(return_type_identifier), ..)) = &return_type else {
				unreachable!();
			};

			// Declare the return address variable
			statements.push(Statement::Declaration(Declaration {
				name: Name("return_address".to_owned()),
				declared_scope_id: block_scope_id,
				tags: TagList::default(),
				initial_value: Expression::Literal(Literal::new(LiteralValue::Object(Object::named(return_type_identifier.name().to_owned())))),
				line_start: 0,
				type_annotation: Some(return_type.clone()),
			}));

			// Declare the return address variable into the scope
			context.scope_data.declare_new_variable(
				Name("return_address".to_owned()),
				Some(function_declaration.parameters.last().unwrap().1.clone()),
				Expression::Literal(Literal::new(LiteralValue::Object(Object::named(return_type_identifier.name().to_owned())))),
				TagList::default(),
			)?;

			function_declaration.parameters = Vec::new();

			// Add the function call itself
			statements.push(Statement::Expression(Expression::FunctionCall(Box::new(Self {
				function: function.clone(),
				arguments,
				has_been_converted_to_block: true,
			}))));

			// Add the tail return statement
			statements.push(Statement::Tail(TailStatement {
				expression: var!("return_address", block_scope_id),
			}));
		}
		// Void function
		else {
			function_declaration.parameters = Vec::new();
			statements.push(Statement::Expression(Expression::FunctionCall(Box::new(Self {
				function: function.clone(),
				arguments,
				has_been_converted_to_block: true,
			}))));
		}

		// if let Some(function_body_scope) = function_declaration.inner_scope_id {
		// 	context.scope_data.move_scope(function_body_scope, block)
		// }

		context.scope_data.exit_scope()?;

		let block = Block {
			statements,
			inner_scope_id: block_scope_id,
		};
		Ok(Expression::Block(block))
	}
}

impl TranspileToC for FunctionCall {
	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		let function = self.function.to_c(context)?;

		let Expression::Literal(Literal(LiteralValue::FunctionDeclaration(function_declaration), ..)) = &self.function else {
			anyhow::bail!("Function call is not a declaration in C");
		};

		let arguments = self
			.arguments
			.iter()
			.enumerate()
			.map(|(index, arg)| {
				if index == self.arguments.len() - 1 && function_declaration.is_non_void {
					Ok("&return_address_u".to_owned())
				} else {
					Ok(format!("&{}", arg.to_c(context)?))
				}
			})
			.collect::<anyhow::Result<Vec<_>>>()?;

		Ok(format!("{}({})", function, arguments.join(", ")))
	}

	fn c_prelude(&self, context: &mut Context) -> anyhow::Result<String> {
		let arguments = self
			.arguments
			.iter()
			.map(|arg: &Expression| arg.c_prelude(context))
			.collect::<anyhow::Result<Vec<_>>>()?
			.join("\n");
		Ok(arguments)
	}
}

impl ToCabin for FunctionCall {
	fn to_cabin(&self) -> String {
		format!(
			"{}({})",
			self.function.to_cabin(),
			self.arguments.iter().map(|argument| argument.to_cabin()).collect::<Vec<_>>().join(", ")
		)
	}
}

impl ColoredCabin for FunctionCall {
	fn to_colored_cabin(&self, context: &mut Context) -> String {
		format!(
			"{}({})",
			self.function.to_colored_cabin(context),
			self.arguments.iter().map(|argument| argument.to_colored_cabin(context)).collect::<Vec<_>>().join(", ")
		)
	}
}
