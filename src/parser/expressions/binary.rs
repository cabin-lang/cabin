use crate::{
	compile_time::{CompileTime, TranspileToC},
	context::Context,
	formatter::{ColoredCabin, ToCabin},
	lexer::{Token, TokenType},
	parser::{
		expressions::{
			function_call::FunctionCall,
			literals::{variable_reference::VariableReference, Literal, LiteralValue},
			run::ParentExpression,
			util::{name::Name, types::Typed},
			Expression,
		},
		Parse, TokenQueue,
	},
};

use std::collections::VecDeque;

use colored::Colorize as _;

/// A binary operation. More specifically, this represents not one operation, but a group of operations that share the same precedence.
/// For example, the `+` and `-` operators share the same precedence, so they are grouped together in the `ADDITIVE` constant.
///
/// # Parameters
/// `<'this>` - The lifetime of this operation, to ensure that the contained reference to the precedent operation lives at least that long.
struct BinaryOperation<'this> {
	/// The operation that has the next highest precedence, or `None` if this operation has the highest precedence.
	precedent: Option<&'this BinaryOperation<'this>>,
	/// The token types that represent this operation, used to parse a binary expression.
	token_types: &'this [TokenType],
}

// TODO: make this right-associative
/// The exponentiation operation, which has the highest precedence. This covers the `^` operator.
static EXPONENTIATION: BinaryOperation<'static> = BinaryOperation {
	precedent: None,
	token_types: &[TokenType::Caret],
};

// TODO: Add modulo
/// The multiplicative operations, which have the second highest precedence. This covers the `*` and `/` operators.
static MULTIPLICATIVE: BinaryOperation<'static> = BinaryOperation {
	precedent: Some(&EXPONENTIATION),
	token_types: &[TokenType::Asterisk, TokenType::ForwardSlash],
};

/// The additive operations, which have the third precedence. This covers the `+` and `-` operators.
static ADDITIVE: BinaryOperation<'static> = BinaryOperation {
	precedent: Some(&MULTIPLICATIVE),
	token_types: &[TokenType::Plus, TokenType::Minus],
};

/// The comparison operations, such as "==", "<=", etc.
static COMPARISON: BinaryOperation<'static> = BinaryOperation {
	precedent: Some(&ADDITIVE),
	token_types: &[TokenType::DoubleEquals, TokenType::LessThan, TokenType::GreaterThan],
};

/// The assignment operations, which have the lowest precedence. This covers the `=` operator.
static ASSIGNMENT: BinaryOperation<'static> = BinaryOperation {
	precedent: Some(&COMPARISON),
	token_types: &[TokenType::Equal],
};

impl BinaryOperation<'_> {
	/// Parses the precedent operation of this one if it exists, otherwise, parses a function call (which has higher precedence than any binary operation)
	///
	/// # Parameters
	/// - `tokens` - The token stream to parse
	/// - `current_scope` - The current scope
	/// - `debug_info` - The debug information
	fn parse_precedent(&self, tokens: &mut VecDeque<Token>, context: &mut Context) -> anyhow::Result<Expression> {
		if let Some(precedent) = self.precedent {
			parse_binary_expression(precedent, tokens, context)
		} else {
			FunctionCall::parse(tokens, context)
		}
	}
}

/// A binary expression node in the abstract syntax tree. This represents an operation that takes two operands in infix notation.
#[derive(Clone, Debug)]
pub struct BinaryExpression {
	/// The left operand of the operation
	pub left: Expression,
	/// The operator of the operation
	pub operator: TokenType,
	/// The right operand of the operation
	pub right: Expression,
}

/// Parses a binary expression with the given operation.
///
/// # Parameters
/// - `operation` - The operation to parse
/// - `tokens` - The token stream to parse
/// - `context` - Global data about the compiler's state
///
/// # Returns
/// A `Result` containing either the parsed expression or an `Error`.
fn parse_binary_expression(operation: &BinaryOperation<'_>, tokens: &mut VecDeque<Token>, context: &mut Context) -> anyhow::Result<Expression> {
	let mut expression = operation.parse_precedent(tokens, context)?;
	while tokens.next_is_one_of(operation.token_types) {
		let operator = tokens.pop_type(tokens.peek().unwrap().token_type.clone()).unwrap_or_else(|_error| unreachable!());
		let right = operation.parse_precedent(tokens, context)?;
		expression = Expression::BinaryExpression(Box::new(BinaryExpression {
			left: expression,
			operator,
			right,
		}));
	}

	Ok(expression)
}

