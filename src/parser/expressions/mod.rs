use colored::Colorize as _;
use run::{ParentExpression, RunExpression};
use try_as::traits as try_as_traits;

use crate::{
	api::{context::Context, traits::TryAs as _},
	bail_err,
	comptime::{memory::VirtualPointer, CompileTime},
	lexer::Span,
	mapped_err,
	parser::{
		expressions::{
			block::Block,
			either::Either,
			foreach::ForEachLoop,
			function_call::FunctionCall,
			function_declaration::FunctionDeclaration,
			group::GroupDeclaration,
			if_expression::IfExpression,
			literal::LiteralObject,
			name::Name,
			object::ObjectConstructor,
			oneof::OneOf,
			operators::{BinaryExpression, FieldAccess},
		},
		statements::tag::TagList,
		Parse, TokenQueue,
	},
	transpiler::TranspileToC,
};

pub mod block;
pub mod either;
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
				.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating a name expression at compile-time".dimmed()))?,
			Self::ObjectConstructor(object_constructor) => object_constructor
				.evaluate_at_compile_time(context)
				.map_err(mapped_err! { while = "evaluating an object constructor at compile-time", context = context, })?,
			Self::Group(group) => group
				.evaluate_at_compile_time(context)
				.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating a group declaration at compile-time".dimmed()))?,
			Self::OneOf(oneof) => oneof
				.evaluate_at_compile_time(context)
				.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating a one-of declaration at compile-time".dimmed()))?,
			Self::ForEachLoop(for_loop) => for_loop
				.evaluate_at_compile_time(context)
				.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating a for-each loop at compile-time".dimmed()))?,
			Self::Run(run_expression) => Expression::Run(
				run_expression
					.evaluate_at_compile_time(context)
					.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating a for-each loop at compile-time".dimmed()))?,
			),
			Self::Void(_) | Self::Pointer(_) => self,
		})
	}
}

impl Expression {
	pub fn try_as_literal<'a>(&'a self, context: &'a Context) -> anyhow::Result<&'a LiteralObject> {
		if let Self::Pointer(pointer) = self {
			return Ok(pointer.virtual_deref(context));
		}

		bail_err! {
			base = "A value that's not fully known at compile-time was used as a type",
			context = context,
		};
	}

	pub fn expect_literal<'a>(&'a self, context: &'a Context) -> anyhow::Result<&'a LiteralObject> {
		self.try_as_literal(context)
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
			base = "A value that's not fully known at compile-time was used as a type.",
			context = context,
		};
	}

	pub fn expect_clone_pointer(&self, context: &Context) -> Expression {
		self.try_clone_pointer(context)
			.expect("Attempted to clone a pointer, but the expression to clone wasn't a pointer.")
	}

	pub fn is_true(&self, context: &Context) -> bool {
		let Ok(literal_address) = self.try_as::<VirtualPointer>() else {
			return false;
		};

		let true_address = context.scope_data.expect_global_variable("true").expect_as().unwrap();

		literal_address == true_address
	}

	/// Returns a mutable reference to the tags on this expression value. If the type of this
	/// expression doesn't support tags, `None` is returned.
	///
	/// For example, literal numbers can't have tags, whereas function declarations can.
	///
	/// This is used, for example in `Declaration::parse` to set the tags on a value after parsing
	// them before the declaration name.
	///
	/// This function should only be called during parse-time.
	///
	/// # Returns
	///
	/// A mutable reference to the tags on this expression, or `None` if this expression doesn't
	/// support tags.
	pub fn tags_mut(&mut self) -> Option<&mut TagList> {
		match self {
			Self::FunctionDeclaration(function) => Some(&mut function.tags),
			_ => None,
		}
	}

	/// This function should only be called during parse-time.
	pub fn name_mut(&mut self) -> Option<&mut Name> {
		match self {
			Self::FunctionDeclaration(function) => Some(&mut function.name),
			Self::Group(group) => Some(&mut group.name),
			Self::Either(either) => Some(&mut either.name),
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
			Self::Either(_) | Self::FunctionDeclaration(_) | Self::Group(_) | Self::OneOf(_) => anyhow::bail!("Attempted to transpile a literal to C as an expression"),
		})
	}
}

impl Typed for Expression {
	fn get_type(&self, context: &mut Context) -> anyhow::Result<VirtualPointer> {
		Ok(match self {
			Expression::Pointer(pointer) => pointer.virtual_deref(context).clone().get_type(context)?,
			Expression::FunctionCall(function_call) => function_call.get_type(context)?,
			Expression::Run(run_expression) => run_expression.get_type(context)?,
			value => {
				dbg!(value);
				todo!()
			},
		})
	}
}

impl Spanned for Expression {
	fn span(&self) -> Span {
		match self {
			Expression::Name(name) => name.span(),
			Expression::Run(run_expression) => run_expression.span(),
			Expression::Block(block) => block.span(),
			Expression::Either(either) => either.span(),
			Expression::FieldAccess(field_access) => todo!(),
			Expression::FunctionCall(function_call) => todo!(),
			Expression::Group(group_declaration) => todo!(),
			Expression::FunctionDeclaration(function_declaration) => todo!(),
			Expression::If(if_expression) => todo!(),
			Expression::ObjectConstructor(object_constructor) => todo!(),
			Expression::ForEachLoop(for_each_loop) => todo!(),
			Expression::OneOf(one_of) => todo!(),
			Expression::Pointer(virtual_pointer) => todo!(),
			Expression::Void(_) => panic!(),
		}
	}
}

pub trait Typed {
	fn get_type(&self, context: &mut Context) -> anyhow::Result<VirtualPointer>;
}

pub trait Spanned {
	fn span(&self) -> Span;
}

impl ParentExpression for Expression {
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
