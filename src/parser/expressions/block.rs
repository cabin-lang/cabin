use crate::{
	api::{context::Context, scope::ScopeType},
	comptime::CompileTime,
	lexer::TokenType,
	parser::{expressions::Expression, statements::Statement, Parse, TokenQueue, TokenQueueFunctionality as _},
	transpiler::TranspileToC,
};

#[derive(Debug, Clone)]
pub struct Block {
	pub statements: Vec<Statement>,
	pub inner_scope_id: usize,
}

impl Parse for Block {
	type Output = Block;

	fn parse(tokens: &mut TokenQueue, context: &mut Context) -> anyhow::Result<Self::Output> {
		if let Some(scope_label) = &context.scope_label {
			context.scope_data.enter_new_scope(ScopeType::Block, scope_label);
		} else {
			context.scope_data.enter_new_unlabeled_scope(ScopeType::Block);
		}

		let scope_id = context.scope_data.unique_id();
		tokens.pop(TokenType::LeftBrace)?;
		let mut statements = Vec::new();
		while !tokens.next_is(TokenType::RightBrace) {
			statements.push(Statement::parse(tokens, context)?);
		}
		tokens.pop(TokenType::RightBrace)?;
		context.scope_data.exit_scope()?;
		Ok(Block {
			statements,
			inner_scope_id: scope_id,
		})
	}
}

impl CompileTime for Block {
	/// The output for evaluating blocks at compile-time is a generic `Expression`. This is because while some blocks
	/// will not be able to be fully evaluated and will remain as blocks, some others *will* be able to be resolved
	/// fully, and those will return either the expressed from their tail statement, or `Expression::Void` if no tail
	/// statement was present.
	type Output = Expression;

	fn evaluate_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self::Output> {
		let mut statements = Vec::new();
		let previous_scope = context.scope_data.set_current_scope(self.inner_scope_id);
		for statement in self.statements {
			let evaluated_statement = statement.evaluate_at_compile_time(context)?;

			// Tail statement
			if let Statement::Tail(tail_statement) = evaluated_statement {
				if tail_statement.value.try_as_literal(context).is_ok() {
					context.scope_data.set_current_scope(previous_scope);
					return Ok(tail_statement.value);
				}
				statements.push(Statement::Tail(tail_statement));
			}
			// Not tail statement
			else {
				statements.push(evaluated_statement);
			}
		}

		context.scope_data.set_current_scope(previous_scope);
		Ok(Expression::Block(Block {
			statements,
			inner_scope_id: self.inner_scope_id,
		}))
	}
}

impl TranspileToC for Block {
	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		let mut builder = String::new();
		builder += "({";
		for statement in &self.statements {
			for line in statement.to_c(context)?.lines() {
				builder += &format!("\n{line}");
			}
		}
		builder += "\n})";

		Ok(builder)
	}
}
