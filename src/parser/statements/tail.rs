use crate::{
	api::context::Context,
	comptime::CompileTime,
	lexer::TokenType,
	parser::{
		expressions::{name::Name, Expression},
		Parse, TokenQueue, TokenQueueFunctionality,
	},
	transpiler::TranspileToC,
};

#[derive(Debug, Clone)]
pub enum Label {
	It,
	Return,
	Identifier(Name),
}

#[derive(Debug, Clone)]
pub struct TailStatement {
	pub label: Label,
	pub value: Expression,
}

impl Parse for TailStatement {
	type Output = TailStatement;

	fn parse(tokens: &mut TokenQueue, context: &mut Context) -> anyhow::Result<Self::Output> {
		let label = match tokens.peek_type()? {
			TokenType::Identifier => match tokens.peek()? {
				"it" => {
					tokens.pop(TokenType::Identifier)?;
					Label::It
				},
				_ => Label::Identifier(Name::parse(tokens, context)?),
			},
			TokenType::KeywordReturn => Label::Return,
			_ => anyhow::bail!("Expected label but found {}", tokens.peek_type().unwrap_or_else(|_| unreachable!())),
		};

		tokens.pop(TokenType::KeywordIs)?;
		let value = Expression::parse(tokens, context)?;

		Ok(TailStatement { label, value })
	}
}

impl CompileTime for TailStatement {
	type Output = TailStatement;

	fn evaluate_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self::Output> {
		let value = self.value.evaluate_at_compile_time(context)?;
		Ok(TailStatement { label: self.label, value })
	}
}

impl TranspileToC for TailStatement {
	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		Ok("goto label;".to_owned())
	}
}