impl Parse for BinaryExpression {
	type Output = Expression;

	fn parse(tokens: &mut VecDeque<Token>, context: &mut Context) -> anyhow::Result<Self::Output> {
		parse_binary_expression(if context.is_parsing_type { &COMPARISON } else { &ASSIGNMENT }, tokens, context)
	}
}

impl Typed for BinaryExpression {
	fn get_type(&self, context: &mut Context) -> anyhow::Result<Literal> {
		let left_type = self
			.left
			.get_type(context)
			.map_err(|error| anyhow::anyhow!("{error}\n\twhile getting the type of the left side of a binary expression"))?;

		if self.operator == TokenType::Dot {
			let Expression::Literal(Literal(LiteralValue::VariableReference(right_identifier, ..), ..)) = &self.right else {
				anyhow::bail!(
					"The right hand side of a \"{}\" operation must be an identifier\n\twhile getting the type of a binary expression",
					".".cyan().bold()
				);
			};

			let Literal(LiteralValue::Group(group), ..) = left_type else {
				anyhow::bail!("Attempted to access a field on an object who's type is not a group.")
			};

			return group
				.fields
				.iter()
				.find(|field| &field.name == right_identifier.name())
				.ok_or_else(|| {
					anyhow::anyhow!(
						"Attempted to access the field \"{}\" on an object, but no field with that name exists for objects of that type.\n\twhile getting the type of a binary expression",
						right_identifier.name().cabin_name().cyan().bold()
					)
				})?
				.value
				.as_ref()
				.unwrap()
				.get_type(context);
		}

		anyhow::bail!("TODO: Binary operation types");
	}
}

