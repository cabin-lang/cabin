/// A trait allowing expressions to produce a "type". Cabin is a statically typed language, so it needs the ability at compile-time to determine the type
/// of an expression. This trait allows expressions to convey their type.
#[enum_dispatch::enum_dispatch]
#[ambassador::delegatable_trait]
pub trait Typed {
	/// Returns the type of this expression. This will be called after parsing has finished, so all variables, including those declared
	/// after this one, will be present in scope (in `context.scope_data` specifically).
	///
	/// # Parameters
	/// - `<'declaration>` - The lifetime of the declaration that the `ResolvedType` points to
	/// - `context` - The global Cabin context. This holds global data about the program, and for this particular function, gives access to the different
	/// scopes of the program and the variables declared within those scopes.
	///
	/// # Returns
	/// The type of this expression, or an error if some error occurred when attempting to retrieve the type.
	fn get_type(&self, context: &mut Context) -> anyhow::Result<Literal>;
}

impl<T: Typed> Typed for Box<T> {
	fn get_type(&self, context: &mut Context) -> anyhow::Result<Literal> {
		self.as_ref().get_type(context)
	}
}

pub use ambassador_impl_Typed;

use crate::{context::Context, parser::expressions::literals::Literal};
