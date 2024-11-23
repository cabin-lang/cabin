use std::collections::VecDeque;

use crate::{
	api::{builtin::call_builtin_at_compile_time, context::Context, traits::TryAs as _},
	bail_err,
	comptime::{memory::VirtualPointer, CompileTime},
	lexer::{Span, Token, TokenType},
	mapped_err, parse_list,
	parser::{
		expressions::{function_declaration::FunctionDeclaration, literal::LiteralConvertible, name::Name, operators::FieldAccess, Expression, Parse},
		ListType, TokenQueueFunctionality,
	},
	transpiler::TranspileToC,
};

use super::{run::ParentExpression, Spanned, Typed};

#[derive(Debug, Clone)]
pub struct FunctionCall {
	pub function: Box<Expression>,
	pub compile_time_arguments: Option<Vec<Expression>>,
	pub arguments: Option<Vec<Expression>>,
	pub scope_id: usize,
	pub span: Span,
}

pub struct PostfixOperators;

impl Parse for PostfixOperators {
	type Output = Expression;

	fn parse(tokens: &mut VecDeque<Token>, context: &mut Context) -> anyhow::Result<Self::Output> {
		// Primary expression
		let mut expression = FieldAccess::parse(tokens, context)?;
		let start = expression.span(context);
		let mut end = start.clone();

		// Postfix function call operators
		while tokens.next_is_one_of(&[TokenType::LeftParenthesis, TokenType::LeftAngleBracket]) {
			// Compile-time arguments
			let compile_time_arguments = if tokens.next_is(TokenType::LeftAngleBracket) {
				let mut compile_time_arguments = Vec::new();
				end = parse_list!(tokens, ListType::AngleBracketed, {
					compile_time_arguments.push(Expression::parse(tokens, context)?);
				})
				.span;
				Some(compile_time_arguments)
			} else {
				None
			};

			// Arguments
			let arguments = if tokens.next_is(TokenType::LeftParenthesis) {
				let mut arguments = Vec::new();
				end = parse_list!(tokens, ListType::Parenthesized, {
					arguments.push(Expression::parse(tokens, context)?);
				})
				.span;
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
				span: start.to(&end),
			});
		}

		Ok(expression)
	}
}

impl CompileTime for FunctionCall {
	type Output = Expression;

