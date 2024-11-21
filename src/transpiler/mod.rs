use crate::{api::context::Context, parser::Program};

pub trait TranspileToC {
	fn to_c(&self, context: &Context) -> anyhow::Result<String>;
}

pub fn transpile(program: &Program, context: &Context) -> anyhow::Result<String> {
	let mut builder = "#include <stdio.h>\n#include <stdlib.h>\n\n".to_string();

	for (pointer, value) in context.virtual_memory.entries() {
		builder += &format!("void* POINTER_{pointer} = {};\n\n", value.to_c(context)?);
	}

	builder += "\n\n// User program starts here -----------------------------------------------------------\n\n";

	builder += &program.to_c(context)?;

	Ok(builder)
}
