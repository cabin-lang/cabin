use std::collections::VecDeque;

use colored::Colorize as _;

use crate::{
	api::{builtin::call_builtin_at_compile_time, context::Context, traits::TryAs as _},
	comptime::{memory::Pointer, CompileTime},
	lexer::{Token, TokenType},
	mapped_err, parse_list,
	parser::{
		expressions::{function::FunctionDeclaration, literal::LiteralConvertible, name::Name, operators::FieldAccess, Expression, Parse},
		ListType, TokenQueueFunctionality,
	},
};

#[derive(Debug, Clone)]
pub struct FunctionCall {
	pub function: Box<Expression>,
	pub compile_time_arguments: Option<Vec<Expression>>,
	pub arguments: Option<Vec<Expression>>,
	pub scope_id: usize,
}

pub struct PostfixOperators;

impl Parse for PostfixOperators {
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
		let mut arguments = if let Some(original_arguments) = self.arguments {
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
		let literal = function.try_as_literal(context);
		if let Ok(function_declaration) = literal {
			let Ok(function_declaration) = FunctionDeclaration::from_literal(function_declaration, context) else {
				anyhow::bail!(
					"Attempted to call a value that's not a function; Instead it's an instance of \"{}\"",
					function_declaration.type_name.unmangled_name().bold().cyan()
				);
			};

			// Set this object
			if let Some(this_object) = function_declaration.this_object {
				arguments = Some(arguments.unwrap_or(Vec::new()));
				arguments.as_mut().unwrap().push(*this_object);
			}

			// Non-builtin
			if let Some(body) = &function_declaration.body {
				if let Expression::Block(block) = body.as_ref() {
					// Add compile-time arguments
					if let Some(compile_time_arguments) = &compile_time_arguments {
						for (argument, (parameter_name, _parameter_type)) in compile_time_arguments.iter().zip(function_declaration.compile_time_parameters.iter()) {
							if !argument.is_pointer() {
								anyhow::bail!("Attempted to pass a value that's not fully known at compile-time to a compile-time parameter.");
							}
							context.scope_data.reassign_variable_from_id(parameter_name, argument.clone(), block.inner_scope_id)?;
						}
					}

					// Add arguments
					if let Some(arguments) = &arguments {
						for (argument, (parameter_name, _parameter_type)) in arguments.iter().zip(function_declaration.compile_time_parameters.iter()) {
							context.scope_data.reassign_variable_from_id(parameter_name, argument.clone(), block.inner_scope_id)?;
						}
					}
				}

				// Return value
				let return_value = body.clone().evaluate_at_compile_time(context).map_err(mapped_err! {
					while = "calling a function at compile-time",
					context = context,
				})?;

				if return_value.try_as_literal(context).is_ok() {
					return Ok(return_value);
				}
			}
			// Builtin function
			else {
				let mut builtin_name = None;
				let mut system_side_effects = false;

				// Get the address of system_side_effects
				let system_side_effects_address = context.scope_data.expect_global_variable("system_side_effects").expect_as::<Pointer>();

				// Get builtin and side effect tags
				for tag in &function_declaration.tags.values {
					if let Ok(object) = tag.try_as_literal(context) {
						if object.type_name == Name::from("BuiltinTag") {
							builtin_name = Some(object.get_field_literal("internal_name", context).unwrap().expect_as::<String>().to_owned());
							continue;
						}

						if tag.expect_as::<Pointer>() == system_side_effects_address {
							system_side_effects = true;
						}
					}
				}

				// Call builtin function
				if let Some(internal_name) = builtin_name {
					if !system_side_effects || context.has_side_effects() {
						return call_builtin_at_compile_time(&internal_name, context, self.scope_id, &arguments.unwrap_or(Vec::new()));
					} else {
						return Ok(Expression::Void(()));
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
