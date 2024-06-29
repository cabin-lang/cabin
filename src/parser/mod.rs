use crate::{
	compile_time::{builtin::IS_FIRST_PRINT, CompileTimeStatement, TranspileToC},
	context::{Context, Severity, TokenError},
	formatter::{ColoredCabin, ToCabin},
	lexer::{Token, TokenType},
	parser::{
		expressions::{literals::group::GroupType, util::name::Name, Expression},
		statements::Statement,
	},
};

use colored::Colorize as _;
use expressions::literals::function_declaration;

// Brings the `write!()` and `writeln!()` macros into scope, which allows appending to a string. This is more efficient than using
// `string = format!("{string}...")`, because it avoids an extra allocation. We have a clippy warning turned on for this very
// purpose. We assign this to `_` to indicate clearly that it's just a trait and not used explicitly anywhere outside of bringing its
// methods into scope.
use std::{fmt::Write as _, sync::atomic::Ordering};

use self::expressions::literals::{Literal, LiteralValue};

/// The expressions module, which handles AST nodes that represent expressions.
pub mod expressions;
/// The statements module, which handles AST nodes that represent statements.
pub mod statements;

/// An abstract syntax tree of an entire program.
#[derive(Debug, Clone)]
pub struct Program {
	/// The statements that make up the program.
	pub statements: Vec<Statement>,
}

impl Parse for Program {
	type Output = Self;

	fn parse(tokens: &mut std::collections::VecDeque<Token>, context: &mut Context) -> anyhow::Result<Self::Output> {
		let mut statements = Vec::new();
		while !tokens.is_empty() {
			statements.push(Statement::parse(tokens, context).map_err(|error| anyhow::anyhow!("{error}\n\twhile attempting to parse the program's global declarations"))?);
		}
		let program = Self { statements };
		context.program = Some(program.clone());
		Ok(program)
	}
}

impl Program {
	/// Converts this program, as an AST, into a "compile-time evaluated" AST program. This loops over every statement and expression in the program and evaluates
	/// it at compile-time if possible. This should be used before transpiling to C.
	///
	/// # Parameters
	/// - `context` - Global data about the program, such as the current scope and its variables. This is passed recursively into every expressions' `compile_time_evaluate`
	/// functions.
	///
	/// # Returns
	/// A new program struct that's been evaluated at compile-time, or an error if there was an error evaluating compile-time code.
	pub fn compile_time_evaluate(&self, context: &mut Context, with_side_effects: bool) -> anyhow::Result<Self> {
		let program = Self {
			statements: self
				.statements
				.iter()
				.map(|statement| statement.compile_time_evaluate_statement(context, with_side_effects))
				.collect::<anyhow::Result<Vec<Statement>>>()
				.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating the program's global variables at compile-time".dimmed()))?,
		};

		if !IS_FIRST_PRINT.load(Ordering::Relaxed) {
			println!();
		}

		Ok(program)
	}
}

impl TranspileToC for Program {
	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		let mut c = "int main(int argc, char** argv) {\n".to_owned();

		let declarations = self
			.statements
			.iter()
			.enumerate()
			.filter_map(|(index, statement)| {
				if let Statement::Declaration(declaration) = statement {
					Some((declaration, index))
				} else {
					None
				}
			})
			.collect::<Vec<_>>();

		let mut declared_variables = Vec::new();

		// For each variable declaration in the global scope,
		for (declaration, _index) in &declarations {
			if declaration.name == Name("return_address".to_owned()) {
				continue;
			}
			// Add the declaration itself
			if !declared_variables.contains(&declaration.name) {
				let value = context
					.scope_data
					.get_scope_from_id(declaration.declared_scope_id)
					.ok_or_else(|| anyhow::anyhow!("Expected scope to exist for declaration"))?
					.get_variable_direct(&declaration.name)
					.cloned()
					.ok_or_else(|| anyhow::anyhow!("Variable {} not found", declaration.name.cabin_name()))?
					.value
					.unwrap();

				if let Expression::Literal(Literal(LiteralValue::Group(_) | LiteralValue::FunctionDeclaration(_) | LiteralValue::Either(_), ..)) = value {
					continue;
				}

				// C itself
				declaration
					.to_c(context)?
					.lines()
					.map(|line| Ok(writeln!(c, "\t{line}")?))
					.collect::<anyhow::Result<Vec<_>>>()?;
				declared_variables.push(declaration.name.clone());
			}
		}

