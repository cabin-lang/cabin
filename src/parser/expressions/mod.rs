use colored::Colorize as _;
use run::{RunExpression, RuntimeableExpression};
use try_as::traits as try_as_traits;

use crate::{
	api::{context::Context, traits::TryAs as _},
	bail_err,
	comptime::{memory::VirtualPointer, CompileTime},
	lexer::Span,
	mapped_err,
	parser::{
		expressions::{
			block::Block, field_access::FieldAccess, foreach::ForEachLoop, function_call::FunctionCall, if_expression::IfExpression, literal::LiteralObject, name::Name,
			object::ObjectConstructor, operators::BinaryExpression,
		},
		statements::tag::TagList,
		Parse, TokenQueue,
	},
	transpiler::TranspileToC,
};

pub mod block;
pub mod either;
pub mod field_access;
pub mod foreach;
pub mod function_call;
pub mod function_declaration;
pub mod group;
pub mod if_expression;
pub mod literal;
pub mod name;
pub mod object;
pub mod oneof;
pub mod operators;
pub mod run;
pub mod sugar;

#[derive(Debug, Clone, try_as::macros::From, try_as::macros::TryInto, try_as::macros::TryAsRef)]
pub enum Expression {
	Block(Block),
	FieldAccess(FieldAccess),
	FunctionCall(FunctionCall),
	If(IfExpression),
	Name(Name),
	ObjectConstructor(ObjectConstructor),
	ForEachLoop(ForEachLoop),
	Pointer(VirtualPointer),
	Run(RunExpression),
	Void(()),
}

impl Parse for Expression {
	type Output = Expression;

	fn parse(tokens: &mut TokenQueue, context: &mut Context) -> anyhow::Result<Self::Output> {
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
			Self::FieldAccess(field_access) => field_access
				.evaluate_at_compile_time(context)
				.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating a field access at compile-time".dimmed()))?,
			Self::FunctionCall(function_call) => function_call
				.evaluate_at_compile_time(context)
				.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating a function call at compile-time".dimmed()))?,
			Self::If(if_expression) => if_expression
				.evaluate_at_compile_time(context)
				.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating an if expression at compile-time".dimmed()))?,
			Self::Name(name) => name.clone().evaluate_at_compile_time(context).map_err(mapped_err! {
				while = format!("evaluating the name \"{}\" at compile-time", name.unmangled_name().bold().cyan()),
				context = context,
			})?,
			Self::ObjectConstructor(constructor) => constructor.evaluate_at_compile_time(context).map_err(mapped_err! {
				while = "evaluating a object constructor expression at compile-time",
				context = context,
			})?,
			Self::ForEachLoop(for_loop) => for_loop
				.evaluate_at_compile_time(context)
				.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating a for-each loop at compile-time".dimmed()))?,
			Self::Run(run_expression) => Expression::Run(
				run_expression
					.evaluate_at_compile_time(context)
					.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating a for-each loop at compile-time".dimmed()))?,
			),
			Self::Pointer(pointer) => Expression::Pointer(
				pointer
					.evaluate_at_compile_time(context)
					.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating a pointer compile-time".dimmed()))?,
			),
			Self::Void(_) => self,
		})
	}
}

impl Expression {
	pub fn try_as_literal_or_name<'a>(&self, context: &'a mut Context) -> anyhow::Result<&'a LiteralObject> {
		Ok(match self {
			Self::Pointer(pointer) => pointer.virtual_deref(context),
			Self::Name(name) => name
				.clone()
				.evaluate_at_compile_time(context)
				.map_err(mapped_err! {
					while = format!("evaluating the name \"{}\" at compile-time", name.unmangled_name().bold().cyan()),
					context = context,
				})?
				.try_as_literal_or_name(context)?,
			_ => bail_err! {
				base = format!("A value that's not fully known at compile-time was used as a type; It can only be evaluated into a {} at compile-time.", self.kind_name().bold().yellow()),
				context = context,
			},
		})
	}

	pub fn expect_literal<'a>(&'a self, context: &'a mut Context) -> anyhow::Result<&'a LiteralObject> {
		self.try_as_literal_or_name(context)
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
			Self::FieldAccess(_) => "field access",
			Self::FunctionCall(_) => "function call",
			Self::Name(_) => "name",
			Self::ObjectConstructor(_) => "object constructor",
			Self::Void(_) => "non-existent value",
			Self::Pointer(_) => "pointer",
			Self::If(_) => "if expression",
			Self::ForEachLoop(_) => "for-each loop",
			Self::Run(_) => "run expression",
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
	pub fn try_clone_pointer(&self, context: &Context) -> anyhow::Result<Expression> {
		if let Self::Pointer(address) = self {
			return Ok(Expression::Pointer(*address));
		}

		bail_err! {
			base = format!("A value that's not fully known at compile-time was used as a type; It can only be evaluated into a {}", self.kind_name().bold().yellow()),
			context = context,
		};
	}

	pub fn expect_clone_pointer(&self, context: &Context) -> anyhow::Result<Expression> {
		self.try_clone_pointer(context)
	}

	pub fn is_true(&self, context: &Context) -> bool {
		let Ok(literal_address) = self.try_as::<VirtualPointer>() else {
			return false;
		};

		let true_address = context.scope_data.expect_global_variable("true").expect_as().unwrap();

		literal_address == true_address
	}

	pub fn set_tags(&mut self, tags: TagList, context: &mut Context) {
		match self {
			Self::ObjectConstructor(constructor) => constructor.tags = tags,
			Self::Pointer(pointer) => pointer.virtual_deref_mut(context).tags = tags,
			_ => {},
		};
	}

	/// This function should only be called during parse-time.
	pub fn name_mut(&mut self) -> Option<&mut Name> {
		match self {
			// Self::FunctionDeclaration(function) => Some(&mut function.name),
			// Self::Group(group) => Some(&mut group.name),
			// Self::Either(either) => Some(&mut either.name),
			Self::ObjectConstructor(object) => Some(&mut object.name),
			_ => None,
		}
	}

	/// Returns whether this expression can be assigned to the type pointed to by `target_type`, which is generally
	/// a call to `Typed::get_type()`.
	///
	/// # Parameters
	///
	/// - `target_type` - A pointer to the group declaration that represents the type we are trying to assign to.
	/// - `context` - Global data about the compiler state.
	///
	/// # Returns
	///
	/// whether this expression can be assigned to the given type.
	pub fn is_assignable_to_type(&self, target_type: VirtualPointer, context: &mut Context) -> anyhow::Result<bool> {
		let value_type = self.get_type(context)?.virtual_deref(context).clone();
		value_type.is_type_assignable_to_type(target_type, context)
	}
}

