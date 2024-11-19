use std::collections::VecDeque;

use block::Block;
use colored::Colorize;
use either::Either;
use foreach::ForEachLoop;
use function::FunctionDeclaration;
use function_call::FunctionCall;
use group::GroupDeclaration;
use if_expression::IfExpression;
use name::Name;
use object::{LiteralObject, ObjectConstructor};
use oneof::OneOf;
use operators::{BinaryExpression, FieldAccess};

use super::{statements::tag::TagList, Parse};
use crate::{
	comptime::{memory::Pointer, CompileTime},
	context::Context,
	lexer::Token,
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

#[derive(Debug, Clone)]
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

	Void,
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
			Self::Void | Self::Pointer(_) => self,
		})
	}
}

impl Expression {
	pub fn as_literal<'a>(&'a self, context: &'a Context) -> anyhow::Result<&'a LiteralObject> {
		if let Self::Pointer(address) = self {
			return context.virtual_memory.get(*address).ok_or_else(|| anyhow::anyhow!("Invalid pointer"));
		}

		anyhow::bail!("Attempted to coerce a non-literal into a literal");
	}

	pub fn is_literal(&self) -> bool {
		matches!(self, Self::Pointer(_))
	}

	pub fn name(&self) -> &'static str {
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
			Self::Void => "non-existent value",
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

	pub fn as_literal_mut<'a>(&'a self, context: &'a mut Context) -> anyhow::Result<&'a mut LiteralObject> {
		if let Self::Pointer(address) = self {
			return context.virtual_memory.get_mut(*address).ok_or_else(|| anyhow::anyhow!("Invalid pointer"));
		}

		anyhow::bail!("Attempted to coerce a non-literal into a literal");
	}

	pub fn as_literal_address(&self) -> anyhow::Result<Pointer> {
		if let Self::Pointer(address) = self {
			return Ok(*address);
		}

		anyhow::bail!("Attempted to coerce a non-literal into a literal");
	}

	pub fn to_owned_literal(&self) -> anyhow::Result<Expression> {
		if let Self::Pointer(address) = self {
			return Ok(Expression::Pointer(*address));
		}

		anyhow::bail!("Attempted to coerce a non-literal into a literal");
	}

	pub fn as_object(self) -> anyhow::Result<ObjectConstructor> {
		if let Self::ObjectConstructor(object) = self {
			Ok(object)
		} else {
			anyhow::bail!("");
		}
	}

	pub fn as_object_ref(&self) -> anyhow::Result<&ObjectConstructor> {
		if let Self::ObjectConstructor(object) = self {
			Ok(object)
		} else {
			anyhow::bail!("");
		}
	}

	pub fn is_true(&self, context: &Context) -> bool {
		let Ok(literal_address) = self.as_literal_address() else {
			return false;
		};

		let true_address = context.scope_data.get_global_variable(&"true".into()).unwrap().value.as_literal_address().unwrap();

		literal_address == true_address
	}

	pub fn tags(&mut self) -> Option<&mut TagList> {
		match self {
			Self::FunctionDeclaration(function) => Some(&mut function.tags),
			_ => None,
		}
	}

	pub fn as_number(&self, context: &Context) -> anyhow::Result<f64> {
		self.as_literal(context)?.get_internal_field("internal_value").unwrap().as_number()
	}
}

pub trait Type {
	fn get_type(tokens: &mut VecDeque<Token>, context: &mut Context) -> anyhow::Result<Expression>;
}
