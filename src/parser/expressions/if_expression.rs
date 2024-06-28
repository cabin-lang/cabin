use crate::{
	compile_time::{CompileTime, CompileTimeStatement, TranspileToC},
	context::Context,
	formatter::{ColoredCabin, ToCabin},
	lexer::{Token, TokenType},
	parser::{
		expressions::{run::ParentExpression, util::types::Typed, Expression},
		statements::Statement,
		Parse, TokenQueue,
	},
	void_literal,
};

// Brings the `write!()` and `writeln!()` macros into scope, which allows appending to a string. This is more efficient than using
// `string = format!("{string}...")`, because it avoids an extra allocation. We have a clippy warning turned on for this very
// purpose. We assign this to `_` to indicate clearly that it's just a trait and not used explicitly anywhere outside of bringing its
// methods into scope.
use std::{collections::VecDeque, fmt::Write as _};

use colored::Colorize as _;

use super::{literals::Literal, run::ParentStatement, util::name::Name};

/// An if expression. This is a conditional statement that executes a block of code if a condition is true, and optionally
/// executes another block of code if the condition is false.
#[derive(Clone, Debug)]
pub struct IfExpression {
	/// The condition of the if statement. This should evaluate to a boolean value.
	condition: Expression,
	/// The body of the if statement. This is executed if the condition is true.
	body: Vec<Statement>,
	/// The body of the else statement. This is executed if the condition is false.
	else_body: Option<Vec<Statement>>,
}

impl Parse for IfExpression {
	type Output = Self;

	fn parse(tokens: &mut VecDeque<Token>, context: &mut Context) -> anyhow::Result<Self::Output> {
		tokens.pop(TokenType::KeywordIf, context)?;
		let condition = Expression::parse(tokens, context)?;
		tokens.pop(TokenType::LeftBrace, context)?;
		let mut body = Vec::new();
		while !tokens.next_is(TokenType::RightBrace) {
			body.push(Statement::parse(tokens, context)?);
		}
		tokens.pop(TokenType::RightBrace, context)?;

		let else_body = if tokens.next_is(TokenType::KeywordOtherwise) {
			tokens.pop(TokenType::KeywordOtherwise, context)?;
			tokens.pop(TokenType::LeftBrace, context)?;
			let mut else_body = Vec::new();
			while !tokens.next_is(TokenType::RightBrace) {
				else_body.push(Statement::parse(tokens, context)?);
			}
			tokens.pop(TokenType::RightBrace, context)?;
			Some(else_body)
		} else {
			None
		};

		Ok(Self { condition, body, else_body })
	}
}

impl Typed for IfExpression {
	fn get_type(&self, context: &mut Context) -> anyhow::Result<Literal> {
		for statement in &self.body {
			if let Statement::Tail(tail) = statement {
				return tail.expression.get_type(context);
			}
		}

		Ok(void_literal!())
	}
}

impl CompileTime for IfExpression {
	fn compile_time_evaluate(&self, context: &mut Context, _with_side_effects: bool) -> anyhow::Result<Expression> {
		let Expression::IfStatement(if_expression) = self.evaluate_children_at_compile_time(context)? else {
			unreachable!()
		};

		if let Expression::Literal(literal) = &if_expression.condition {
			let true_expression = context.scope_data.get_global_variable(&Name("true".to_owned())).unwrap().value.as_ref().unwrap().clone();
			let true_literal = true_expression.as_literal(context).unwrap();

			if literal.is(true_literal, context)? {
				for statement in &if_expression.body {
					let result = statement.evaluate_statement_children_at_compile_time(context)?;
					if let Statement::Tail(tail_statement) = result {
						return Ok(tail_statement.expression);
					}
				}
			}
		}

		Ok(Expression::IfStatement(if_expression))
	}
}

impl ParentExpression for IfExpression {
	fn evaluate_children_at_compile_time(&self, context: &mut Context) -> anyhow::Result<Expression> {
		let condition = self.condition.compile_time_evaluate(context, true)?;
		let body = self
			.body
			.iter()
			.map(|statement| statement.compile_time_evaluate_statement(context, true))
			.collect::<anyhow::Result<Vec<_>>>()?;

		let else_body = self
			.else_body
			.as_ref()
			.map(|else_body| {
				else_body
					.iter()
					.map(|statement| statement.compile_time_evaluate_statement(context, true))
					.collect::<anyhow::Result<Vec<_>>>()
			})
			.transpose()?;

		Ok(Expression::IfStatement(Box::new(Self { condition, body, else_body })))
	}
}

impl TranspileToC for IfExpression {
	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		let condition = self.condition.to_c(context)?;
		let body = self.body.iter().map(|statement| statement.to_c(context)).collect::<anyhow::Result<Vec<_>>>()?.join("\n");
		let else_body = self
			.else_body
			.as_ref()
			.map(|else_body| else_body.iter().map(|statement| statement.to_c(context)).collect::<anyhow::Result<Vec<_>>>())
			.transpose()?
			.map_or("NULL;".to_owned(), |else_body| else_body.join("\n"));

		Ok(format!("{condition} ? ({{ {body} }}) : ({{ {else_body} }})"))
	}

	fn c_prelude(&self, context: &mut Context) -> anyhow::Result<String> {
		let condition = self.condition.c_prelude(context)?;
		let body = self
			.body
			.iter()
			.map(|statement| statement.c_prelude(context))
			.collect::<anyhow::Result<Vec<_>>>()?
			.join("\n");

		let else_body = self
			.else_body
			.as_ref()
			.map(|else_body| else_body.iter().map(|statement| statement.c_prelude(context)).collect::<anyhow::Result<Vec<_>>>())
			.transpose()?
			.map_or(String::new(), |else_body| else_body.join("\n"));

		Ok([condition, body, else_body].join("\n"))
	}
}

impl ToCabin for IfExpression {
	fn to_cabin(&self) -> String {
		let mut cabin_code = format!("if {} {{", self.condition.to_cabin());
		for statement in &self.body {
			for line in statement.to_cabin().lines() {
				write!(cabin_code, "\t{line}").unwrap();
			}
		}
		if let Some(else_body) = &self.else_body {
			cabin_code.push_str("\totherwise {");
			for statement in else_body {
				for line in statement.to_cabin().lines() {
					write!(cabin_code, "\t{line}").unwrap();
				}
			}
		}
		cabin_code
	}
}

impl ColoredCabin for IfExpression {
	fn to_colored_cabin(&self, context: &mut Context) -> String {
		let mut cabin_code = format!("{} {} {{", "if".purple(), self.condition.to_colored_cabin(context));
		for statement in &self.body {
			for line in statement.to_colored_cabin(context).lines() {
				write!(cabin_code, "\t{line}").unwrap();
			}
		}
		if let Some(else_body) = &self.else_body {
			write!(cabin_code, "\t{} {{", "otherwise".purple()).unwrap();
			for statement in else_body {
				for line in statement.to_colored_cabin(context).lines() {
					write!(cabin_code, "\t{line}").unwrap();
				}
			}
		}
		cabin_code
	}
}