impl CompileTime for BinaryExpression {
	fn compile_time_evaluate(&self, context: &mut Context, with_side_effects: bool) -> anyhow::Result<Expression> {
		let left = self.left.compile_time_evaluate(context, with_side_effects).map_err(|error| {
			anyhow::anyhow!(
				"{error}\n\t{}",
				format!(
					"while evaluating the left-hand side of the binary expression \"{}\" at compile-time",
					format!("{}", self.operator).bold().cyan()
				)
				.dimmed()
			)
		})?;

		// The left must be a literal (be it an object literal or an identifier) to perform this expression at compile-time
		let Expression::Literal(mut left_literal) = left else {
			let right = self.right.compile_time_evaluate(context, with_side_effects).map_err(|error| {
				anyhow::anyhow!(
					"{error}\n\t{}",
					format!(
						"while evaluating the right-hand side of the binary expression \"{}\" at compile-time",
						format!("{}", self.operator).bold().cyan()
					)
					.dimmed()
				)
			})?;

			return Ok(Expression::BinaryExpression(Box::new(Self {
				left,
				right,
				operator: self.operator.clone(),
			})));
		};

		// Struct field access
		if self.operator == TokenType::Dot {
			let Expression::Literal(Literal(LiteralValue::VariableReference(right_variable_reference), ..)) = &self.right else {
				anyhow::bail!("The binary operator \".\" must be followed by a variable name on the right side\n\twhile evaluating the right side of a field access expression at compile-time");
			};

			return Ok(match left_literal.value() {
				LiteralValue::Object(left_table) => {
					let Some(field) = left_table.get_field(right_variable_reference.name()) else {
						anyhow::bail!("Attempted to access the field \"{field}\" on an object of type \"{object_type}\", but no field\nwith the name \"{field}\" exists on objects of the type \"{object_type}\". {fields}.\n\twhile evaluating the binary expression \"{}\" at compile-time",
							".".bold().cyan(),
							field = right_variable_reference.name().cabin_name().bold().cyan(),
							object_type = left_table.name.cabin_name().bold().cyan(),
							fields = if left_table.fields.is_empty() {
								format!("Objects of type \"{object_type}\" {} because the group \"{object_type}\"\nis declared as an empty group, and no {} declarations exist for \"{object_type}\" that declare new fields on it",
									"have no fields".bold().white(),
									"represent-as".bold().white(),
									object_type = left_table.name.cabin_name().bold().cyan(),
								)
							} else {
								format!("The available fields on objects of type {} are [{}]",
									left_table.name.cabin_name().bold().cyan(),
									left_table.fields.iter().map(|field| field.name.cabin_name()).collect::<Vec<_>>().join(", ")
								)
							}
						);
					};

					field.clone()
				},
				LiteralValue::VariableReference(left_variable_reference) => {
					let left_value = &context
						.scope_data
						.get_variable_from_id(left_variable_reference.name(), left_variable_reference.scope_id())
						.ok_or_else(|| anyhow::anyhow!("Error getting the value of a variable"))?
						.value
						.clone()
						.unwrap();

					match left_value {
						Expression::Literal(Literal(LiteralValue::Object(object), ..)) => {
							let Some(field) = object.get_field(right_variable_reference.name()) else {
								anyhow::bail!("Attempted to access the field \"{field}\" on an object of type \"{object_type}\", but no field with the name \"{field}\" exists on objects of the type \"{object_type}\". Fields available are {fields}",
									field = left_variable_reference.name().cabin_name().bold().cyan(),
									object_type = object.name.cabin_name().bold().cyan(),
									fields = object.fields.iter().map(|field| field.name.cabin_name()).collect::<Vec<_>>().join(", ")
								);
							};

							field.clone()
						},
						Expression::Literal(Literal(LiteralValue::Group(group), ..)) => {
							let Some(field) = group.fields.iter().find(|field| &field.name == right_variable_reference.name()) else {
								anyhow::bail!(
									"Attempted to access the field \"{field}\" on a group, but no field with the name \"{field}\" exists.",
									field = left_variable_reference.name().cabin_name().bold().cyan(),
								);
							};

							field.value.clone().unwrap()
						},

						Expression::Literal(Literal(LiteralValue::Either(either), ..)) => {
							let Some(field) = either.variants().iter().find(|field| &field.0 == right_variable_reference.name()) else {
								anyhow::bail!(
									"Attempted to access the field \"{field}\" on a group, but no field with the name \"{field}\" exists.",
									field = left_variable_reference.name().cabin_name().bold().cyan(),
								);
							};

							field.1.clone()
						},

						Expression::Literal(literal) => {
							if literal.is(&context.unknown_at_compile_time().clone(), context)? {
								return Ok(Expression::BinaryExpression(Box::new(Self {
									left: Expression::Literal(Literal::new(LiteralValue::VariableReference(left_variable_reference.clone()))),
									right: self.right.clone(),
									operator: self.operator.clone(),
								})));
							}

							// Uniques don't have fields
							anyhow::bail!(
								"Attempted to access the field \"{}\" on a unique value, and unique values don't have fields.",
								left_variable_reference.name().cabin_name().bold().cyan()
							)
						},

						// If the left is any other expression, like a binary expression or a function call, that means that
						// it couldn't be fully evaluated into an object at compile-time. In this case, we just return the
						// binary expression
						_ => {
							return Ok(Expression::BinaryExpression(Box::new(Self {
								left: Expression::Literal(Literal::new(LiteralValue::VariableReference(left_variable_reference.clone()))),
								right: self.right.clone(),
								operator: self.operator.clone(),
							})))
						},
					}
				},
				_ => anyhow::bail!("Attempted to access a field on a {:?}", left_literal),
			});
		}

		// TODO: better compile-time evaluation here
		if self.operator == TokenType::Equal {
			let mut right = self.right.compile_time_evaluate(context, with_side_effects)?;

			if let Expression::Literal(Literal(LiteralValue::VariableReference(variable_reference), ..)) = &right {
				right = context.scope_data.get_variable(variable_reference.name()).unwrap().value.clone().unwrap();
			}

			let Literal(LiteralValue::VariableReference(variable_reference), ..) = &left_literal else {
				anyhow::bail!("Attempted to assign to non-identifier value");
			};
			if let Expression::Literal(right_literal) = &right {
				if !right_literal.is(&context.unknown_at_compile_time().clone(), context)? {
					context.scope_data.reassign_variable(variable_reference.name(), right)?;
				}
			} else {
				context.scope_data.reassign_variable(variable_reference.name(), right)?;
			}
			return Ok(Expression::BinaryExpression(Box::new(self.clone())));
		}

		// Other operators
		let operator = Name(
			match self.operator {
				TokenType::Plus => "plus",
				TokenType::Minus => "minus",
				TokenType::Asterisk => "times",
				TokenType::ForwardSlash => "divided_by",
				TokenType::DoubleEquals => "equals",
				_ => anyhow::bail!(
					"The binary operator \"{operator}\" is not yet supported\n\twhile converting the binary operation \"{operator}\" to C code",
					operator = format!("{:?}", self.operator).bold().cyan()
				),
			}
			.to_owned(),
		);

		let right = self.right.compile_time_evaluate(context, with_side_effects).map_err(|error| {
			anyhow::anyhow!(
				"{error}\n\twhile evaluating the right-hand side of the binary expression \"{}\" at compile-time",
				format!("{}", self.operator).bold().cyan()
			)
		})?;

		if let Literal(LiteralValue::VariableReference(variable_reference), ..) = left_literal {
			let left_value = &context
				.scope_data
				.get_variable_from_id(variable_reference.name(), variable_reference.scope_id())
				.ok_or_else(|| anyhow::anyhow!(""))?
				.value
				.as_ref()
				.unwrap()
				.clone();

			left_literal = match left_value {
				// If the left is an object, our literal is that object
				Expression::Literal(Literal(LiteralValue::Object(object), ..)) => Literal::new(LiteralValue::Object(object.clone())),

				// If its a parameter, we just return the binary expression
				Expression::Literal(literal) => {
					if literal.is(&context.unknown_at_compile_time().clone(), context)? {
						return Ok(Expression::BinaryExpression(Box::new(Self {
							left: Expression::Literal(Literal::new(LiteralValue::VariableReference(variable_reference))),
							right,
							operator: self.operator.clone(),
						})));
					}

					anyhow::bail!("invalid literal on the left side of a binary expression: {literal:?}")
				},

				// If the left is any other expression, like a binary expression or a function call, that means that
				_ => {
					return Ok(Expression::BinaryExpression(Box::new(Self {
						left: Expression::Literal(Literal::new(LiteralValue::VariableReference(variable_reference))),
						right,
						operator: self.operator.clone(),
					})));
				},
			};
		}

		let Literal(LiteralValue::Object(left_table), ..) = left_literal else { unreachable!() };

		// Perform the operation
		if let Some(value) = left_table.get_field(&operator) {
			let Expression::Literal(Literal(LiteralValue::FunctionDeclaration(_), ..)) = value else {
				anyhow::bail!(
					"Attempted to call the binary operation \"{field}\" on an object, and while the field \"{field}\" exists on the object, it is not a function.",
					field = operator.cabin_name().bold().cyan()
				);
			};

			return FunctionCall {
				function: value.clone(),
				arguments: vec![Expression::Literal(Literal::new(LiteralValue::Object(left_table))), right],
				has_been_converted_to_block: true,
			}
			.compile_time_evaluate(context, with_side_effects)
			.map_err(|error| {
				anyhow::anyhow!(
					"{error}\n\twhile evaluating the binary expression \"{}\" at compile-time",
					operator.cabin_name().bold().cyan()
				)
			});
		}

		anyhow::bail!(
			"Attempted to access \"{access}\" on an object, but no field with the name \"{}\" exists on the object\n\twhile evaluating the binary access expression \"{access}\" at compile-time",
			operator.cabin_name().bold().cyan(),
			access = format!(".{}", operator.cabin_name()).bold().cyan()
		);
	}
}