		let done_indices = declarations.iter().map(|(_name, index)| *index).collect::<Vec<_>>();

		// Transpile the statements
		for (index, statement) in self.statements.iter().enumerate() {
			if !done_indices.contains(&index) {
				statement
					.to_c(context)?
					.lines()
					.map(|line| Ok(writeln!(c, "\t{line}")?))
					.collect::<anyhow::Result<Vec<_>>>()?;
			}
		}
		if let Some(main_function) = &context.main_function_name {
			writeln!(c, "{main_function}();").unwrap();
		}
		c.push('}');
		c = regex_macro::regex!("(\\s*\r?\n){3,}").replace_all(&c, "\n\n").to_string();

		Ok(c)
	}

	fn c_prelude(&self, context: &mut Context) -> anyhow::Result<String> {
		let mut prelude = String::new();

		let declarations = self
			.statements
			.iter()
			.enumerate()
			.filter_map(|(index, statement)| {
				if let Statement::Declaration(declaration) = statement {
					Some((declaration, index))
				} else {
					None
				}
			})
			.collect::<Vec<_>>();

		let mut declared_variables = Vec::new();

		// For each variable declaration in the global scope,
		for (declaration, _index) in &declarations {
			// Add the declaration itself
			if !declared_variables.contains(&declaration.name) {
				let value = context
					.scope_data
					.get_scope_from_id(declaration.declared_scope_id)
					.ok_or_else(|| anyhow::anyhow!("Expected scope to exist for declaration"))?
					.get_variable_direct(&declaration.name)
					.cloned()
					.ok_or_else(|| anyhow::anyhow!("Variable {} not found", declaration.name.cabin_name()))?
					.value
					.unwrap();

				match value {
					Expression::Literal(Literal(LiteralValue::Group(_) | LiteralValue::FunctionDeclaration(_) | LiteralValue::Either(_), ..)) => {},
					_ => continue,
				}

				if let Expression::Literal(Literal(LiteralValue::Group(..), ..)) = value {
					declaration
						.c_prelude(context)?
						.lines()
						.map(|line| Ok(writeln!(prelude, "{line}")?))
						.collect::<anyhow::Result<Vec<_>>>()?;
				}

				declared_variables.push(declaration.name.clone());
			}
		}

		#[allow(clippy::filter_map_identity)] // This is much clearer with `filter_map()`; IMHO using `flatten()` is much more confusing here
		let done_indices = declarations
			.iter()
			.map(|(declaration, index)| {
				let value = context
					.scope_data
					.get_scope_from_id(declaration.declared_scope_id)
					.ok_or_else(|| anyhow::anyhow!("Expected scope to exist for declaration"))?
					.get_variable_direct(&declaration.name)
					.cloned()
					.ok_or_else(|| anyhow::anyhow!("Variable {} not found", declaration.name.cabin_name()))?
					.value
					.unwrap();

				Ok(match value {
					Expression::Literal(Literal(LiteralValue::Group(_) | LiteralValue::FunctionDeclaration(_) | LiteralValue::Either(_), ..)) => Some(*index),
					_ => None,
				})
			})
			.collect::<anyhow::Result<Vec<_>>>()?
			.into_iter()
			.filter_map(|maybe_index| maybe_index)
			.collect::<Vec<_>>();

		// Transpile the statements
		for (index, statement) in self.statements.iter().enumerate() {
			if !done_indices.contains(&index) {
				if let Statement::Declaration(declaration) = statement {
					if let Expression::Literal(Literal(LiteralValue::Group(..), ..)) = &declaration.initial_value {
						statement
							.to_c(context)?
							.lines()
							.map(|line| Ok(writeln!(prelude, "{line}")?))
							.collect::<anyhow::Result<Vec<_>>>()?;
					}
				}
			}
		}

		let mut forward_declarations = Vec::new();

		for (group, group_type) in &context.groups {
			forward_declarations.push(format!(
				"typedef {} {group} {group};",
				match group_type {
					GroupType::Group => "struct",
					GroupType::Either => "enum",
				},
			));
		}

		for function in context.function_declarations.clone() {
			forward_declarations.push(format!(
				"void {name}({parameters});",
				name = format!("{}_{}", function.name.as_ref().unwrap(), function.id),
				parameters = function
					.parameters
					.iter()
					.map(|parameter| Ok(format!("{}* {}", parameter.1.to_c(context)?, parameter.0.cabin_name())))
					.collect::<anyhow::Result<Vec<_>>>()?
					.join(", ")
			));

			prelude.push_str(&function.c_prelude(context)?);
		}

		prelude = format!(
			"{}\n\n{forward_declarations}\n\n{prelude}",
			unindent::unindent(
				"
				#include <stdio.h>
				#include <stdlib.h>
				#include <sys/stat.h>
				#include <sys/types.h>

				static void* this = NULL;
			"
			),
			forward_declarations = forward_declarations.join("\n"),
		);
		prelude = regex_macro::regex!("(\\s*\r?\n){3,}").replace_all(&prelude, "\n\n").to_string();

		Ok(prelude)
	}
}

