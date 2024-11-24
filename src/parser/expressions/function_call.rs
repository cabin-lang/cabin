use std::collections::VecDeque;

use crate::{
	api::{builtin::call_builtin_at_compile_time, context::Context, scope::ScopeId, traits::TryAs as _},
	bail_err,
	comptime::{memory::VirtualPointer, CompileTime},
	if_then_else_default,
	lexer::{Span, Token, TokenType},
	mapped_err, parse_list,
	parser::{
		expressions::{
			field_access::FieldAccess, function_declaration::FunctionDeclaration, literal::LiteralConvertible, name::Name, run::RuntimeableExpression, Expression, Parse, Spanned,
			Typed,
		},
		ListType, TokenQueueFunctionality,
	},
	transpiler::TranspileToC,
};

#[derive(Debug, Clone)]
pub struct FunctionCall {
	function: Box<Expression>,
	compile_time_arguments: Vec<Expression>,
	arguments: Vec<Expression>,
	scope_id: ScopeId,
	span: Span,
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
			let compile_time_arguments = if_then_else_default!(tokens.next_is(TokenType::LeftAngleBracket), {
				let mut compile_time_arguments = Vec::new();
				end = parse_list!(tokens, ListType::AngleBracketed, {
					compile_time_arguments.push(Expression::parse(tokens, context)?);
				})
				.span;
				compile_time_arguments
			});

			// Arguments
			let arguments = if_then_else_default!(tokens.next_is(TokenType::LeftParenthesis), {
				let mut arguments = Vec::new();
				end = parse_list!(tokens, ListType::Parenthesized, {
					arguments.push(Expression::parse(tokens, context)?);
				})
				.span;
				arguments
			});

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
		let compile_time_arguments = {
			let mut evaluated_compile_time_arguments = Vec::new();
			for compile_time_argument in self.compile_time_arguments {
				let evaluated = compile_time_argument.evaluate_at_compile_time(context).map_err(mapped_err! {
					while = "evaluating a function call's compile-time argument at compile-time",
					context = context,
				})?;
				evaluated_compile_time_arguments.push(evaluated);
			}
			evaluated_compile_time_arguments
		};

		// Arguments
		let mut arguments = {
			let mut evaluated_arguments = Vec::new();
			for argument in self.arguments {
				let evaluated = argument.evaluate_at_compile_time(context).map_err(mapped_err! {
					while = "evaluating a function call's argument at compile-time",
					context = context,
				})?;
				evaluated_arguments.push(evaluated);
			}
			evaluated_arguments
		};

