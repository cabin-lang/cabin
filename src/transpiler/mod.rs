use std::collections::HashSet;

use crate::{
	api::context::Context,
	comptime::memory::Pointer,
	mapped_err,
	parser::{
		expressions::{
			function_declaration::FunctionDeclaration,
			literal::{LiteralConvertible, LiteralObject},
			name::NameOption,
			Expression, Type as _,
		},
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
				format!("void {}{}\n\n", function.name.to_c_or_pointer(address), function.to_c(context)?)
			},
			_ => String::new(),
		}
	}
	Ok(builder)
}

pub fn transpile_main(context: &mut Context) -> anyhow::Result<String> {
	let mut builder = "int main(int argc, char** argv) {".to_owned();

	let mut visited = Vec::new();
	for (address, value) in context.virtual_memory.entries() {
		if matches!(value.type_name.unmangled_name().as_str(), "Group" | "Function" | "OneOf" | "Either") {
			continue;
		}

		let mut current_tree = Vec::new();
		builder += &transpile_literal(context, &value, address, &mut visited, &mut current_tree)?;
	}
	Ok(builder)
}

pub fn transpile_literal(context: &mut Context, value: &LiteralObject, address: usize, done: &mut Vec<usize>, current_cycle: &mut Vec<usize>) -> anyhow::Result<String> {
	if matches!(value.type_name.unmangled_name().as_str(), "Group" | "Function" | "OneOf" | "Either") {
		return Ok(String::new());
	}

	// Avoid repetition
	if done.contains(&address) {
		return Ok(String::new());
	}

	// Cycle detection
	if current_cycle.contains(&address) {
		current_cycle.push(address);
		dbg!(current_cycle
			.iter()
			.map(|addr| (addr, Pointer::unchecked(*addr).virtual_deref(context)))
			.collect::<Vec<_>>());
		anyhow::bail!(
			"Recursive dependency cycle detected: {}",
			current_cycle.iter().map(usize::to_string).collect::<Vec<_>>().join(" -> "),
		);
	}
	current_cycle.push(address);

	let mut builder = String::new();

	// Transpile dependencies
	for dependency in value.dependencies() {
		builder += &transpile_literal(context, &dependency.virtual_deref(context).clone(), dependency.value(), done, current_cycle)?;
	}

	// Transpile self
	let c = format!(
		"{}* {} = {};\n\n",
		value.get_type(context)?.to_c(context)?,
		value.name.to_c_or_pointer(address),
		value.to_c(context)?
	);
	for line in c.lines() {
		builder += &format!("\n\t{line}");
	}
	done.push(address);

	// Return the string
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
