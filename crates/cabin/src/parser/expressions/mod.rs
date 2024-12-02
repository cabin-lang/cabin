use std::fmt::Debug;

use crate::{api::context::context, cli::theme::Styled, debug_log, parser::expressions::try_as_traits::TryAsMut};
use colored::Colorize as _;
use either::Either;
use group::GroupDeclaration;
use literal::LiteralConvertible;
use match_expression::Match;
use parameter::Parameter;
use represent_as::RepresentAs;
use run::{RunExpression, RuntimeableExpression};
use try_as::traits as try_as_traits;
use unary::UnaryOperation;

use crate::{
	api::traits::TryAs as _,
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
pub mod match_expression;
pub mod name;
pub mod object;
pub mod oneof;
pub mod operators;
pub mod parameter;
pub mod represent_as;
pub mod run;
pub mod sugar;
pub mod unary;

#[derive(Clone, try_as::macros::From, try_as::macros::TryInto, try_as::macros::TryAsRef, try_as::macros::TryAsMut)]
pub enum Expression {
	Block(Block),
	FieldAccess(FieldAccess),
	FunctionCall(FunctionCall),
	If(IfExpression),
	Match(Match),
	Name(Name),
	ObjectConstructor(ObjectConstructor),
	ForEachLoop(ForEachLoop),
	Pointer(VirtualPointer),
	Run(RunExpression),
	Unary(UnaryOperation),
	Parameter(Parameter),
	RepresentAs(RepresentAs),
	Void(()),
}

impl Parse for Expression {
	type Output = Expression;

	fn parse(tokens: &mut TokenQueue) -> anyhow::Result<Self::Output> {
		BinaryExpression::parse(tokens)
	}
}

impl CompileTime for Expression {
	type Output = Expression;

	fn evaluate_at_compile_time(self) -> anyhow::Result<Self::Output> {
		Ok(match self {
			Self::Block(block) => block
				.evaluate_at_compile_time()
				.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating a block at compile-time".dimmed()))?,
			Self::FieldAccess(field_access) => field_access
				.evaluate_at_compile_time()
				.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating a field access at compile-time".dimmed()))?,
			Self::FunctionCall(function_call) => function_call
				.evaluate_at_compile_time()
				.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating a function call at compile-time".dimmed()))?,
			Self::If(if_expression) => if_expression
				.evaluate_at_compile_time()
				.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating an if expression at compile-time".dimmed()))?,
			Self::RepresentAs(represent_as) => Expression::RepresentAs(represent_as.evaluate_at_compile_time().map_err(mapped_err! {
				while = "evaluating a represent-as expression at compile-time",
			})?),
			Self::Name(name) => name.clone().evaluate_at_compile_time().map_err(mapped_err! {
				while = format!("evaluating the name \"{}\" at compile-time", name.unmangled_name().bold().cyan()),
			})?,
			Self::ObjectConstructor(constructor) => constructor.evaluate_at_compile_time().map_err(mapped_err! {
				while = "evaluating a object constructor expression at compile-time",
			})?,
			Self::Match(match_expression) => match_expression.evaluate_at_compile_time().map_err(mapped_err! {
				while = "evaluating a match expression at compile-time",
			})?,
			Self::Parameter(parameter) => Expression::Parameter(parameter.evaluate_at_compile_time().map_err(mapped_err! {
				while = "evaluating a parameter expression at compile-time",
			})?),
			Self::Unary(unary) => unary.evaluate_at_compile_time().map_err(mapped_err! {
				while = "evaluating a unary operation expression at compile-time",
			})?,
			Self::ForEachLoop(for_loop) => for_loop
				.evaluate_at_compile_time()
				.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating a for-each loop at compile-time".dimmed()))?,
			Self::Run(run_expression) => Expression::Run(
				run_expression
					.evaluate_at_compile_time()
					.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating a for-each loop at compile-time".dimmed()))?,
			),
			Self::Pointer(pointer) => Expression::Pointer(
				pointer
					.evaluate_at_compile_time()
					.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while evaluating a pointer compile-time".dimmed()))?,
			),
			Self::Void(_) => self,
		})
	}
}

impl Expression {
	pub fn try_as_literal(&self) -> anyhow::Result<&'static LiteralObject> {
		debug_log!("Interpreting {} as a literal", self.kind_name().cyan());
		Ok(match self {
			Self::Pointer(pointer) => pointer.virtual_deref(),
			Self::Name(name) => name
				.clone()
				.evaluate_at_compile_time()
				.map_err(mapped_err! {
					while = format!("evaluating the name \"{}\" at compile-time", name.unmangled_name().bold().cyan()),
				})?
				.try_as_literal()?,
			_ => bail_err! {
				base = format!("A value that's not fully known at compile-time was used as a type; It can only be evaluated into a {} at compile-time.", self.kind_name().bold().yellow()),
			},
		})
	}

	pub fn is_fully_known_at_compile_time(&self) -> anyhow::Result<bool> {
		Ok(match self {
			Self::Pointer(_) => true,
			Self::Parameter(_) => true,
			Self::Name(name) => name
				.clone()
				.evaluate_at_compile_time()
				.map_err(mapped_err! {
					while = format!("evaluating the name \"{}\" at compile-time", name.unmangled_name().bold().cyan()),
				})?
				.is_fully_known_at_compile_time()?,
			_ => false,
		})
	}

	pub fn evaluate_as_type(self) -> anyhow::Result<Expression> {
		Ok(match self {
			Self::Pointer(pointer) => Expression::Pointer(pointer),
			_ => self.evaluate_at_compile_time()?,
		})
	}

	pub const fn is_pointer(&self) -> bool {
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

	pub const fn kind_name(&self) -> &'static str {
		match self {
			Self::Block(_) => "block",
			Self::FieldAccess(_) => "field access",
			Self::FunctionCall(_) => "function call",
			Self::Name(_) => "name",
			Self::ObjectConstructor(_) => "object constructor",
			Self::Unary(_) => "unary operation",
			Self::Void(_) => "non-existent value",
			Self::Pointer(_) => "pointer",
			Self::If(_) => "if expression",
			Self::ForEachLoop(_) => "for-each loop",
			Self::Run(_) => "run expression",
			Self::Parameter(_) => "parameter",
			Self::Match(_) => "match",
			Self::RepresentAs(_) => "represent-as expression",
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
	pub fn try_clone_pointer(&self) -> anyhow::Result<Expression> {
		if let Self::Pointer(address) = self {
			return Ok(Expression::Pointer(*address));
		}

		bail_err! {
			base = format!("A value that's not fully known at compile-time was used as a type; It can only be evaluated into a {}", self.kind_name().bold().yellow()),
		};
	}

	pub fn is_true(&self) -> bool {
		let Ok(literal_address) = self.try_as::<VirtualPointer>() else {
			return false;
		};

		let true_address = context().scope_data.get_variable("true").unwrap().try_as().unwrap();

		literal_address == true_address
	}

	pub fn set_tags(&mut self, tags: TagList) {
		match self {
			Self::ObjectConstructor(constructor) => constructor.tags = tags,
			Self::Pointer(pointer) => pointer.virtual_deref_mut().tags = tags,
			_ => {},
		};
	}

	pub fn try_set_name(&mut self, name: Name) {
		match self {
			Self::ObjectConstructor(object) => object.name = name,
			Self::Pointer(pointer) => {
				let value = pointer.virtual_deref_mut();
				let address = value.address;

				if value.type_name() == &"Group".into() {
					let mut group = GroupDeclaration::from_literal(value).unwrap();
					group.set_name(name);
					*value = group.to_literal();
					value.address = address;
					return;
				}

				if value.type_name() == &"RepresentAs".into() {
					let mut represent_as = RepresentAs::from_literal(value).unwrap();
					represent_as.set_name(name);
					*value = represent_as.to_literal();
					value.address = address;
					return;
				}

				if value.type_name() == &"Either".into() {
					let mut either = Either::from_literal(value).unwrap();
					either.set_name(name);
					*value = either.to_literal();
					value.address = address;
					return;
				}

				value.name = name;
			},
			_ => {},
		}
	}

	pub fn try_set_scope_label(&mut self, name: Name) {
		let scope_id = match self {
			Self::If(if_expression) => Some(if_expression.inner_scope_id()),
			Self::Pointer(pointer) => pointer.virtual_deref().inner_scope_id,
			_ => None,
		};

		if let Some(scope_id) = scope_id {
			context().scope_data.get_scope_mut_from_id(scope_id).set_label(name);
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
	pub fn is_assignable_to_type(&self, target_type: VirtualPointer) -> anyhow::Result<bool> {
		let this_type = self.get_type()?.virtual_deref();
		// TODO:
		this_type.is_type_assignable_to_type(target_type)
	}
}

impl TranspileToC for Expression {
	fn to_c(&self) -> anyhow::Result<String> {
		Ok(match self {
			Self::If(if_expression) => if_expression.to_c()?,
			Self::Block(block) => block.to_c()?,
			Self::FieldAccess(field_access) => field_access.to_c()?,
			Self::Name(name) => name.to_c()?,
			Self::FunctionCall(function_call) => function_call.to_c()?,
			Self::ForEachLoop(for_each_loop) => for_each_loop.to_c()?,
			Self::Pointer(pointer) => pointer.to_c()?,
			Self::ObjectConstructor(object_constructor) => object_constructor.to_c()?,
			Self::Run(run_expression) => run_expression.to_c()?,
			Self::Void(_) => "void".to_owned(),
			_ => todo!(),
		})
	}
}

impl Typed for Expression {
	fn get_type(&self) -> anyhow::Result<VirtualPointer> {
		Ok(match self {
			Expression::Pointer(pointer) => pointer.virtual_deref().get_type()?,
			Expression::FunctionCall(function_call) => function_call.get_type()?,
			Expression::Run(run_expression) => run_expression.get_type()?,
			Expression::Parameter(parameter) => parameter.get_type()?,
			Expression::Void(()) => bail_err! {
				base = "Attempted to get the type of a non-existent value",
				while = "getting the type of a generic expression",
			},
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
			Expression::ObjectConstructor(object_constructor) => object_constructor.span(),
			Expression::Pointer(virtual_pointer) => virtual_pointer.span(),
			Expression::FunctionCall(function_call) => function_call.span(),
			Expression::If(if_expression) => if_expression.span(),
			Expression::FieldAccess(field_access) => field_access.span(),
			Expression::ForEachLoop(for_each_loop) => for_each_loop.span(),
			Expression::Parameter(parameter) => parameter.span(),
			Expression::Match(match_expression) => match_expression.span(),
			Expression::RepresentAs(represent_as) => represent_as.span(),
			Expression::Unary(unary) => unary.span(),
			Expression::Void(_) => todo!(),
		}
	}
}

pub trait Typed {
	fn get_type(&self) -> anyhow::Result<VirtualPointer>;
}

pub trait Spanned {
	/// Returns the section of the source code that this expression spans. This is used by the compiler to print information about
	/// errors that occur, such as while line and column the error occurred on.
	///
	/// # Returns
	///
	/// The second of the program's source code that this expression spans.
	fn span(&self) -> Span;
}

impl RuntimeableExpression for Expression {
	fn evaluate_subexpressions_at_compile_time(self) -> anyhow::Result<Self> {
		Ok(match self {
			Self::FunctionCall(function_call) => Expression::FunctionCall(function_call.evaluate_subexpressions_at_compile_time()?),
			_ => bail_err! {
				base = format!("Attempted to use a run-expression on a {}, but forcing this type of expression to run at runtime is pointless.", self.kind_name()),
			},
		})
	}
}

impl Debug for Expression {
	fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Block(block) => block.fmt(formatter),
			Self::FieldAccess(field_access) => field_access.fmt(formatter),
			Self::FunctionCall(function_call) => function_call.fmt(formatter),
			Self::ForEachLoop(for_loop) => for_loop.fmt(formatter),
			Self::If(if_expression) => if_expression.fmt(formatter),
			Self::Unary(unary) => unary.fmt(formatter),
			Self::Name(name) => name.fmt(formatter),
			Self::ObjectConstructor(object) => object.fmt(formatter),
			Self::Parameter(parameter) => parameter.fmt(formatter),
			Self::Pointer(pointer) => pointer.fmt(formatter),
			Self::Run(run) => run.fmt(formatter),
			Self::Match(match_expression) => match_expression.fmt(formatter),
			Self::RepresentAs(represent_as) => represent_as.fmt(formatter),
			Self::Void(()) => write!(formatter, "{}", "<void>".style(context().theme.keyword())),
		}
	}
}