		// If not all arguments are known at compile-time, we can't call the function at compile time. In this case, we just
		// return a function call expression, and it'll get transpiled to C and called at runtime.
		if arguments
			.iter()
			.map(|argument| argument.is_fully_known_at_compile_time(context))
			.collect::<anyhow::Result<Vec<_>>>()?
			.iter()
			.any(|value| !value)
		{
			return Ok(Expression::FunctionCall(FunctionCall {
				function: Box::new(function),
				compile_time_arguments,
				arguments,
				scope_id: self.scope_id,
				span: self.span,
			}));
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
			if let Some(this_object) = function_declaration.this_object() {
				if let Some((parameter_name, _)) = function_declaration.parameters().first() {
					if parameter_name.unmangled_name() == "this" {
						arguments.insert(0, this_object.clone());
					}
				}
			}

			// Validate compile-time arguments
			for (argument, (_parameter_name, parameter_type)) in compile_time_arguments.iter().zip(function_declaration.compile_time_parameters().iter()) {
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
				if !argument.is_fully_known_at_compile_time(context)? {
					anyhow::bail!("Attempted to pass a value that's not fully known at compile-time to a compile-time parameter.");
				}
			}

			// Validate arguments
			for (argument, (_parameter_name, parameter_type)) in arguments.iter().zip(function_declaration.parameters().iter()) {
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
						at = argument.span(context)
					};
				}
			}

			// Non-builtin
			if let Some(body) = function_declaration.body() {
				if let Expression::Block(block) = body {
					// Validate and add compile-time arguments
					for (argument, (parameter_name, _parameter_type)) in compile_time_arguments.iter().zip(function_declaration.compile_time_parameters().iter()) {
						context.scope_data.reassign_variable_from_id(parameter_name, argument.clone(), block.inner_scope_id)?;
					}

					// Validate and add arguments
					for (argument, (parameter_name, _parameter_type)) in arguments.iter().zip(function_declaration.parameters().iter()) {
						context.scope_data.reassign_variable_from_id(parameter_name, argument.clone(), block.inner_scope_id)?;
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
					.get_variable("system_side_effects")
					.unwrap()
					.expect_as::<VirtualPointer>()
					.map_err(mapped_err! {
						while = format!("interpreting the global variable \"{}\" as a pointer", "system_side_effects".bold().cyan()),
						context = context,
					})?;

				// Get builtin and side effect tags
				for tag in &function_declaration.tags().values {
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
						return call_builtin_at_compile_time(&internal_name, context, self.scope_id, arguments, self.span).map_err(mapped_err! {
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

		let return_type = if let Some(return_type) = function.return_type() {
			format!("{}* return_address;", return_type.expect_literal(context)?.clone().to_c_type(context)?)
		} else {
			String::new()
		};

		let ending_return_address = if let Some(_return_type) = function.return_type() {
			"return_address;".to_owned()
		} else {
			String::new()
		};

		let maybe_return_address = if let Some(_return_type) = function.return_type() {
			let maybe_comma = if function.parameters().is_empty() { "" } else { ", " };
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
					.parameters()
					.iter()
					.map(|parameter| Ok(format!("{}*", parameter.1.expect_literal(context)?.clone().to_c_type(context)?)))
					.collect::<anyhow::Result<Vec<_>>>()?
					.join(", ");
				if let Some(return_type) = function.return_type().as_ref() {
					if !parameters.is_empty() {
						parameters += ", ";
					}
					parameters += &format!("{}*", return_type.expect_literal(context)?.clone().to_c_type(context)?);
				}
				parameters
			},
			function_to_call = self.function.to_c(context)?,
			this_object = if let Some(object) = function.this_object() {
				if function.parameters().first().is_some_and(|param| param.0 == "this".into()) {
					format!("{}, ", object.to_c(context)?)
				} else {
					String::new()
				}
			} else {
				String::new()
			},
			argument_declaration = self
				.arguments
				.iter()
				.map(|argument| Ok(format!(
					"{}* arg0 = {};",
					argument.get_type(context)?.virtual_deref(context).clone().to_c_type(context)?,
					argument.to_c(context)?
				)))
				.collect::<anyhow::Result<Vec<_>>>()?
				.join(", "),
			arguments = (0..self.arguments.len()).map(|index| format!("arg{index}")).collect::<Vec<_>>().join(", "),
		)))
	}
}

impl RuntimeableExpression for FunctionCall {
	fn evaluate_subexpressions_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self> {
		let function = self.function.evaluate_at_compile_time(context).map_err(mapped_err! {
			while = "evaluating the function to call on a function-call expression at compile-time",
			context = context,
		})?;

		// Compile-time arguments
		let compile_time_arguments = {
			let mut compile_time_arguments = Vec::new();
			for argument in self.compile_time_arguments {
				let evaluated = argument.evaluate_at_compile_time(context)?;
				compile_time_arguments.push(evaluated);
			}
			compile_time_arguments
		};

		// Arguments
		let arguments = {
			let mut arguments = Vec::new();
			for argument in self.arguments {
				let evaluated = argument.evaluate_at_compile_time(context)?;
				arguments.push(evaluated);
			}
			arguments
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
		if let Some(return_type) = function.return_type() {
			return_type.expect_as::<VirtualPointer>().cloned()
		} else {
			context.scope_data.get_variable("Nothing").unwrap().expect_as::<VirtualPointer>().cloned()
		}
	}
}

impl Spanned for FunctionCall {
	fn span(&self, _context: &Context) -> Span {
		self.span.clone()
	}
}

impl FunctionCall {
	/// Converts a binary operation expression into a function call. In Cabin, binary operations are just function calls, so the expression:
	///
	/// ```
	/// first + second
	/// ```
	///
	/// is equivalent to:
	///
	/// ```
	/// first.plus(second)
	/// ```
	///
	/// So, this function converts from the first form of that into the second. This is used by `operators::parse_binary_expression()` at
	/// parse-time to convert parsed binary expressions into function calls.
	///
	/// # Parameters
	///
	/// - `left` - The expression on the left of the binary expression
	/// - `right` - The expression on the right of the binary expression
	/// - `operation` - The token of the operation symbol
	/// - `context` - Global data about the compiler's state
	///
	/// # Returns
	///
	/// The function call object created from the binary expression.
	///
	/// # Errors
	///
	/// Only if the given token does not represent a valid binary operation. The given token must have a type of
	/// `TokenType::Plus`, `TokenType::Minus`, etc.
	pub fn from_binary_operation(left: Expression, right: Expression, operation: Token, context: &Context) -> anyhow::Result<FunctionCall> {
		let function_name = match operation.token_type {
			TokenType::Asterisk => "times",
			TokenType::DoubleEquals => "equals",
			TokenType::ForwardSlash => "divided_by",
			TokenType::LessThan => "is_less_than",
			TokenType::GreaterThan => "is_greater_than",
			TokenType::Minus => "minus",
			TokenType::Plus => "plus",
			_ => bail_err! {
				base = format!("Attempted to convert a binary operation into a function call, but no function name exists for the operation \"{}\"", format!("{}", operation.token_type).bold().cyan()),
				while = "getting the function name for a binary operation",
				context = context,
			},
		};

		let start = left.span(context);
		let middle = operation.span;
		let end = right.span(context);

		Ok(FunctionCall {
			function: Box::new(Expression::FieldAccess(FieldAccess::new(
				left,
				Name::from(function_name),
				context.scope_data.unique_id(),
				start.to(&middle),
			))),
			arguments: vec![right],
			compile_time_arguments: Vec::new(),
			scope_id: context.scope_data.unique_id(),
			span: start.to(&end),
		})
	}
}
