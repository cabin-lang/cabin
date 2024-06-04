use crate::{
	context::Context,
	parser::{expressions::Expression, statements::Statement},
};

/// The type tree module, which handles detection of circular dependencies in compile-time code.
pub mod type_tree;

/// The builtin module, which handles running built-in functions at compile-time and transpiling built-in functions to C code.
pub mod builtin;

/// An expression which can be evaluated at compile-time. This is a trait applied to all expressions.
#[enum_dispatch::enum_dispatch]
pub trait CompileTime {
	/// Evaluates the expression at compile-time. This should also recursively evaluate any sub-expressions.
	///
	/// # Parameters
	/// - `context` - The context of the compiler.
	/// - `with_side_effects` - Whether or not to allow side effects in the expression. This is used to prevent side effects in expressions that are
	/// not completely known at compile-time, i.e., a function declaration, which may be called at an unknown time.
	///
	/// # Returns
	/// A `Result` containing either the evaluated expression or an `Error`.
	fn compile_time_evaluate(&self, context: &mut Context, with_side_effects: bool) -> anyhow::Result<Expression>;
}

// Allow calling `to_c` and `c_prelude` on boxed types `Box<T>` when `T` implements `C`
impl<T: CompileTime> CompileTime for Box<T> {
	fn compile_time_evaluate(&self, context: &mut Context, with_side_effects: bool) -> anyhow::Result<Expression> {
		self.as_ref().compile_time_evaluate(context, with_side_effects)
	}
}

/// A statement which can be evaluated at compile-time. This is a trait applied to all statements.
#[enum_dispatch::enum_dispatch]
pub trait CompileTimeStatement {
	/// Evaluates the statement at compile-time. This should also recursively evaluate any sub-statements.
	///
	/// # Parameters
	/// - `context` - The context of the compiler.
	/// - `with_side_effects` - Whether or not to allow side effects in the statement. This is used to prevent side effects in statements that are
	/// not completely known at compile-time, i.e., a function declaration, which may be called at an unknown time.
	///
	/// # Returns
	/// A `Result` containing either the evaluated statement or an `Error`.
	fn compile_time_evaluate_statement(&self, context: &mut Context, with_side_effects: bool) -> anyhow::Result<Statement>;
}

/// A trait indicating that an expression can be converted into C code. This trait is implemented by individual AST nodes to convert themselves into C code. Cabin
/// is a transpiled language, meaning that after lexing and parsing the code, it is transpiled into C before being compiled and run. This trait provides that
/// mechanism. This trait also is `enum_dispatch`ed, meaning `Expression` implements it by calling it on any individual variant; The same goes for `Statement`.
/// Thus, all `Statement`s and `Expression`s must implement this.
#[enum_dispatch::enum_dispatch]
#[ambassador::delegatable_trait]
pub trait TranspileToC {
	/// Converts this AST node into valid C code. This specifically converts the code into the part of the C code that goes where this expression is placed
	/// in the Cabin code. For example, in the expression `constant example: String`, we don't want the `String` here to be a struct declaration, but just a
	/// reference to that struct. To declare the struct beforehand, see `c_prelude()`.
	///
	/// # Parameters
	/// - `context` - The global context of the program. This holds important global information such as the current scope and stored variables of the program.
	///
	/// # Returns
	/// The C code for this expression as a string, or an `Err` if an error occurred when converting this AST node into C code.
	fn to_c(&self, context: &mut Context) -> anyhow::Result<String>;

	/// Generates a "prelude" of C code for this AST node. This is a bit of code that should be declared towards the top of the file that is needed to exist
	/// before using the actual C code of this expression. For example, when making an anonymous table, we need to declare the type as a struct before instantiating it.
	/// This would be where that struct declaration goes. Furthermore, structs that implement this are responsible for also including the prelude of their child
	/// nodes. Even if you have no need for a prelude, still include the prelude your child nodes have, as this is recursively called down the AST.
	///
	/// # Parameters
	/// - `context` - The global context of the program. This holds important global information such as the current scope and stored variables of the program.
	///
	/// # Returns
	/// The C prelude code for this expression as a string, or an `Err` if an error occurred when converting this AST node into C code.
	fn c_prelude(&self, context: &mut Context) -> anyhow::Result<String>;
}

// Allow calling `to_c` and `c_prelude` on boxed types `Box<T>` when `T` implements `C`
impl<T: TranspileToC> TranspileToC for Box<T> {
	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		self.as_ref().to_c(context)
	}

	fn c_prelude(&self, context: &mut Context) -> anyhow::Result<String> {
		self.as_ref().c_prelude(context)
	}
}

// This was driving me crazy trying to find this issue - these need to be declared *after* the traits!
pub use ambassador_impl_TranspileToC;