	fn evaluate_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self::Output> {
		let function = self.function.evaluate_at_compile_time(context).map_err(mapped_err! {
			while = "evaluating the function to call on a function-call expression at compile-time",
			context = context,
		})?;

		// Compile-time arguments
		let compile_time_arguments = if let Some(original_compile_time_arguments) = self.compile_time_arguments {
			let mut compile_time_arguments = Vec::new();
			for argument in original_compile_time_arguments {
				let evaluated = argument.evaluate_at_compile_time(context).map_err(mapped_err! {
					while = "evaluating a function call's compile-time argument at compile-time",
					context = context,
				})?;
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
				let evaluated = argument.evaluate_at_compile_time(context).map_err(mapped_err! {
					while = "evaluating a function call's argument at compile-time",
					context = context,
				})?;
				arguments.push(evaluated);
			}
			Some(arguments)
		} else {
			None
		};

		// If not all arguments are known at compile-time, we can't call the function at compile time. In this case, we just
		// return a function call expression, and it'll get transpiled to C and called at runtime.
		if let Some(argument_list) = &arguments {
			if !argument_list.iter().all(|argument| argument.is_pointer()) {
				return Ok(Expression::FunctionCall(FunctionCall {
					function: Box::new(function),
					compile_time_arguments,
					arguments,
					scope_id: self.scope_id,
					span: self.span,
				}));
			}
		}

		// Evaluate function
		let literal = function.try_as_literal_or_name(context);
		if let Ok(function_declaration) = literal {
			let function_declaration = match FunctionDeclaration::from_literal(function_declaration) {
				Ok(function_declaration) => function_declaration,
				Err(error) => {
					bail_err! {
						base = error,
						while = "calling a function at compile-time",
						context = context,
					};
				},
			};

			// Set this object
			if let Some(this_object) = function_declaration.this_object {
				arguments = Some(arguments.unwrap_or(Vec::new()));
				if let Some((parameter_name, _)) = function_declaration.parameters.first() {
					if parameter_name.unmangled_name() == "this" {
						arguments.as_mut().unwrap().insert(0, this_object);
					}
				}
			}

			// Non-builtin
			if let Some(body) = &function_declaration.body {
				if let Expression::Block(block) = body {
					// Validate and add compile-time arguments
					if let Some(compile_time_arguments) = &compile_time_arguments {
						for (argument, (parameter_name, parameter_type)) in compile_time_arguments.iter().zip(function_declaration.compile_time_parameters.iter()) {
							let parameter_type_pointer = parameter_type.try_as_literal_or_name(context)?.address.as_ref().unwrap().to_owned();
							if !argument.is_assignable_to_type(parameter_type_pointer, context)? {
								bail_err! {
									base = format!(
										"Attempted to pass a argument of type \"{}\" to a compile-time parameter of type \"{}\"",
										argument.get_type(context)?.virtual_deref(context).name().unmangled_name().bold().cyan(),
										parameter_type_pointer.virtual_deref(context).name().unmangled_name().bold().cyan(),
									),
									while = "validating the arguments in a function call",
									context = context,
								};
							}
							if !argument.is_pointer() {
								anyhow::bail!("Attempted to pass a value that's not fully known at compile-time to a compile-time parameter.");
							}
							context.scope_data.reassign_variable_from_id(parameter_name, argument.clone(), block.inner_scope_id)?;
						}
					}

					// Validate and add arguments
					if let Some(arguments) = &arguments {
						for (argument, (parameter_name, parameter_type)) in arguments.iter().zip(function_declaration.parameters.iter()) {
							let parameter_type_pointer = parameter_type.try_as_literal_or_name(context)?.address.as_ref().unwrap().to_owned();
							if !argument.is_assignable_to_type(parameter_type_pointer, context)? {
								bail_err! {
									base = format!(
										"Attempted to pass a argument of type \"{}\" to a parameter of type \"{}\"",
										argument.get_type(context)?.virtual_deref(context).name().unmangled_name().bold().cyan(),
										parameter_type_pointer.virtual_deref(context).name().unmangled_name().bold().cyan(),
									),
									while = "validating the arguments in a function call",
									context = context,
									position = argument.span(context)
								};
							}
							context.scope_data.reassign_variable_from_id(parameter_name, argument.clone(), block.inner_scope_id)?;
						}
					}
				}

				// Return value
				let return_value = body.clone().evaluate_at_compile_time(context).map_err(mapped_err! {
					while = "calling a function at compile-time",
					context = context,
				})?;

				if return_value.try_as_literal_or_name(context).is_ok() {
					return Ok(return_value);
				}
			}
			// Builtin function
			else {
				let mut builtin_name = None;
				let mut system_side_effects = false;

				// Get the address of system_side_effects
				let system_side_effects_address = *context
					.scope_data
					.expect_global_variable("system_side_effects")
					.expect_as::<VirtualPointer>()
					.map_err(mapped_err! {
						while = format!("interpreting the global variable \"{}\" as a pointer", "system_side_effects".bold().cyan()),
						context = context,
					})?;

				// Get builtin and side effect tags
				for tag in &function_declaration.tags.values {
					if let Ok(object) = tag.try_as_literal_or_name(context).cloned() {
						if object.type_name() == &Name::from("BuiltinTag") {
							builtin_name = Some(
								object
									.get_field_literal("internal_name", context)
									.unwrap()
									.expect_as::<String>()
									.map_err(mapped_err! {
										while = format!("interpreting the literal field \"{}\" of a {} as a string", "internal_name".bold().cyan(), "BuiltinTag".bold().cyan()),
										context = context,
									})?
									.to_owned(),
							);
							continue;
						}

						if tag.expect_as::<VirtualPointer>().unwrap() == &system_side_effects_address {
							system_side_effects = true;
						}
					}
				}

				// Call builtin function
				if let Some(internal_name) = builtin_name {
					if !system_side_effects || context.has_side_effects() {
						return call_builtin_at_compile_time(&internal_name, context, self.scope_id, arguments.unwrap_or(Vec::new())).map_err(mapped_err! {
							while = format!("calling the built-in function {} at compile-time", internal_name.bold().purple()),
							context = context,
						});
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
			span: self.span,
		}))
	}
}

impl TranspileToC for FunctionCall {
	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		let function = FunctionDeclaration::from_literal(
			self.function
				.clone()
				.evaluate_at_compile_time(context)?
				.expect_as::<VirtualPointer>()?
				.virtual_deref(context),
		)?;

