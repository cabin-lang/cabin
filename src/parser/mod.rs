use std::collections::{HashMap, VecDeque};

use colored::Colorize as _;
use expressions::{
	field_access::FieldAccessType,
	literal::LiteralObject,
	object::{Field, ObjectConstructor},
};
use statements::{
	declaration::{Declaration, DeclarationType},
	tag::TagList,
};

use crate::{
	api::{
		context::context,
		scope::{ScopeId, ScopeType},
		traits::TryAs,
	},
	comptime::{memory::VirtualPointer, CompileTime},
	lexer::{Span, Token, TokenType},
	mapped_err,
	transpiler::TranspileToC,
};

pub mod expressions;
pub mod statements;

pub fn parse(tokens: &mut TokenQueue) -> anyhow::Result<Module> {
	Module::parse(tokens)
}

#[derive(Debug)]
pub struct Module {
	declarations: Vec<Declaration>,
	inner_scope_id: ScopeId,
}

impl Parse for Module {
	type Output = Self;

	fn parse(tokens: &mut TokenQueue) -> anyhow::Result<Self::Output> {
		context().scope_data.enter_new_scope(ScopeType::File);
		let inner_scope_id = context().scope_data.unique_id();
		let mut statements = Vec::new();
		while !tokens.is_empty() {
			statements.push(Declaration::parse(tokens).map_err(mapped_err! {
				while = "parsing the program's top-level declarations",
			})?);
		}
		context().scope_data.exit_scope()?;
		Ok(Module {
			declarations: statements,
			inner_scope_id,
		})
	}
}

impl CompileTime for Module {
	type Output = Module;

	fn evaluate_at_compile_time(self) -> anyhow::Result<Self::Output> {
		let previous = context().scope_data.set_current_scope(self.inner_scope_id);
		let evaluated = Self {
			declarations: self
				.declarations
				.into_iter()
				.map(|statement| statement.evaluate_at_compile_time())
				.collect::<anyhow::Result<Vec<_>>>()
				.map_err(mapped_err! {
					while = "evaluating the program's global statements at compile-time",
				})?
				.into_iter()
				.collect(),
			inner_scope_id: self.inner_scope_id,
		};
		context().scope_data.set_current_scope(previous);
		Ok(evaluated)
	}
}

impl TranspileToC for Module {
	fn to_c(&self) -> anyhow::Result<String> {
		Ok(self
			.declarations
			.iter()
			.map(|declaration| {
				if declaration.declaration_type() == &DeclarationType::RepresentAs
					|| declaration
						.value()
						.map_err(mapped_err! {
							while = "getting the value of a declaration",
						})?
						.is_pointer()
				{
					return Ok(None);
				}
				Ok(Some(declaration.to_c()?))
			})
			.collect::<anyhow::Result<Vec<_>>>()
			.map_err(mapped_err! {
				while = "transpiling the program's global statements to C",
			})?
			.into_iter()
			.flatten()
			.collect::<Vec<_>>()
			.join("\n"))
	}
}

/// A trait for treating a collection of tokens as a queue of tokens that can be parsed. This is
/// traditionally implemented for `VecDeque<Token>`.
pub trait TokenQueueFunctionality {
	/// Removes and returns the next token's value in the queue if the token matches the given token type. If it
	/// does not (or the token stream is empty), an error is returned.
	///
	/// # Parameters
	/// - `token_type` - The type of token to pop.
	///
	/// # Returns
	/// A `Result` containing either the value of the popped token or an `Error`.
	fn pop(&mut self, token_type: TokenType) -> anyhow::Result<Token>;

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
	fn peek(&self) -> anyhow::Result<&str>;

	fn peek_type(&self) -> anyhow::Result<TokenType>;

	fn peek_type2(&self) -> anyhow::Result<TokenType>;

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
		token_types.iter().any(|token_type| self.next_is(token_type.to_owned()))
	}

	fn current_position(&self) -> Option<Span>;
}

impl TokenQueueFunctionality for std::collections::VecDeque<Token> {
	fn peek(&self) -> anyhow::Result<&str> {
		Ok(&self.front().ok_or_else(|| anyhow::anyhow!("Unexpected end of file"))?.value)
	}

