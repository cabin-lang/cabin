use crate::{
	api::{context::Context, traits::TryAs},
	comptime::memory::VirtualPointer,
	mapped_err,
	parser::{
		expressions::{
			either::Either,
			function_declaration::FunctionDeclaration,
			group::GroupDeclaration,
			literal::{LiteralConvertible, LiteralObject},
			Expression, Typed,
		},
		Program,
	},
};

pub trait TranspileToC {
	fn to_c(&self, context: &mut Context) -> anyhow::Result<String>;
}

pub fn transpile(program: &Program, context: &mut Context) -> anyhow::Result<String> {
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
	builder += &transpile_forward_declarations(context).map_err(mapped_err! {
		while = "transpiling the program's forward declarations to C",
		context = context,
	})?;

	// Transpile virtual memory
	builder += &transpile_types(context).map_err(mapped_err! {
		while = "transpiling the program's groups to C",
		context = context,
	})?;

	builder += &transpile_functions(context).map_err(mapped_err! {
		while = "transpiling the program's functions to C",
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

pub fn transpile_types(context: &mut Context) -> anyhow::Result<String> {
	let mut builder = String::new();

	for (address, value) in context.virtual_memory.entries() {
		builder += &match value.type_name().unmangled_name().as_str() {
			"Group" => {
				let group = GroupDeclaration::from_literal(&value).map_err(mapped_err! {
					while = "deserializing a literal in memory to a group",
					context = context,
				})?;
				format!(
					"struct {} {};\n\n",
					value.to_c_type(context).map_err(mapped_err! {
						while = "transpiling the group into its type name",
						context = context,
					})?,
					group.to_c(context).map_err(mapped_err! {
						while = "transpiling a group declaration's value to C",
						context = context,
					})?,
				)
			},
			"Either" => {
				let either = Either::from_literal(&value)?;
				format!("enum {} {};\n\n", value.name.to_c(context)?, either.to_c(context)?,)
			},
			"Object" => {
				let mut builder = format!("struct type_{}_{} {{", value.name.to_c(context)?, address);

				// Anything fields
				if value.name != "Anything".into() {
					let anything = GroupDeclaration::from_literal(context.scope_data.expect_global_variable("Anything").clone().expect_literal(context)?)?;
					for field in &anything.fields {
						builder += &format!(
							"\n\t{}* {};",
							if let Some(field_type) = &field.field_type {
								field_type.expect_as::<VirtualPointer>()?.virtual_deref(context).clone().to_c_type(context)?
							} else {
								field
									.value
									.as_ref()
									.unwrap_or(&Expression::Void(()))
									.get_type(context)?
									.virtual_deref(context)
									.clone()
									.to_c_type(context)?
							},
							field.name.to_c(context)?
						);
					}
				}

				// Add object fields
				for (field_name, field_value) in value.fields() {
					builder += &format!(
						"\n\t{}* {};",
						field_value
							.virtual_deref(context)
							.clone()
							.get_type(context)?
							.virtual_deref(context)
							.clone()
							.to_c_type(context)?,
						field_name.to_c(context)?
					);
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

pub fn transpile_functions(context: &mut Context) -> anyhow::Result<String> {
	let mut builder = String::new();
	for (address, value) in context.virtual_memory.entries() {
		builder += &match value.type_name().unmangled_name().as_str() {
			"Function" => {
				let function = FunctionDeclaration::from_literal(&value).map_err(mapped_err! {
					while = "deserializing a function declaration literal into a function declaration",
					context = context,
				})?;
				let value = function.to_c(context).map_err(mapped_err! {
					while = "transpiling a function declaration expression to C",
					context = context,
				})?;
				if value.is_empty() {
					String::new()
				} else {
					format!("void call_{}_{address}{}\n\n", function.name.to_c(context)?, value)
				}
			},
			_ => String::new(),
		}
	}
	Ok(builder)
}

pub fn transpile_main(context: &mut Context) -> anyhow::Result<String> {
	let mut visited = Vec::new();
	let mut builder = "int main(int argc, char** argv) {".to_owned();

	// Anything
	// let anything_pointer = context.scope_data.expect_global_variable("Anything").try_as::<VirtualPointer>()?.to_owned();
	// let anything = anything_pointer.virtual_deref(context).clone();
	// let mut current_tree = Vec::new();
	// builder += &transpile_literal(context, &anything, anything_pointer, &mut visited, &mut current_tree)?;

	// Virtual memory
	for (address, value) in context.virtual_memory.entries() {
		if matches!(value.type_name().unmangled_name().as_str(), "OneOf" | "Either") {
			continue;
		}

		let mut current_tree = Vec::new();
		builder += &transpile_literal(context, &value, address, &mut visited, &mut current_tree)?;
	}
	Ok(builder)
}

pub fn transpile_literal(
	context: &mut Context,
	value: &LiteralObject,
	address: VirtualPointer,
	done: &mut Vec<VirtualPointer>,
	current_cycle: &mut Vec<VirtualPointer>,
) -> anyhow::Result<String> {
	// Avoid repetition
	if done.contains(&address) {
		return Ok(String::new());
	}

	if value.type_name() == &"Function".into() {
		let function = FunctionDeclaration::from_literal(value)?;
		if !function.compile_time_parameters.is_empty() {
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
		builder += &transpile_literal(context, &dependency.virtual_deref(context).clone(), dependency, done, current_cycle)?;
	}

	// Transpile self
	let c = {
		let type_name = value.get_type(context)?.virtual_deref(context).clone().to_c_type(context)?;
		format!("{}* {}_{address} = {};\n\n", type_name, value.name.to_c(context)?, value.to_c(context)?)
	};

	for line in c.lines() {
		builder += &format!("\n\t{line}");
	}
	done.push(address);

	// Return the string
	Ok(builder)
}

pub fn transpile_forward_declarations(context: &mut Context) -> anyhow::Result<String> {
	let mut builder = "// Forward declarations -----------------------------------------------------------------------\n\n".to_owned();
	for (address, value) in context.virtual_memory.entries() {
		builder += &match value.type_name().unmangled_name().as_str() {
			"Group" => format!("typedef struct {name} {name};\n", name = value.to_c_type(context)?),
			"Either" => format!("typedef enum either_{name}_{address} {name}_{address};\n", name = value.name.to_c(context)?),
			"Object" => {
				format!("typedef struct type_{name}_{address} type_{name}_{address};\n", name = value.name.to_c(context)?)
			},
			_ => String::new(),
		}
	}
	builder += "\n";
	Ok(builder)
}
