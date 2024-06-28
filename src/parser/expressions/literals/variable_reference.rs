use crate::{
	cli::theme::Styled,
	compile_time::{CompileTime, TranspileToC},
	context::Context,
	formatter::{ColoredCabin, ToCabin},
	lexer::TokenType,
	parser::{
		expressions::{
			literals::{Literal, LiteralValue},
			run::ParentExpression,
			util::{name::Name, types::Typed},
			Expression,
		},
		Parse, TokenQueue,
	},
};

use colored::Colorize as _;

/// An identifier that references a variable.
#[derive(Debug, Clone)]
pub struct VariableReference {
	/// The name of the variable being referenced.
	name: Name,
	/// The ID of the scope that the variable reference is declared.
	referenced_scope_id: usize,
	/// The line in the source code that the variable reference is declared.
	line: usize,
	/// The column in the source code that the variable reference is declared.
	_column: usize,
}

impl VariableReference {
	/// Creates a new variable reference with undefined line and column numbers.
	///
	/// # Parameters
	/// - `name` - The name of the variable being referenced
	/// - `referenced_scope_id` - The ID that the variable reference is declared in.
	///
	/// # Returns
	/// The newly created variable referenced.
	pub const fn new(name: Name, referenced_scope_id: usize) -> Self {
		Self {
			name,
			referenced_scope_id,
			line: 1,
			_column: 1,
		}
	}

	/// Creates a new variable reference with the specified position in the source code.
	///
	/// # Parameters
	/// - `name` - The name of the variable being referenced
	/// - `referenced_scope_id` - The scope ID which the variable is being referenced in
	/// - `line` - The line in the source code that the variable reference occurs
	/// - `column` - The column in the source code that the variable reference occurs
	///
	/// # Returns
	/// The newly created variable reference.
	pub const fn with_position(name: Name, referenced_scope_id: usize, line: usize, column: usize) -> Self {
		Self {
			name,
			referenced_scope_id,
			line,
			_column: column,
		}
	}

	/// Returns the name of this variable reference.
	///
	/// # Parameters
	/// - `name` - The name of the variable being referenced.
	pub const fn name(&self) -> &Name {
		&self.name
	}

	/// Returns the scope ID that this variable reference is declared in.
	///
	/// # Returns
	/// The scope ID that this variable reference is declared in.
	pub const fn scope_id(&self) -> usize {
		self.referenced_scope_id
	}

	/// Returns the value that this variable reference points to, or itself if the variable is pointing to a function parameter.
	///
	/// # Parameters
	/// - `context` - Global data about the compiler, including scope data that's used to get the value of the variable reference based on its name.
	///
	/// # Returns
	/// The value that this variable reference points to, or itself if the variable is pointing to a function parameter.
	pub fn value(&self, context: &mut Context) -> anyhow::Result<Expression> {
		if context.parameter_names.iter().any(|(name, _)| name == &self.name) || context.generics_stack.last().is_some_and(|last| last.contains(&self.name)) {
			return Ok(Expression::Literal(Literal::new(LiteralValue::VariableReference(self.clone()))));
		}

		Ok(context
			.scope_data
			.get_variable_from_id(self.name(), context.scope_data.unique_id())
			.cloned()
			.ok_or_else(|| {
				context.current_bad_identifier = Some(self.name().clone());
				let program = context.colored_program();
				context.add_error_details(format!(
					"In this part of the program, you refer to a variable \"{}\", but no variable with that name exists here:\n\n{}:\n\n{}\n\n{}",
					self.name().cabin_name().bold().cyan(),
					format!("In {} (line {})", context.file_name.bold().white(), self.line),
					format!("\n{}\n",
						program
							.lines()
							.enumerate() // Wee add +1 to line number in these next 2 calls because lines() is 0 indexed but our line numbers are 1 indexed
							.filter(|(line_number, _line)| (isize::try_from(*line_number).unwrap() + 1 - isize::try_from(self.line).unwrap()).abs() < 3)
							.map(|(line_number, line)| format!("    {line_number}    {line}", 
								line_number = if line_number + 1 == self.line {
									(line_number + 1).to_string().bold().red()
								} else {
									(line_number + 1).to_string().style(context.theme().line_numbers())
								},
								line = if (line_number as isize) + 1 == (self.line as isize) - 1 {
									let spacing = " ".repeat(self.line - 1 + format!("    {line_number}    ").len());
									let arrows = "v".repeat(self.name().cabin_name().len());
									format!("{}", format!("{line}\n{spacing}{arrows}  the error is with this variable reference").style(context.theme().line_numbers()))
								} else {
									line.to_owned()
								}
							))
							.collect::<Vec<_>>()
							.join("\n")
					).style(context.theme().background()),
					format!(
						"Here you reference a variable \"{name}\", but no variable with the name \"{name}\" exists at this part of the program.\nIf this is a typo and you don't expect a variable with this name to exist, you could be attempting to refer to one\nof these variables, which are the variables with the closest names that exist at this point in the program:\n{}",
						context
							.scope_data
							.get_closest_variables(self.name(), 3)
							.into_iter()
							.map(|(name, _variable)| format!("    - {}", name.cabin_name().cyan().bold()))
							.collect::<Vec<_>>()
							.join("\n"),
						name = self.name().cabin_name().bold().cyan()
					)
				));

				anyhow::anyhow!("You refer to a variable with the name \"{name}\", but no variable with the name \"{name}\" exists\nat the part of the program that you refer to it.\n", name = self.name().cabin_name().bold().cyan())
			})?
			.value
			.unwrap())
	}
}

