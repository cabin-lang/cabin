use crate::{
	api::context::Context,
	mapped_err,
	parser::{
		expressions::{function_declaration::FunctionDeclaration, literal::LiteralConvertible, name::NameOption, Expression},
		Program,
	},
};

pub trait TranspileToC {
	fn to_c(&self, context: &mut Context) -> anyhow::Result<String>;
}

pub fn transpile(program: &Program, context: &mut Context) -> anyhow::Result<String> {
	let mut builder = "#include <stdio.h>\n#include <stdlib.h>\n\n".to_string();

	// Forward declarations
	builder += &transpile_forward_declarations(context).map_err(mapped_err! {
		while = "transpiling the program's forward declarations to C",
		context = context,
	})?;

	// Transpile virtual memory
	builder += &transpile_virtual_memory(context).map_err(mapped_err! {
		while = "transpiling the program's functions and groups to C",
		context = context,
	})?;

	builder += &transpile_main(context).map_err(mapped_err! {
		while = "transpiling the program's constants to C",
		context = context,
	})?;

	builder += &transpile_program(program, context).map_err(mapped_err! {
		while = "transpiling the main program to C",
		context = context,
	})?;

	builder += "\n\n\treturn 0;\n}";

	Ok(builder)
}

pub fn transpile_program(program: &Program, context: &mut Context) -> anyhow::Result<String> {
	let mut builder = String::new();
	for line in program.to_c(context)?.lines() {
		builder += &format!("\n\t{line}");
	}
	Ok(builder)
}

pub fn transpile_virtual_memory(context: &mut Context) -> anyhow::Result<String> {
	let mut builder = String::new();
	for (address, value) in context.virtual_memory.entries() {
		builder += &match value.type_name.unmangled_name().as_str() {
			"Group" => format!("struct {} {};\n\n", value.name.to_c_or_pointer(address), value.to_c(context)?),
			"Function" => {
				let function = FunctionDeclaration::from_literal(&value, context).map_err(mapped_err! {
					while = "deserializing a function declaration literal into a function declaration",
					context = context,
				})?;
				format!(
					"{} {}{}\n\n",
					function.return_type.as_ref().unwrap_or(&Box::new(Expression::Void(()))).to_c(context)?,
					function.name.to_c_or_pointer(address),
					function.to_c(context)?
				)
			},
			_ => String::new(),
		}
	}
	Ok(builder)
}

pub fn transpile_main(context: &mut Context) -> anyhow::Result<String> {
	let mut builder = "int main(int argc, char** argv) {".to_owned();
	for (address, value) in context.virtual_memory.entries() {
		match value.type_name.unmangled_name().as_str() {
			"Group" | "Function" => continue,
			_ => {},
		};

		let c = format!("void* {} = &{};\n\n", value.name.to_c_or_pointer(address), value.to_c(context)?);
		for line in c.lines() {
			builder += &format!("\n\t{line}");
		}
	}
	Ok(builder)
}

pub fn transpile_forward_declarations(context: &Context) -> anyhow::Result<String> {
	let mut builder = String::new();
	for (address, value) in context.virtual_memory.entries() {
		builder += &match value.type_name.unmangled_name().as_str() {
			"Group" => format!("typedef struct {name} {name};\n", name = value.name.to_c_or_pointer(address)),
			_ => String::new(),
		}
	}
	builder += "\n";
	Ok(builder)
}