		let return_type = if let Some(return_type) = function.return_type.as_ref() {
			format!("{}* return_address;", return_type.expect_literal(context)?.clone().to_c_type(context)?)
		} else {
			String::new()
		};

		let ending_return_address = if let Some(_return_type) = function.return_type.as_ref() {
			"return_address;".to_owned()
		} else {
			String::new()
		};

		let maybe_return_address = if let Some(_return_type) = function.return_type.as_ref() {
			let maybe_comma = if function.parameters.is_empty() { "" } else { ", " };
			format!("{maybe_comma}return_address")
		} else {
			String::new()
		};

		Ok(unindent::unindent(&format!(
			"
			({{
				{return_type}	
				{argument_declaration}
				(((void (*)({parameter_types}))({function_to_call}->call))({this_object}{arguments}{maybe_return_address}));
				{ending_return_address}
			}})	
			",
			parameter_types = {
				let mut parameters = function
					.parameters
					.iter()
					.map(|parameter| Ok(format!("{}*", parameter.1.expect_literal(context)?.clone().to_c_type(context)?)))
					.collect::<anyhow::Result<Vec<_>>>()?
					.join(", ");
				if let Some(return_type) = function.return_type.as_ref() {
					if !parameters.is_empty() {
						parameters += ", ";
					}
					parameters += &format!("{}*", return_type.expect_literal(context)?.clone().to_c_type(context)?);
				}
				parameters
			},
			function_to_call = self.function.to_c(context)?,
			this_object = if let Some(object) = function.this_object {
				if function.parameters.first().is_some_and(|param| param.0 == "this".into()) {
					format!("{}, ", object.to_c(context)?)
				} else {
					String::new()
				}
			} else {
				String::new()
			},
			argument_declaration = self
				.arguments
				.as_ref()
				.unwrap_or(&Vec::new())
				.iter()
				.map(|argument| Ok(format!(
					"{}* arg0 = {};",
					argument.get_type(context)?.virtual_deref(context).clone().to_c_type(context)?,
					argument.to_c(context)?
				)))
				.collect::<anyhow::Result<Vec<_>>>()?
				.join(", "),
			arguments = (0..self.arguments.as_ref().unwrap_or(&Vec::new()).len())
				.map(|index| format!("arg{index}"))
				.collect::<Vec<_>>()
				.join(", "),
		)))
	}
}

impl ParentExpression for FunctionCall {
	fn evaluate_subexpressions_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self> {
		let function = self.function.evaluate_at_compile_time(context).map_err(mapped_err! {
			while = "evaluating the function to call on a function-call expression at compile-time",
			context = context,
		})?;

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

		Ok(FunctionCall {
			function: Box::new(function),
			compile_time_arguments,
			arguments,
			scope_id: self.scope_id,
			span: self.span,
		})
	}
}

impl Typed for FunctionCall {
	fn get_type(&self, context: &mut Context) -> anyhow::Result<VirtualPointer> {
		let function = FunctionDeclaration::from_literal(self.function.expect_literal(context)?)?;
		if let Some(return_type) = function.return_type {
			return_type.expect_as::<VirtualPointer>().cloned()
		} else {
			context.scope_data.expect_global_variable("Nothing").expect_as::<VirtualPointer>().cloned()
		}
	}
}

impl Spanned for FunctionCall {
	fn span(&self, _context: &Context) -> Span {
		self.span.clone()
	}
}
