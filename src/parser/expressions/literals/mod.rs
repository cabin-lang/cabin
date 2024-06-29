use std::sync::atomic::{AtomicUsize, Ordering};

use crate::{
	compile_time::{ambassador_impl_TranspileToC, CompileTime, TranspileToC},
	context::Context,
	formatter::{ambassador_impl_ColoredCabin, ambassador_impl_ToCabin, ColoredCabin, ToCabin},
	lexer::{Token, TokenType},
	object_literal, parse_list,
	parser::{
		expressions::{
			literals::{
				either::Either,
				function_declaration::FunctionDeclaration,
				group::GroupDeclaration,
				object::{InternalValue, Object},
				variable_reference::VariableReference,
			},
			run::ParentExpression,
			util::{
				name::Name,
				types::{ambassador_impl_Typed, Typed},
			},
			Expression,
		},
		Parse, TokenQueue,
	},
};

/// The `object` module, which handles object literals.
pub mod object;

/// The `group` module, which handles group declarations.
pub mod group;

/// The `function_declaration` module, which handles function declarations.
pub mod function_declaration;

/// The `variable_reference` module, which handles identifiers that reference variables.
pub mod variable_reference;

/// The `either` module, which handles either declarations.
pub mod either;

/// A literal value, which has properties with known values. This is comprised of a `LiteralValue` (the value), and a virtual address, which is a unique identifier
/// for this literal.
///
/// A literal also refers to a type. A "type" in Cabin is any expression that can be evaluated into a literal at compile-time. Thus, functions that need to return types, such as `Expression::get_type`,
/// returns a `Literal`.
#[derive(ambassador::Delegate)]
#[delegate(TranspileToC, target = "0")]
#[delegate(ToCabin, target = "0")]
#[delegate(ColoredCabin, target = "0")]
#[delegate(Typed, target = "0")]
#[derive(Debug, Clone)]
pub struct Literal(pub LiteralValue, pub usize);

impl Literal {
	/// Creates a new literal with the given virtual address.
	///
	/// # Parameters
	/// - `value` - The value of the literal
	/// - `virtual_address` - The virtual address of the literal.
	///
	/// # Returns
	/// The newly created literal.
	pub const fn with_virtual_address(value: LiteralValue, virtual_address: usize) -> Self {
		Self(value, virtual_address)
	}

	/// Creates a new literal and assigns the virtual address to the next unused virtual address.
	///
	/// # Parameters
	/// - `value` - The value of the literal
	///
	/// # Returns
	/// The newly created literal.
	pub fn new(value: LiteralValue) -> Self {
		Self(value, next_unused_virtual_address())
	}

	/// Returns the value of this literal.
	pub const fn value(&self) -> &LiteralValue {
		&self.0
	}

	/// Returns the virtual address of this literal.
	pub const fn virtual_address(&self) -> usize {
		self.1
	}

	/// Returns whether this literal points to the same value as another literal by comparing their virtual addresses.
	pub fn is(&self, other: &Self, context: &mut Context) -> anyhow::Result<bool> {
		let self_address = if let Self(LiteralValue::VariableReference(variable_reference), ..) = self {
			let value = variable_reference.value(context)?;
			value.as_literal(context).map_or(None, |literal| Some(literal.virtual_address()))
		} else {
			Some(self.virtual_address())
		};

		let other_address = if let Self(LiteralValue::VariableReference(variable_reference), ..) = other {
			let value = variable_reference.value(context)?;
			value.as_literal(context).map_or(None, |literal| Some(literal.virtual_address()))
		} else {
			Some(other.virtual_address())
		};

		Ok(self_address == other_address)
	}
}

// Ensure that when literals are evaluated at compile-time, they keep the same virtual address
impl CompileTime for Literal {
	fn compile_time_evaluate(&self, context: &mut Context, with_side_effects: bool) -> anyhow::Result<Expression> {
		let Expression::Literal(Self(value, ..)) = self.value().compile_time_evaluate(context, with_side_effects)? else {
			context.encountered_compiler_bug = true;
			anyhow::bail!("Literal after compile-time evaluation is not a literal");
		};

		Ok(Expression::Literal(Self::with_virtual_address(value, self.virtual_address())))
	}
}

