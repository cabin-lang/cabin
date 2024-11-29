use crate::{
	api::{context::context, scope::ScopeType},
	comptime::CompileTime,
	lexer::TokenType,
	parser::{
		expressions::{name::Name, Expression},
		Parse, TokenQueue, TokenQueueFunctionality,
	},
	transpiler::TranspileToC,
};

#[derive(Debug, Clone)]
pub struct Label {
	kind: ScopeType,
}

impl Label {
	pub fn new(name: Name) -> anyhow::Result<Self> {
		Ok(Self {
			kind: context().scope_data.scope_type_of(&name)?.to_owned(),
		})
	}
}

#[derive(Debug, Clone)]
pub struct TailStatement {
	pub label: Name,
	pub value: Expression,
}

impl Parse for TailStatement {
	type Output = TailStatement;

	fn parse(tokens: &mut TokenQueue) -> anyhow::Result<Self::Output> {
		let label = Name::parse(tokens)?;

		tokens.pop(TokenType::KeywordIs)?;
		let value = Expression::parse(tokens)?;

		Ok(TailStatement { label, value })
	}
}

impl CompileTime for TailStatement {
	type Output = TailStatement;

	fn evaluate_at_compile_time(self) -> anyhow::Result<Self::Output> {
		let value = self.value.evaluate_at_compile_time()?;
		Ok(TailStatement { label: self.label, value })
	}
}

impl TranspileToC for TailStatement {
	fn to_c(&self) -> anyhow::Result<String> {
		Ok(match Label::new(self.label.clone())?.kind {
			ScopeType::Function => format!("*return_address = {};\nreturn;", self.value.to_c()?),
			_ => format!("*tail_value = {};\ngoto label_{};", self.value.to_c()?, self.label.to_c()?),
		})
	}
}