impl TranspileToC for Expression {
	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		Ok(match self {
			Self::If(if_expression) => if_expression.to_c(context)?,
			Self::Block(block) => block.to_c(context)?,
			Self::FieldAccess(field_access) => field_access.to_c(context)?,
			Self::Name(name) => name.to_c(context)?,
			Self::FunctionCall(function_call) => function_call.to_c(context)?,
			Self::ForEachLoop(for_each_loop) => for_each_loop.to_c(context)?,
			Self::Pointer(pointer) => pointer.to_c(context)?,
			Self::ObjectConstructor(object_constructor) => object_constructor.to_c(context)?,
			Self::Run(run_expression) => run_expression.to_c(context)?,
			Self::Void(_) => "void".to_owned(),
		})
	}
}

impl Typed for Expression {
	fn get_type(&self, context: &mut Context) -> anyhow::Result<VirtualPointer> {
		Ok(match self {
			Expression::Pointer(pointer) => pointer.virtual_deref(context).clone().get_type(context)?,
			Expression::FunctionCall(function_call) => function_call.get_type(context)?,
			Expression::Run(run_expression) => run_expression.get_type(context)?,
			Expression::Void(()) => bail_err! {
				base = "Attempted to get the type of a non-existent value",
				while = "getting the type of a generic expression",
				context = context,
			},
			value => {
				dbg!(value);
				todo!()
			},
		})
	}
}

impl Spanned for Expression {
	fn span(&self, context: &Context) -> Span {
		match self {
			Expression::Name(name) => name.span(context),
			Expression::Run(run_expression) => run_expression.span(context),
			Expression::Block(block) => block.span(context),
			Expression::ObjectConstructor(object_constructor) => object_constructor.span(context),
			Expression::Pointer(virtual_pointer) => virtual_pointer.span(context),
			Expression::FunctionCall(function_call) => function_call.span(context),
			Expression::If(if_expression) => if_expression.span(context),
			Expression::FieldAccess(field_access) => field_access.span(context),
			Expression::ForEachLoop(for_each_loop) => for_each_loop.span(context),
			Expression::Void(_) => panic!(),
		}
	}
}

pub trait Typed {
	fn get_type(&self, context: &mut Context) -> anyhow::Result<VirtualPointer>;
}

pub trait Spanned {
	/// Returns the section of the source code that this expression spans. This is used by the compiler to print information about
	/// errors that occur, such as while line and column the error occurred on.
	///
	/// # Parameters
	///
	/// - `context` - Global data about the compiler's state. This is currently only used by the implementation of `Spanned` for
	/// `VirtualPointer`, which uses it to access it's value in virtual memory and return that value's span. See `VirtualPointer::span()`
	/// for more information.
	///
	/// # Returns
	///
	///
	/// The second of the program's source code that this expression spans.
	fn span(&self, context: &Context) -> Span;
}

impl RuntimeableExpression for Expression {
	fn evaluate_subexpressions_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self> {
		Ok(match self {
			Self::FunctionCall(function_call) => Expression::FunctionCall(function_call.evaluate_subexpressions_at_compile_time(context)?),
			_ => bail_err! {
				base = format!("Attempted to use a run-expression on a {}, but forcing this type of expression to run at runtime is pointless.", self.kind_name()),
				context = context,
			},
		})
	}
}
