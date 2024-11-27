pub mod memory;

pub trait CompileTime {
	type Output;

	/// Evaluates this AST node at compile-time, as much as possible. For example, for if-expressions, this
	/// will evaluate the condition, and if the condition is fully evaluable at compile-time and resolves to
	/// `true`, it will run the `if` body.
	fn evaluate_at_compile_time(self) -> anyhow::Result<Self::Output>;
}