impl ToCabin for Program {
	fn to_cabin(&self) -> String {
		let mut cabin_code = String::new();
		for statement in &self.statements {
			write!(cabin_code, "{}\n\n", statement.to_cabin()).unwrap();
		}
		cabin_code
	}
}

impl ColoredCabin for Program {
	fn to_colored_cabin(&self, context: &mut Context) -> String {
		let mut cabin_code = String::new();
		let mut current_line = 1;
		for statement in &self.statements {
			if let Statement::Declaration(declaration) = statement {
				while current_line <= declaration.line_start {
					cabin_code.push('\n');
					current_line += 1;
				}
			}
			write!(cabin_code, "{}", statement.to_colored_cabin(context)).unwrap();
			current_line = cabin_code.lines().count() + 1;
		}

		cabin_code
	}
}

/// Parses a token stream into an abstract syntax tree.
///
/// # Parameters
/// - `tokens` - A mutable reference to a token stream.
///
/// # Returns
/// A `Result` containing either an `AST` or an `Error`.
pub fn parse(tokens: &mut std::collections::VecDeque<Token>, context: &mut Context) -> anyhow::Result<Program> {
	Program::parse(tokens, context)
}

/// A trait for parsing a token stream into an abstract syntax tree node using a specific rule.
pub trait Parse {
	/// The type of abstract syntax tree node that this rule parses into.
	type Output;

	/// Parses a token stream into an abstract syntax tree node using this rule.
	///
	/// # Parameters
	/// - `tokens` - The token stream to parse
	///
	/// # Returns
	/// A `Result` containing either an abstract syntax tree node or an `Error`.
	fn parse(tokens: &mut std::collections::VecDeque<Token>, context: &mut Context) -> anyhow::Result<Self::Output>;
}

/// A trait for treating a collection of tokens as a queue of tokens that can be parsed. This is
/// traditionally implemented for `VecDeque<Token>`.
pub trait TokenQueue {
	/// Removes and returns the next token's value in the queue if the token matches the given token type. If it
	/// does not (or the token stream is empty), an error is returned.
	///
	/// # Parameters
	/// - `token_type` - The type of token to pop.
	///
	/// # Returns
	/// A `Result` containing either the value of the popped token or an `Error`.
	fn pop(&mut self, token_type: TokenType, context: &mut Context) -> Result<String, TokenError>;

