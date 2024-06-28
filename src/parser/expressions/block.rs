use crate::{
	compile_time::{CompileTime, CompileTimeStatement, TranspileToC},
	context::Context,
	formatter::{ColoredCabin, ToCabin},
	lexer::{Token, TokenType},
	parser::{
		expressions::{
			run::{ParentExpression, ParentStatement},
			util::types::Typed,
			Expression,
		},
		statements::Statement,
		Parse, TokenQueue,
	},
	scopes::ScopeType,
};

use std::{collections::VecDeque, fmt::Write as _};

use colored::Colorize as _;

use super::literals::{Literal, LiteralValue};

/// An expression block. This allows running a series of statements inside an expression, and returning a value from one of those statements.
#[derive(Debug, Clone)]
pub struct Block {
	/// The statements inside this block.
	pub statements: Vec<Statement>,

	/// The id of the scope of the inside of this block.
	pub inner_scope_id: usize,
}

impl Parse for Block {
	type Output = Self;

	fn parse(tokens: &mut VecDeque<Token>, context: &mut Context) -> anyhow::Result<Self::Output> {
		tokens
			.pop(TokenType::LeftBrace, context)
			.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while attempting to parse the opening left brace at the start of a block".dimmed()))?;

		context.scope_data.enter_new_scope(ScopeType::Block);

		let inner_scope_id = context.scope_data.unique_id();

		let mut statements = Vec::new();
		while !tokens.next_is(TokenType::RightBrace) {
			statements.push(
				Statement::parse(tokens, context)
					.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while attempting to parse the closing right brace at the start of a block".dimmed()))?,
			);
		}

		tokens
			.pop(TokenType::RightBrace, context)
			.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while attempting to parse the closing right brace at the start of a block".dimmed()))?;

		context.scope_data.exit_scope()?;

		Ok(Self { statements, inner_scope_id })
	}
}

impl CompileTime for Block {
	fn compile_time_evaluate(&self, context: &mut Context, with_side_effects: bool) -> anyhow::Result<Expression> {
		let previous_scope_id = context.scope_data.set_current_scope(self.inner_scope_id);

		let block = Self {
			statements: self
				.statements
				.iter()
				.map(|statement| statement.compile_time_evaluate_statement(context, with_side_effects))
				.collect::<anyhow::Result<Vec<_>>>()
				.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating a block expression at compile-time".dimmed()))?,
			inner_scope_id: self.inner_scope_id,
		};

		for statement in &block.statements {
			if let Statement::Tail(tail_statement) = statement {
				if let Expression::Literal(literal) = &tail_statement.expression {
					let mut literal_value = literal.clone();

					if let Literal(LiteralValue::VariableReference(variable_reference), ..) = literal {
						let value = context
							.scope_data
							.get_variable_from_id(variable_reference.name(), variable_reference.scope_id())
							.unwrap()
							.value
							.clone()
							.unwrap()
							.compile_time_evaluate(context, with_side_effects)
							.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating the tail statement in a block at compile-time".dimmed()))?;

						if let Expression::Literal(evaluated_value) = value {
							literal_value = evaluated_value;
						}
					}

					if !literal.is(&context.unknown_at_compile_time().clone(), context)? {
						context.scope_data.set_current_scope(previous_scope_id);
						return Ok(Expression::Literal(literal_value));
					}
				}
			}
		}

		context.scope_data.set_current_scope(previous_scope_id);
		Ok(Expression::Block(block))
	}
}

impl ParentExpression for Block {
	fn evaluate_children_at_compile_time(&self, context: &mut Context) -> anyhow::Result<Expression> {
		let previous_scope_id = context.scope_data.set_current_scope(self.inner_scope_id);

		let block = Expression::Block(Self {
			statements: self
				.statements
				.iter()
				.map(|statement| statement.evaluate_statement_children_at_compile_time(context))
				.collect::<anyhow::Result<Vec<_>>>()
				.map_err(|error| {
					anyhow::anyhow!(
						"{error}\n\t{}",
						"while evaluating the sub-expressions in the statements of a block at compile-time".dimmed()
					)
				})?,
			inner_scope_id: self.inner_scope_id,
		});

		context.scope_data.set_current_scope(previous_scope_id);

		Ok(block)
	}
}

impl TranspileToC for Block {
	fn c_prelude(&self, context: &mut Context) -> anyhow::Result<String> {
		Ok(self
			.statements
			.iter()
			.map(|statement| Ok(format!("\t{}", statement.c_prelude(context)?)))
			.collect::<anyhow::Result<Vec<_>>>()?
			.join("\n"))
	}

	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		let mut c = "({".to_owned();

		if !self.statements.is_empty() {
			c.push('\n');
		}

		for statement in &self.statements {
			for line in statement.to_c(context)?.lines() {
				writeln!(c, "\t{line}").unwrap();
			}
		}
		c.push_str("})");
		Ok(c)
	}
}

impl ToCabin for Block {
	fn to_cabin(&self) -> String {
		format!(
			"{{\n{}\n}}",
			self.statements
				.iter()
				.flat_map(|statement| statement.to_cabin().lines().map(|line| format!("\t{line}")).collect::<Vec<_>>())
				.collect::<Vec<_>>()
				.join("\n")
		)
	}
}

impl ColoredCabin for Block {
	fn to_colored_cabin(&self, context: &mut Context) -> String {
		format!(
			"{{\n{}\n}}",
			self.statements
				.iter()
				.map(|statement| format!("\t{}", statement.to_colored_cabin(context)))
				.collect::<Vec<_>>()
				.join("\n")
		)
	}
}

impl Typed for Block {
	fn get_type(&self, context: &mut Context) -> anyhow::Result<Literal> {
		for statement in &self.statements {
			if let Statement::Tail(tail) = statement {
				return tail.expression.get_type(context);
			}
		}

		anyhow::bail!("Attempted to get the type of a block with no tail statement: {self:?}");
	}
}
