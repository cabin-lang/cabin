use crate::{
	api::context::context,
	comptime::memory::VirtualPointer,
	mapped_err,
	parser::{
		expressions::{
			either::Either,
			function_declaration::FunctionDeclaration,
			group::GroupDeclaration,
			literal::{LiteralConvertible, LiteralObject},
			Typed,
		},
		Module,
	},
};

pub trait TranspileToC {
	fn to_c(&self) -> anyhow::Result<String>;
}

pub fn transpile(program: &Module) -> anyhow::Result<String> {
	let mut builder = unindent::unindent(
		"
		#include <stdio.h>
		#include <stdlib.h>
		#include <string.h>

		// Cabin internals -----------------------------------------------------------------------------------------------------------------

		typedef struct {
			char* value;
			int capacity;
		} DynamicString;

		void push_to_dynamic_string(DynamicString* string, char* append) {
			if (strlen(string->value) + strlen(append) > string->capacity) {
				string->capacity *= 2;
				string->value = (char*) realloc(string->value, sizeof(char) * string->capacity);
			}

			strcat(&string->value, append);
		}
		",
	) + "\n";

	// Forward declarations
	builder += &transpile_forward_declarations().map_err(mapped_err! {
		while = "transpiling the program's forward declarations to C",
	})?;

	// Transpile virtual memory
	builder += &transpile_types().map_err(mapped_err! {
		while = "transpiling the program's groups to C",
	})?;

	builder += &transpile_functions().map_err(mapped_err! {
		while = "transpiling the program's functions to C",
	})?;

	builder += &transpile_literals().map_err(mapped_err! {
		while = "transpiling the program's constants to C",
	})?;

	builder += &transpile_program(program).map_err(mapped_err! {
		while = "transpiling the main program to C",
	})?;

	builder += "\n\n\treturn 0;\n}";

	Ok(builder)
}

pub fn transpile_program(program: &Module) -> anyhow::Result<String> {
	let mut builder = String::new();
	for line in program.to_c()?.lines() {
		builder += &format!("\n\t{line}");
	}
	Ok(builder)
}

pub fn transpile_types() -> anyhow::Result<String> {
	let mut builder = String::new();

	for (address, value) in context().virtual_memory.entries() {
		builder += &match value.type_name().unmangled_name() {
			"Group" => {
				let group = GroupDeclaration::from_literal(value).map_err(mapped_err! {
					while = "deserializing a literal in memory to a group",
				})?;
				format!(
					"struct {} {};\n\n",
					value.to_c_type().map_err(mapped_err! {
						while = "transpiling the group into its type name",
					})?,
					group.to_c().map_err(mapped_err! {
						while = "transpiling a group declaration's value to C",
					})?,
				)
			},
			"Either" => {
				let either = Either::from_literal(value)?;
				format!("enum {} {};\n\n", value.name.to_c()?, either.to_c()?,)
			},
			"Object" => {
				let mut builder = format!("struct type_{}_{} {{", value.name.to_c()?, address);

				// Add object fields
				for (field_name, field_value) in value.fields() {
					builder += &format!("\n\t{}* {};", field_value.virtual_deref().get_type()?.virtual_deref().to_c_type()?, field_name.to_c()?);
				}

				// Finish building the string
				builder += "\n};\n\n";
				builder
			},
			_ => String::new(),
		}
	}
	Ok(builder)
}

pub fn transpile_functions() -> anyhow::Result<String> {
	let mut builder = String::new();
	for (address, value) in context().virtual_memory.entries() {
		builder += &match value.type_name().unmangled_name() {
			"Function" => {
				let function = FunctionDeclaration::from_literal(value).map_err(mapped_err! {
					while = "deserializing a function declaration literal into a function declaration",
				})?;
				let value = function.to_c().map_err(mapped_err! {
					while = "transpiling a function declaration expression to C",
				})?;
				if value.is_empty() {
					String::new()
				} else {
					format!("void call_{}_{address}{}\n\n", function.name().to_c()?, value)
				}
			},
			_ => String::new(),
		}
	}
	Ok(builder)
}

pub fn transpile_literals() -> anyhow::Result<String> {
	let mut visited = Vec::new();
	let mut builder = "int main(int argc, char** argv) {".to_owned();

	// Virtual memory
	for (address, value) in context().virtual_memory.entries() {
		if matches!(value.type_name().unmangled_name(), "OneOf" | "Either") {
			continue;
		}

		let mut current_tree = Vec::new();
		builder += &transpile_literal(value, address, &mut visited, &mut current_tree)?;
	}
	Ok(builder)
}

pub fn transpile_literal(value: &LiteralObject, address: VirtualPointer, done: &mut Vec<VirtualPointer>, current_cycle: &mut Vec<VirtualPointer>) -> anyhow::Result<String> {
	// Avoid repetition
	if done.contains(&address) {
		return Ok(String::new());
	}

	if value.type_name() == &"Function".into() {
		let function = FunctionDeclaration::from_literal(value)?;

		// TODO: rah
		if !function.compile_time_parameters().is_empty() {
			return Ok(String::new());
		}
	}

	// Cycle detection
	if current_cycle.contains(&address) {
		current_cycle.push(address);
		anyhow::bail!(
			"Recursive dependency cycle detected: {}",
			current_cycle.iter().map(|pointer| format!("{pointer}")).collect::<Vec<_>>().join(" -> "),
		);
	}
	current_cycle.push(address);

	let mut builder = String::new();

	// Transpile dependencies
	for dependency in value.dependencies() {
		builder += &transpile_literal(dependency.virtual_deref(), dependency, done, current_cycle)?;
	}

	// Transpile self
	let c = {
		let type_name = value.get_type()?.virtual_deref().to_c_type()?;
		format!("{}* {}_{address} = {};\n\n", type_name, value.name.to_c()?, value.to_c()?)
	};

	for line in c.lines() {
		builder += &format!("\n\t{line}");
	}
	done.push(address);

	// Return the string
	Ok(builder)
}

pub fn transpile_forward_declarations() -> anyhow::Result<String> {
	let mut builder = "// Forward declarations -----------------------------------------------------------------------\n\n".to_owned();
	for (address, value) in context().virtual_memory.entries() {
		builder += &match value.type_name().unmangled_name() {
			"Group" => format!("typedef struct {name} {name};\n", name = value.to_c_type()?),
			"Either" => format!("typedef enum either_{name}_{address} {name}_{address};\n", name = value.name.to_c()?),
			"Object" => {
				format!("typedef struct type_{name}_{address} type_{name}_{address};\n", name = value.name.to_c()?)
			},
			_ => String::new(),
		}
	}
	builder += "\n";
	Ok(builder)
}