// Ensure that when literals are evaluated at compile-time, they keep the same virtual address
impl ParentExpression for Literal {
	fn evaluate_children_at_compile_time(&self, context: &mut Context) -> anyhow::Result<Expression> {
		let Expression::Literal(Self(value, ..)) = self.value().evaluate_children_at_compile_time(context)? else {
			context.encountered_compiler_bug = true;
			anyhow::bail!("Literal after compile-time evaluation is not a literal");
		};

		Ok(Expression::Literal(Self::with_virtual_address(value, self.virtual_address())))
	}
}

impl Parse for Literal {
	type Output = Self;

	fn parse(tokens: &mut std::collections::VecDeque<Token>, context: &mut Context) -> anyhow::Result<Self::Output> {
		Ok(Self::new(LiteralValue::parse(tokens, context)?))
	}
}

/// A literal in the language. Literals are values that are directly written in the code, such as numbers, strings, and identifiers.
#[derive(Clone, Debug)]
#[enum_dispatch::enum_dispatch(TranspileToC)]
#[enum_dispatch::enum_dispatch(ToCabin)]
#[enum_dispatch::enum_dispatch(CompileTime)]
#[enum_dispatch::enum_dispatch(ColoredCabin)]
#[enum_dispatch::enum_dispatch(ParentExpression)]
#[enum_dispatch::enum_dispatch(Typed)]
pub enum LiteralValue {
	/// An identifier literal.
	VariableReference(VariableReference),
	/// A table literal.
	Object(Object),
	/// A group literal.
	Group(GroupDeclaration),
	/// A function declaration literal.
	FunctionDeclaration(Box<FunctionDeclaration>),
	/// An either declaration literal.
	Either(Either),
}

impl Parse for LiteralValue {
	type Output = Self;

	fn parse(tokens: &mut std::collections::VecDeque<Token>, context: &mut Context) -> anyhow::Result<Self::Output> {
		Ok(match tokens.peek().ok_or_else(|| anyhow::anyhow!("Expected literal but found end of input"))?.token_type {
			// Number literals
			TokenType::Number => {
				object_literal! {
					Number {
						internal_fields = {
							internal_value = InternalValue::Number(tokens.pop(TokenType::Number, context)?.parse::<f64>()?)
						}
					}
				}
			},

			// String literals
			TokenType::String => {
				let string = tokens.pop(TokenType::String, context)?;
				object_literal! {
					Text {
						internal_fields = {
							internal_value = InternalValue::String(unindent::unindent(string.get(1 .. string.len() - 1).unwrap_or_else(|| unreachable!())))
						}
					}
				}
			},

			// List literal
			TokenType::LeftBracket => {
				tokens.pop(TokenType::LeftBracket, context)?;
				let mut list = Vec::new();
				if !tokens.next_is(TokenType::RightBracket) {
					parse_list!(tokens, context, {
						list.push(Expression::parse(tokens, context)?);
					});
				}
				tokens.pop(TokenType::RightBracket, context)?;
				object_literal! {
					List {
						internal_fields = {
							data = InternalValue::List(list)
						}
					}
				}
			},

			// Other expressions
			TokenType::Identifier => Self::VariableReference(VariableReference::parse(tokens, context)?),
			TokenType::KeywordNew => Self::Object(Object::parse(tokens, context)?),
			TokenType::KeywordGroup => Self::Group(GroupDeclaration::parse(tokens, context)?),
			TokenType::KeywordEither => Self::Either(Either::parse(tokens, context)?),
			TokenType::KeywordAction => Self::FunctionDeclaration(Box::new(FunctionDeclaration::parse(tokens, context)?)),

			// Not a literal
			_ => anyhow::bail!("Expected literal but found {}", tokens.peek().unwrap().token_type),
		})
	}
}

/// The next unused virtual address for literals.
static NEXT_UNUSED_VIRTUAL_ADDRESS: AtomicUsize = AtomicUsize::new(0);

/// Returns the next unused virtual address for a literal.
///
/// # Returns
/// the next unused virtual address.
fn next_unused_virtual_address() -> usize {
	NEXT_UNUSED_VIRTUAL_ADDRESS.fetch_add(1, Ordering::Relaxed)
}