	/// Removes and returns the next token's type in the queue if the token matches the given token type. If it
	/// does not (or the token stream is empty), an error is returned.
	///
	/// # Parameters
	/// - `token_type` - The type of token to pop.
	///
	/// # Returns
	/// A `Result` containing either the type of the popped token or an `Error`.
	fn pop_type(&mut self, token_type: TokenType) -> anyhow::Result<TokenType>;

	/// Returns a reference to the next token in the queue without removing it. If the queue is empty, `None`
	/// is returned.
	///
	/// # Returns
	/// A reference to the next token in the queue or `None` if the queue is empty.
	fn peek(&self) -> Option<&Token>;

	/// Returns whether the next token in the queue matches the given token type.
	fn next_is(&self, token_type: TokenType) -> bool;

	/// Returns whether the next token in the queue matches one of the given token types.
	///
	/// # Parameters
	/// - `token_types` - The token types to check against.
	///
	/// # Returns
	/// Whether the next token in the queue matches one of the given token types.
	fn next_is_one_of(&self, token_types: &[TokenType]) -> bool {
		token_types.iter().any(|token_type| self.next_is(token_type.clone()))
	}

	/// Returns the line number, as given in the original source code, that the *next* token is written on. This
	/// is used by AST nodes during parsing to get positional information about their tokens and use that to
	/// pretty-print errors.
	fn current_line(&self) -> usize;

	/// Returns the column number, as given in the original source code, that the *next* token is written on. This
	/// is used by AST nodes during parsing to get positional information about their tokens and use that to
	/// pretty-print errors.
	fn current_column(&self) -> usize;
}

impl TokenQueue for std::collections::VecDeque<Token> {
	fn peek(&self) -> Option<&Token> {
		self.front()
	}

	fn pop(&mut self, token_type: TokenType, context: &mut Context) -> Result<String, TokenError> {
		if let Some(token) = self.pop_front() {
			if token.token_type == token_type {
				return Ok(token.value);
			}

			return Err(TokenError {
				message: format!(
					"Expected {} but found {}",
					format!("{token_type}").bold().cyan(),
					format!("{}", token.token_type).bold().cyan()
				),
				line: token.line,
				severity: Severity::Error,
				filename: context.file_name.clone(),
			});
		}

		Err(TokenError {
			message: format!("Expected {token_type} but found EOF"),
			line: 0,
			severity: Severity::Error,
			filename: context.file_name.clone(),
		})
	}

	fn pop_type(&mut self, token_type: TokenType) -> anyhow::Result<TokenType> {
		if let Some(token) = self.pop_front() {
			if token.token_type == token_type {
				return Ok(token.token_type);
			}
		}
		anyhow::bail!(
			"{}:{}:pop_type error: Expected {token_type} but found {}",
			self.peek().map_or(0, |token| token.line),
			self.peek().map_or(0, |token| token.column),
			self.peek().map_or("EOF".to_owned(), |token| format!("{}", token.token_type))
		)
	}

	fn next_is(&self, token_type: TokenType) -> bool {
		self.peek().map_or(false, |token| token.token_type == token_type)
	}

	fn current_line(&self) -> usize {
		self.peek().unwrap().line
	}

	fn current_column(&self) -> usize {
		self.peek().unwrap().column
	}
}

/// Parses a comma-separated list of things. This takes a block of code as one of its parameters. The block is run once at the beginning,
/// and then while the next token is a comma, a comma is consumed and the block is run again. This is used for many comma-separated lists
/// in the language like function parameters, function arguments, group fields, group instantiation, etc.
///
/// TODO: Currently the language doesn't allow trailing commas. We should consider when/if we want to allow these and how to do so.
#[macro_export]
macro_rules! parse_list {
	(
		$tokens: expr, $context: expr, $body: block
	) => {
		$body;
		while $tokens.next_is(TokenType::Comma) {
			$tokens.pop(TokenType::Comma, $context)?;
			$body;
		}
	};
}
