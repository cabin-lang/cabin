use crate::{
	context::Context,
	parser::expressions::{literals::LiteralValue, Expression},
};

/// A trait for AST nodes to convert themselves into pretty, human-readable Cabin code. This is used for formatting cabin files, in which Cabin files are
/// parsed and then use this trait to convert themselves into a pretty-string.
#[enum_dispatch::enum_dispatch]
#[ambassador::delegatable_trait]
pub trait ToCabin {
	/// Converts this AST node into pretty, human-readable, cabin code. This should recursively convert any sub-nodes and use their Cabin representations
	/// in the return value of this. This is used for formatting Cabin files, in which the process is essentially just parsing Cabin code into an AST
	/// and then converting the AST into Cabin using this trait.
	fn to_cabin(&self) -> String;
}

impl<T: ToCabin> ToCabin for Box<T> {
	fn to_cabin(&self) -> String {
		self.as_ref().to_cabin()
	}
}

/// A trait for abstract syntax tree (AST) nodes indicating that they can be pretty-printed to the console as syntax-highlighted code. This is used by AST
/// nodes to print errors that print a syntax-highlighted code snippet showing where the error occurred.
#[enum_dispatch::enum_dispatch]
#[ambassador::delegatable_trait]
pub trait ColoredCabin {
	/// Converts this AST node into a colored string of Cabin code. This is used to print pretty errors to the console that show a code snippet where the error
	/// occurred. This can be called at any time, including before compile-time evaluation, so structs that implement this shouldn't worry about doing semantic
	/// analysis, i.e., distinguishing a function identifier from a non-function identifier; The returned code should only really have syntactic highlighting
	/// instead of semantic.
	///
	/// **Do not use tabs to format code in this method. This code will not always appear at the first column in the terminal, meaning tabs can be printed with
	/// varying widths depending on the character the tab starts on. Use 4 spaces instead.**
	///
	/// # Returns
	/// A colored string of Cabin code that is equivalent to (a formatted version of) the source code that was written that was parsed into this AST node.
	fn to_colored_cabin(&self, context: &mut Context) -> String;
}

impl<T: ColoredCabin> ColoredCabin for Box<T> {
	fn to_colored_cabin(&self, context: &mut Context) -> String {
		self.as_ref().to_colored_cabin(context)
	}
}

pub use ambassador_impl_ColoredCabin;
pub use ambassador_impl_ToCabin;