impl ParentExpression for BinaryExpression {
	fn evaluate_children_at_compile_time(&self, context: &mut Context) -> anyhow::Result<Expression> {
		let left = self.left.compile_time_evaluate(context, true).map_err(|error| {
			anyhow::anyhow!(
				"{error}\n\t{}",
				format!("while evaluating the left-hand side of the binary expression {operator} at compile-time\n\twhile evaluating the sub-expressions of the binary expression {operator} at compile-time", operator = self.operator).dimmed()
			)
		})?;
		let right = self.right.compile_time_evaluate(context, true).map_err(|error| {
			anyhow::anyhow!(
				"{error}\n\t{}",
				format!("while evaluating the right-hand side of the binary expression {operator} at compile-time\n\twhile evaluating the sub-expressions of the binary expression {operator} at compile-time", operator = self.operator).dimmed()
			)
		})?;

		Ok(Expression::BinaryExpression(Box::new(Self {
			left,
			right,
			operator: self.operator.clone(),
		})))
	}
}

impl TranspileToC for BinaryExpression {
	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		let left = self.left.to_c(context)?;
		let right = self.right.to_c(context)?;

		if self.operator == TokenType::DoubleEquals {
			return Ok(format!("({{ Number_u* this = ((Number_u*) {left}); this->equals_u(this, &{right}); }})"));
		}

