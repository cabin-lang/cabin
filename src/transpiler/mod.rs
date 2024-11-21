use crate::{
	api::context::Context,
	mapped_err,
	parser::{
		expressions::{function_declaration::FunctionDeclaration, literal::LiteralConvertible, Expression},
		Program,
	},
};

pub trait TranspileToC {
	fn to_c(&self, context: &Context) -> anyhow::Result<String>;
}

pub fn transpile(program: &Program, context: &Context) -> anyhow::Result<String> {
	let mut builder = "#include <stdio.h>\n#include <stdlib.h>\n\n".to_string();

	// Forward declarations
	builder += &transpile_forward_declarations(context).map_err(mapped_err! {
		while = "transpiling the program's forward declarations to C",
		context = context,
	})?;

	// Transpile virtual memory
	builder += &transpile_virtual_memory(context).map_err(mapped_err! {
		while = "transpiling the program's constants to C",
		context = context,
	})?;

	builder += "\n\n// User program starts here -----------------------------------------------------------\n\n";

	builder += &program.to_c(context)?;

	Ok(builder)
}

pub fn transpile_virtual_memory(context: &Context) -> anyhow::Result<String> {
	let mut builder = String::new();
	for (pointer, value) in context.virtual_memory.entries() {
		builder += &match value.type_name.unmangled_name().as_str() {
			"Group" => format!("struct POINTER_{pointer} {};\n\n", value.to_c(context)?),
			"Function" => {
				let function = FunctionDeclaration::from_literal(value, context).map_err(mapped_err! {
					while = "deserializing a function declaration literal into a function declaration",
					context = context,
				})?;
				format!(
					"{} POINTER_{pointer}{}\n\n",
					function.return_type.as_ref().unwrap_or(&Box::new(Expression::Void(()))).to_c(context)?,
					function.to_c(context)?
				)
			},
			_ => format!("void* POINTER_{pointer} = {};\n\n", value.to_c(context)?),
		}
	}
	Ok(builder)
}

pub fn transpile_forward_declarations(context: &Context) -> anyhow::Result<String> {
	let mut builder = String::new();
	for (pointer, value) in context.virtual_memory.entries() {
		builder += &match value.type_name.unmangled_name().as_str() {
			"Group" => format!("typedef struct POINTER_{pointer} POINTER_{pointer};\n"),
			_ => String::new(),
		}
	}
	Ok(builder)
}
