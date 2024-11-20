use std::collections::VecDeque;

use colored::Colorize as _;
use try_as::traits::{self as try_as_traits, TryAsRef as _};

use super::{statements::tag::TagList, Parse};
use crate::{
	comptime::{memory::Pointer, CompileTime},
	context::Context,
	lexer::Token,
	parser::expressions::{
		block::Block,
		either::Either,
		foreach::ForEachLoop,
		function::FunctionDeclaration,
		function_call::FunctionCall,
		group::GroupDeclaration,
		if_expression::IfExpression,
		name::Name,
		object::{LiteralObject, ObjectConstructor},
		oneof::OneOf,
		operators::{BinaryExpression, FieldAccess},
	},
};

pub mod block;
pub mod either;
pub mod foreach;
pub mod function;
pub mod function_call;
pub mod group;
pub mod if_expression;
pub mod list;
pub mod name;
pub mod object;
pub mod oneof;
pub mod operators;

#[derive(Debug, Clone, try_as::macros::From, try_as::macros::TryInto, try_as::macros::TryAsRef)]
pub enum Expression {
	Block(Block),

	/// This type of expression only exists reliably before compile-time evaluation; During compile-time evaluation all `eithers` will be converted
	/// into objects and stored in virtual memory.
	Either(Either),
	FieldAccess(FieldAccess),
	FunctionCall(FunctionCall),
	Group(GroupDeclaration),

	/// This type of expression only exists reliably before compile-time evaluation; During compile-time evaluation all function declarations will be converted
	/// into objects and stored in virtual memory.
	FunctionDeclaration(FunctionDeclaration),
	If(IfExpression),
	Name(Name),
	ObjectConstructor(ObjectConstructor),
	ForEachLoop(ForEachLoop),

	/// This type of expression only exists reliably before compile-time evaluation; During compile-time evaluation all `oneofs` will be converted
	/// into objects and stored in virtual memory.
	OneOf(OneOf),

	Pointer(Pointer),

	Void(()),
}

impl Parse for Expression {
	type Output = Expression;

	fn parse(tokens: &mut VecDeque<Token>, context: &mut Context) -> anyhow::Result<Self::Output> {
		BinaryExpression::parse(tokens, context)
	}
}

impl CompileTime for Expression {
	type Output = Expression;

	fn evaluate_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self::Output> {
		Ok(match self {
			Self::Block(block) => block
				.evaluate_at_compile_time(context)
				.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating a block at compile-time".dimmed()))?,
			Self::Either(either) => either
				.evaluate_at_compile_time(context)
				.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating an either at compile-time".dimmed()))?,
			Self::FunctionDeclaration(function_declaration) => function_declaration
				.evaluate_at_compile_time(context)
				.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating a function declaration at compile-time".dimmed()))?,
			Self::FieldAccess(field_access) => field_access
				.evaluate_at_compile_time(context)
				.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating a field access at compile-time".dimmed()))?,
			Self::FunctionCall(function_call) => function_call
				.evaluate_at_compile_time(context)
				.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating a function call at compile-time".dimmed()))?,
			Self::If(if_expression) => if_expression
				.evaluate_at_compile_time(context)
				.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating an if expression at compile-time".dimmed()))?,
			Self::Name(name) => name
				.evaluate_at_compile_time(context)
				.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while getting the value of a name at compile-time".dimmed()))?,
			Self::ObjectConstructor(object_constructor) => object_constructor
				.evaluate_at_compile_time(context)
				.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating an object constructor at compile-time".dimmed()))?,
			Self::Group(group) => group
				.evaluate_at_compile_time(context)
				.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating a group declaration at compile-time".dimmed()))?,
			Self::OneOf(oneof) => oneof
				.evaluate_at_compile_time(context)
				.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating a one-of declaration at compile-time".dimmed()))?,
			Self::ForEachLoop(for_loop) => for_loop
				.evaluate_at_compile_time(context)
				.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating a for-each loop at compile-time".dimmed()))?,
			Self::Void(_) | Self::Pointer(_) => self,
		})
	}
}

impl Expression {
	pub fn try_as_literal<'a>(&'a self, context: &'a Context) -> anyhow::Result<&'a LiteralObject> {
		if let Self::Pointer(address) = self {
			return context.virtual_memory.get(*address).ok_or_else(|| anyhow::anyhow!("Invalid pointer"));
		}

		anyhow::bail!("Attempted to coerce a non-literal into a literal");
	}

	pub fn expect_literal<'a>(&'a self, context: &'a Context) -> &'a LiteralObject {
		self.try_as_literal(context).unwrap()
	}

	pub fn is_pointer(&self) -> bool {
		matches!(self, Self::Pointer(_))
	}

	/// Returns the name of this type of expression as a string.
	///
	/// This is used when the compiler reports errors; For example, if an if-expression is
	/// used as a type, which should be a literal, the compiler will say something like "attempted
	/// to parse a literal, but an if-expression was found".
	///
	/// # Returns
	/// The name of the kind of expression of this as a string.
	#[must_use]
	pub const fn kind_name(&self) -> &'static str {
		match self {
			Self::Block(_) => "block",
			Self::Either(_) => "either",
			Self::FieldAccess(_) => "field access",
			Self::FunctionCall(_) => "function call",
			Self::FunctionDeclaration(_) => "function declaration",
			Self::Group(_) => "group declaration",
			Self::Name(_) => "name",
			Self::ObjectConstructor(_) => "object constructor",
			Self::OneOf(_) => "one-of",
			Self::Void(_) => "non-existent value",
			Self::Pointer(_) => "pointer",
			Self::If(_) => "if expression",
			Self::ForEachLoop(_) => "for-each loop",
		}
	}

	pub fn can_be_literal(&self) -> bool {
		match self {
			Self::ObjectConstructor(object) => object.is_literal(),
			Self::Pointer(_) => true,
			_ => false,
		}
	}

	/// Returns a new owned pointer to the same value in virtual memory as this referenced
	/// pointer. If this expression does indeed refer to a pointer, this is effectively a
	/// cheap `to_owned()`. If not, an error is returned.
	///
	/// # Errors
	/// If this expression doesn't refer to a pointer.
	///
	/// # Performance
	/// This clone is very cheap; Only the underlying pointer address (a `usize`) is cloned.
	#[must_use]
	pub fn try_clone_pointer(&self) -> anyhow::Result<Expression> {
		if let Self::Pointer(address) = self {
			return Ok(Expression::Pointer(*address));
		}

		anyhow::bail!("A value that's not fully known at compile-time was used as a type.");
	}

	pub fn is_true(&self, context: &Context) -> bool {
		let Some(literal_address): Option<&Pointer> = self.try_as_ref() else {
			return false;
		};

		let true_address = context.scope_data.get_global_variable(&"true".into()).unwrap().try_as_ref().unwrap();

		literal_address == true_address
	}

	// Returns a mutable reference to the tags on this expression value. If the type of this
	// expression doesn't support tags, `None` is returned.
	//
	// For example, literal numbers can't have tags, whereas function declarations can.
	//
	// This is used, for example in `Declaration::parse` to set the tags on a value after parsing
	// them before the declaration name.
	//
	// # Returns
	// A mutable reference to the tags on this expression, or `None` if this expression doesn't
	// support tags.
	pub fn tags_mut(&mut self) -> Option<&mut TagList> {
		match self {
			Self::FunctionDeclaration(function) => Some(&mut function.tags),
			_ => None,
		}
	}
}

pub trait Type {
	fn get_type(tokens: &mut VecDeque<Token>, context: &mut Context) -> anyhow::Result<Expression>;
}
