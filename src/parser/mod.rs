use std::collections::VecDeque;

use colored::Colorize as _;

use crate::{
	api::context::Context,
	comptime::CompileTime,
	lexer::{Position, Token, TokenType},
	mapped_err,
	parser::statements::Statement,
};

pub mod expressions;
pub mod statements;

pub fn parse(tokens: &mut TokenQueue, context: &mut Context) -> anyhow::Result<Program> {
	Program::parse(tokens, context)
}

#[derive(Debug)]
pub struct Program {
	statements: Vec<Statement>,
}

impl Parse for Program {
	type Output = Self;

	fn parse(tokens: &mut TokenQueue, context: &mut Context) -> anyhow::Result<Self::Output> {
		let mut statements = Vec::new();
		while !tokens.is_empty() {
			statements.push(Statement::parse(tokens, context).map_err(mapped_err! {
				while = "while parsing the program's top-level statements",
				context = context,
			})?);
		}
		Ok(Program { statements })
	}
}

impl CompileTime for Program {
	type Output = Program;

	fn evaluate_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self::Output> {
		Ok(Self {
			statements: self
				.statements
				.into_iter()
				.map(|statement| statement.evaluate_at_compile_time(context))
				.collect::<anyhow::Result<Vec<_>>>()
				.map_err(mapped_err! {
					while = "evaluating the program's global statements at compile-time",
					context = context,
				})?
				.into_iter()
				.collect(),
		})
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

	fn peek_type(&self) -> anyhow::Result<&TokenType>;

	fn peek_type2(&self) -> anyhow::Result<&TokenType>;

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

	fn current_position(&self) -> Option<Position>;
}

impl TokenQueueFunctionality for std::collections::VecDeque<Token> {
	fn peek(&self) -> anyhow::Result<&str> {
		Ok(&self.front().ok_or_else(|| anyhow::anyhow!("Unexpected end of file"))?.value)
	}

	fn peek_type(&self) -> anyhow::Result<&TokenType> {
		Ok(&self.front().ok_or_else(|| anyhow::anyhow!("Unexpected end of file."))?.token_type)
	}

	fn peek_type2(&self) -> anyhow::Result<&TokenType> {
		Ok(&self.get(1).ok_or_else(|| anyhow::anyhow!("Unexpected end of file."))?.token_type)
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
		self.peek_type().map_or(false, |token| token == &token_type)
	}

	fn current_position(&self) -> Option<Position> {
		self.front().map(|front| front.position.clone())
	}
}

/// Parses a comma-separated list of things. This takes a block of code as one of its parameters. The block is run once at the beginning,
/// and then while the next token is a comma, a comma is consumed and the block is run again. This is used for many comma-separated lists
/// in the language like function parameters, function arguments, group fields, group instantiation, etc.
#[macro_export]
macro_rules! parse_list {
	(
		$tokens: expr, $list_type: expr, $body: block
	) => {{
		use $crate::parser::TokenQueueFunctionality as _;

		$tokens.pop($list_type.opening())?;
		while !$tokens.next_is($list_type.closing()) {
			$body
			if $tokens.next_is($crate::lexer::TokenType::Comma) {
				$tokens.pop($crate::lexer::TokenType::Comma)?;
			} else {
				break;
			}
		}

		$tokens.pop($list_type.closing())?;
	}};
}

pub enum ListType {
	AngleBracketed,
	Braced,
	Bracketed,
	Parenthesized,
	Tag,
}

impl ListType {
	fn opening(&self) -> TokenType {
		match self {
			Self::AngleBracketed => TokenType::LeftAngleBracket,
			Self::Braced => TokenType::LeftBrace,
			Self::Bracketed => TokenType::LeftBracket,
			Self::Parenthesized => TokenType::LeftParenthesis,
			Self::Tag => TokenType::TagOpening,
		}
	}

	fn closing(&self) -> TokenType {
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

	fn parse(tokens: &mut TokenQueue, context: &mut Context) -> anyhow::Result<Self::Output>;
}

pub type TokenQueue = VecDeque<Token>;

pub trait ToCabin {
	fn to_cabin(&self) -> String;
}