impl Parse for VariableReference {
	type Output = Self;
	fn parse(tokens: &mut std::collections::VecDeque<crate::lexer::Token>, context: &mut Context) -> anyhow::Result<Self::Output> {
		let line_number = tokens.current_line();
		let column_number = tokens.current_column();
		let identifier_name = tokens.pop(TokenType::Identifier, context)?;
		Ok(Self::with_position(Name(identifier_name), context.scope_data.unique_id(), line_number, column_number))
	}
}

impl CompileTime for VariableReference {
	fn compile_time_evaluate(&self, context: &mut Context, _with_side_effects: bool) -> anyhow::Result<Expression> {
		Ok(Expression::Literal(Literal::new(LiteralValue::VariableReference(self.clone()))))
	}
}

impl ParentExpression for VariableReference {
	fn evaluate_children_at_compile_time(&self, context: &mut Context) -> anyhow::Result<Expression> {
		self.value(context)?.evaluate_children_at_compile_time(context)
	}
}

impl Typed for VariableReference {
	fn get_type(&self, context: &mut Context) -> anyhow::Result<Literal> {
		if let Some(variable) = context.parameter_names.iter().find(|(name, _)| name == &self.name) {
			return Ok(variable.1.clone());
		}

		let identifier_variable = context
			.scope_data
			.get_variable_from_id(self.name(), self.scope_id())
			.ok_or_else(|| {
				anyhow::anyhow!(
					"Error getting the type of identifier \"{name}\": The variable \"{name}\" does not exist in this scope\n",
					name = self.name().cabin_name()
				)
			})?
			.clone();

		Ok(if let Some(type_annotation) = identifier_variable.type_annotation {
			let Expression::Literal(Literal(LiteralValue::VariableReference(_variable_reference), ..)) = type_annotation else {
				anyhow::bail!("Type of object is not an identifier");
			};
			// Type::Group(variable_reference.name().to_owned(), variable_reference.scope_id())
			todo!()
		} else {
			identifier_variable.value.as_ref().unwrap().get_type(context)?
		})
	}
}

impl TranspileToC for VariableReference {
	fn c_prelude(&self, _context: &mut Context) -> anyhow::Result<String> {
		Ok(String::new())
	}

	fn to_c(&self, _context: &mut Context) -> anyhow::Result<String> {
		Ok(self.name().c_name())
	}
}

impl ToCabin for VariableReference {
	fn to_cabin(&self) -> String {
		self.name().cabin_name()
	}
}

impl ColoredCabin for VariableReference {
	fn to_colored_cabin(&self, context: &mut Context) -> String {
		self.name().to_colored_cabin(context)
	}
}