		if self.operator == TokenType::Dot {
			return Ok(format!("{left}->{right}"));
		}

		if self.operator == TokenType::Equal {
			return Ok(format!("*{left} = {right}"));
		}

		let operator = match self.operator {
			TokenType::Plus => "plus",
			TokenType::Minus => "minus",
			TokenType::Asterisk => "times",
			TokenType::ForwardSlash => "divided_by",
			TokenType::DoubleEquals => "equals",
			_ => anyhow::bail!(
				"The binary operator \"{operator}\" is not yet supported\n\twhile converting the binary operation \"{operator}\" to C code",
				operator = format!("{:?}", self.operator).bold().cyan()
			),
		};

		Ok(format!("CALL(({left}), {}, &{right})", Name(operator.to_owned()).c_name()))
	}

	fn c_prelude(&self, context: &mut Context) -> anyhow::Result<String> {
		let left = self.left.c_prelude(context)?;
		let right = self.right.c_prelude(context)?;
		Ok([left, right].join("\n"))
	}
}

impl ToCabin for BinaryExpression {
	fn to_cabin(&self) -> String {
		let operator = match self.operator {
			TokenType::Plus => " + ",
			TokenType::Minus => " - ",
			TokenType::Asterisk => " * ",
			TokenType::ForwardSlash => " / ",
			TokenType::Dot => ".",
			TokenType::Equal => " = ",
			_ => panic!("Unsupported operator {:?}", self.operator),
		};

		format!("{}{}{}", self.left.to_cabin(), operator, self.right.to_cabin())
	}
}

impl ColoredCabin for BinaryExpression {
	fn to_colored_cabin(&self, context: &mut Context) -> String {
		let operator = match self.operator {
			TokenType::Plus => " + ",
			TokenType::Minus => " - ",
			TokenType::Asterisk => " * ",
			TokenType::ForwardSlash => " / ",
			TokenType::Dot => ".",
			TokenType::DoubleEquals => " == ",
			_ => panic!("Unsupported operator {:?}", self.operator),
		};

		format!("{}{}{}", self.left.to_colored_cabin(context), operator, self.right.to_colored_cabin(context))
	}
}

/// A placeholder struct for parsing access expressions, which is a binary expression with a "." (dot) operator. This is different from other binary expressions
/// because the right hand side is always an identifier, so it's in its own struct.
pub struct AccessExpression;
impl Parse for AccessExpression {
	type Output = Expression;

	fn parse(tokens: &mut VecDeque<Token>, context: &mut Context) -> anyhow::Result<Self::Output> {
		let mut expression = Expression::Literal(Literal::new(LiteralValue::parse(tokens, context)?)); // There should be no map_err here
		while tokens.next_is(TokenType::Dot) {
			tokens.pop(TokenType::Dot, context)?;
			let current_line = tokens.current_line();
			let current_column = tokens.current_column();
			let right = Expression::Literal(Literal::new(LiteralValue::VariableReference(VariableReference::with_position(
				Name(tokens.pop(TokenType::Identifier, context)?),
				context.scope_data.unique_id(),
				current_line,
				current_column,
			))));
			expression = Expression::BinaryExpression(Box::new(BinaryExpression {
				left: expression,
				operator: TokenType::Dot,
				right,
			}));
		}

		Ok(expression)
	}
}