	fn peek_type(&self) -> anyhow::Result<TokenType> {
		Ok(self.front().ok_or_else(|| anyhow::anyhow!("Unexpected end of file."))?.token_type)
	}

	fn peek_type2(&self) -> anyhow::Result<TokenType> {
		Ok(self.get(1).ok_or_else(|| anyhow::anyhow!("Unexpected end of file."))?.token_type)
	}

	fn pop(&mut self, token_type: TokenType) -> anyhow::Result<Token> {
		if let Some(token) = self.pop_front() {
			if token.token_type == token_type {
				return Ok(token);
			}

			anyhow::bail!(
				"Expected {} but found {}",
				format!("{token_type}").bold().cyan(),
				format!("{}", token.token_type).bold().cyan()
			);
		}

		anyhow::bail!("Expected {token_type} but found EOF");
	}

	fn pop_type(&mut self, token_type: TokenType) -> anyhow::Result<TokenType> {
		let token = self.pop_front();
		if let Some(token) = token {
			if token.token_type == token_type {
				return Ok(token.token_type);
			}
			anyhow::bail!("Expected {token_type} but found {}", token.token_type);
		}

		anyhow::bail!("Expected {token_type} but found end of file.");
	}

	fn next_is(&self, token_type: TokenType) -> bool {
		self.peek_type().map_or(false, |token| token == token_type)
	}

	fn current_position(&self) -> Option<Span> {
		self.front().map(|front| front.span)
	}
}

impl Module {
	pub fn into_literal(self) -> anyhow::Result<LiteralObject> {
		Ok(LiteralObject {
			type_name: "Object".into(),
			fields: self
				.declarations
				.into_iter()
				.filter_map(|declaration| {
					(declaration.declaration_type() != &DeclarationType::RepresentAs).then(|| {
						let name = declaration.name().to_owned();
						let value = declaration.value().unwrap();
						(name, value.try_as::<VirtualPointer>().unwrap().to_owned())
					})
				})
				.collect(),
			internal_fields: HashMap::new(),
			field_access_type: FieldAccessType::Normal,
			inner_scope_id: Some(self.inner_scope_id),
			outer_scope_id: self.inner_scope_id,
			name: "anonymous_module".into(),
			address: None,
			span: Span::unknown(),
			tags: TagList::default(),
		})
	}

	pub fn into_object(self) -> anyhow::Result<ObjectConstructor> {
		Ok(ObjectConstructor {
			type_name: "Module".into(),
			fields: self
				.declarations
				.into_iter()
				.filter_map(|declaration| {
					(declaration.declaration_type() != &DeclarationType::RepresentAs).then(|| {
						let name = declaration.name().to_owned();
						let value = Some(declaration.value().unwrap().clone());
						Field { name, value, field_type: None }
					})
				})
				.collect(),
			internal_fields: HashMap::new(),
			field_access_type: FieldAccessType::Normal,
			inner_scope_id: self.inner_scope_id,
			outer_scope_id: self.inner_scope_id,
			name: "anonymous_module".into(),
			span: Span::unknown(),
			tags: TagList::default(),
		})
	}
}

pub enum ListType {
	AngleBracketed,
	Braced,
	Bracketed,
	Parenthesized,
	Tag,
}

impl ListType {
	const fn opening(&self) -> TokenType {
		match self {
			Self::AngleBracketed => TokenType::LeftAngleBracket,
			Self::Braced => TokenType::LeftBrace,
			Self::Bracketed => TokenType::LeftBracket,
			Self::Parenthesized => TokenType::LeftParenthesis,
			Self::Tag => TokenType::TagOpening,
		}
	}

	const fn closing(&self) -> TokenType {
		match self {
			Self::AngleBracketed => TokenType::RightAngleBracket,
			Self::Braced => TokenType::RightBrace,
			Self::Bracketed => TokenType::RightBracket,
			Self::Parenthesized => TokenType::RightParenthesis,
			Self::Tag => TokenType::RightBracket,
		}
	}
}

pub trait Parse {
	type Output;

	fn parse(tokens: &mut TokenQueue) -> anyhow::Result<Self::Output>;
}

pub type TokenQueue = VecDeque<Token>;

pub trait ToCabin {
	fn to_cabin(&self) -> String;
}
