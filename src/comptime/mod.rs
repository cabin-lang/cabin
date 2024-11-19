use crate::context::Context;

pub mod memory;

pub trait CompileTime {
	type Output;

	fn evaluate_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self::Output>;
}
