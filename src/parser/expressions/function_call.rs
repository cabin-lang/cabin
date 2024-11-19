use std::collections::VecDeque;

use colored::Colorize;

use crate::{
	builtin::call_builtin_at_compile_time,
	comptime::CompileTime,
	context::Context,
	lexer::{Token, TokenType},
	parse_list,
	parser::{ListType, TokenQueueFunctionality},
};

use super::{function::FunctionDeclaration, name::Name, object::LiteralConvertible, operators::FieldAccess, Expression, Parse};

#[derive(Debug, Clone)]
pub struct FunctionCall {
	pub function: Box<Expression>,
	pub compile_time_arguments: Option<Vec<Expression>>,
	pub arguments: Option<Vec<Expression>>,
	pub scope_id: usize,
}

impl Parse for FunctionCall {
	type Output = Expression;

	fn parse(tokens: &mut VecDeque<Token>, context: &mut Context) -> anyhow::Result<Self::Output> {
		// Primary expression
		let mut expression = FieldAccess::parse(tokens, context)?;

		// Postfix function call operators
		while tokens.next_is_one_of(&[TokenType::LeftParenthesis, TokenType::LeftAngleBracket]) {
			// Compile-time arguments
			let compile_time_arguments = if tokens.next_is(TokenType::LeftAngleBracket) {
				let mut compile_time_arguments = Vec::new();
				parse_list!(tokens, ListType::AngleBracketed, {
					compile_time_arguments.push(Expression::parse(tokens, context)?);
				});
				Some(compile_time_arguments)
			} else {
				None
			};

			// Arguments
			let arguments = if tokens.next_is(TokenType::LeftParenthesis) {
				let mut arguments = Vec::new();
				parse_list!(tokens, ListType::Parenthesized, {
					arguments.push(Expression::parse(tokens, context)?);
				});
				Some(arguments)
			} else {
				None
			};

			// Reassign base expression
			expression = Expression::FunctionCall(FunctionCall {
				function: Box::new(expression),
				compile_time_arguments,
				arguments,
				scope_id: context.scope_data.unique_id(),
			});
		}

		Ok(expression)
	}
}

impl CompileTime for FunctionCall {
	type Output = Expression;

	fn evaluate_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self::Output> {
		let function = self.function.evaluate_at_compile_time(context)?;

		// Compile-time arguments
		let compile_time_arguments = if let Some(original_compile_time_arguments) = self.compile_time_arguments {
			let mut compile_time_arguments = Vec::new();
			for argument in original_compile_time_arguments {
				let evaluated = argument.evaluate_at_compile_time(context)?;
				compile_time_arguments.push(evaluated);
			}
			Some(compile_time_arguments)
		} else {
			None
		};

		// Arguments
		let arguments = if let Some(original_arguments) = self.arguments {
			let mut arguments = Vec::new();
			for argument in original_arguments {
				let evaluated = argument.evaluate_at_compile_time(context)?;
				arguments.push(evaluated);
			}
			Some(arguments)
		} else {
			None
		};

		// Evaluate function
		if let Ok(Ok(function_declaration)) = function.as_literal(context).map(|literal| FunctionDeclaration::from_literal(literal, context)) {
			// Non-builtin
			if let Some(body) = &function_declaration.body {
				// Add compile-time arguments
				if let Expression::Block(block) = body.as_ref() {
					if let Some(compile_time_arguments) = &compile_time_arguments {
						for (argument, (parameter_name, _parameter_type)) in compile_time_arguments.iter().zip(function_declaration.compile_time_parameters.iter()) {
							if !argument.is_literal() {
								anyhow::bail!("Attempted to pass a value that's not fully known at compile-time to a compile-time parameter.");
							}
							context.scope_data.reassign_variable_from_id(parameter_name, argument.clone(), block.inner_scope_id)?;
						}
					}
				}

				// Return value
				let return_value = body
					.clone()
					.evaluate_at_compile_time(context)
					.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while calling a function at compile-time".dimmed()))?;
				if return_value.as_literal(context).is_ok() {
					return Ok(return_value);
				}
			}
			// Builtin function
			else {
				let mut builtin_name = None;
				let mut system_side_effects = false;

				let system_side_effects_address = context
					.scope_data
					.get_global_variable(&"system_side_effects".into())
					.unwrap()
					.value
					.as_literal_address()
					.unwrap();

				for tag in &function_declaration.tags.values {
					if let Ok(object) = tag.as_literal(context) {
						if object.type_name == Name::from("BuiltinTag") {
							builtin_name = Some(object.get_field_literal(&Name::from("internal_name"), context).unwrap().as_string().unwrap().to_owned());
							continue;
						}

						if tag.as_literal_address().unwrap() == system_side_effects_address {
							system_side_effects = true;
						}
					}
				}

				if let Some(internal_name) = builtin_name {
					if !system_side_effects || context.has_side_effects() {
						return call_builtin_at_compile_time(&internal_name, context, self.scope_id, &arguments.unwrap_or(Vec::new()));
					} else {
						return Ok(Expression::Void);
					}
				}

				anyhow::bail!("Attempted to call a function that doesn't have a body.");
			}
		}

		Ok(Expression::FunctionCall(FunctionCall {
			function: Box::new(function),
			compile_time_arguments,
			arguments,
			scope_id: self.scope_id,
		}))
	}
}
