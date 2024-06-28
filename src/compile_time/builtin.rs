use crate::{
	boolean, global_var, number, object,
	parser::expressions::{literals::object::InternalValue, Expression},
	string, void,
};

use std::{
	path::Path,
	sync::atomic::{AtomicBool, Ordering},
};

use colored::Colorize as _;

/// A builtin function in the language. These are functions that cannot be expressed purely in Cabin code, they need native code from other languages to run.
/// For example, things like printing and taking user input are here. These each have a unique name and are stored in the map `BUILTINS` (see below).
struct BuiltinFunction {
	/// Calls this builtin function at compile-time.
	///
	/// # Parameters
	/// - `expressions` - The arguments passed to the function, as expressions.
	///
	/// # Returns
	/// The return value of the function as an expression, or an `Err` if there was some error when calling the function.
	compile_time: fn(&mut [Expression]) -> anyhow::Result<Expression>,

	/// Converts this builtin function into a C function. Cabin is a transpiled language, so the code is all converted to C code before being compiled. This
	/// means that we need to be able to convert our builtin functions, which have no bodies in the Cabin code, into C functions.
	///
	/// # Parameters
	/// - `parameter_names` - The names of the parameters of the function.
	///
	/// # Returns
	/// The function as a string of valid C code, or an error if there was some error such as getting the `nth` parameter. Note that this does not include the
	/// declaration of the function (such as the name or return type), but only includes the body.
	to_c: fn(&[String]) -> anyhow::Result<String>,
}

/// Whether the program is yet to print to the terminal at compile-time. This is used to print extra spacing around
/// the compile-time prints. This is true iff there has not been any calls to `terminal.print`at compile-time yet.
pub static IS_FIRST_PRINT: AtomicBool = AtomicBool::new(true);

/// The builtin functions of the language. These are annotated in the source code as `#[builtin("name")]`, where `name` is the name of the builtin.
/// This name must be known at compile-time.
static BUILTINS: phf::Map<&'static str, BuiltinFunction> = phf::phf_map! {
	"terminal.print" => BuiltinFunction {
		compile_time: |args| {
			let text = args
				.first()
				.ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the two numbers to add), but no arguments were given", "Number.plus".bold().cyan()))?
				.as_string()?;

			if IS_FIRST_PRINT.load(Ordering::Relaxed) {
				println!("\n\n{text}");
				IS_FIRST_PRINT.store(false, Ordering::Relaxed);
			} else {
				println!("{text}");
			}

			Ok(void!())
		},
		to_c: |parameter_names| {
			Ok(format!("printf(\"%s\\n\", {}->internal_value);", parameter_names.first().unwrap()))
		},
	},
	"terminal.clear" => BuiltinFunction {
		compile_time: |_args| {
			print!("{esc}c", esc = 27 as char);
			Ok(void!())
		},
		to_c: |_parameter_names| {
			Ok(r#"printf("\e[1;1H\e[2J");"#.to_owned())
		},
	},
	"terminal.print_error" => BuiltinFunction {
		compile_time: |args| {
			let text = args
				.first()
				.ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the two numbers to add), but no arguments were given", "Number.plus".bold().cyan()))?
				.as_string()?;

			eprintln!("{text}");
			Ok(void!())
		},
		to_c: |parameter_names| {
			Ok(format!("printf(\"%s\\n\", {}->internal_value);", parameter_names.first().unwrap()))
		},
	},
	"terminal.input" => BuiltinFunction {
		compile_time: |_args| {
			let mut buffer = String::new();
			std::io::stdin().read_line(&mut buffer)?;
			Ok(string!(buffer.trim()))
		},
		to_c: |parameter_names| {
			let return_address = parameter_names
				.first()
				.ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the two numbers to add), but no arguments were given", "Number.plus".bold().cyan()))?;
			Ok(format!("char* buffer = malloc(sizeof(char) * 256);\nfgets(buffer, 256, stdin);\n*{return_address} = (Text_u) {{ .internal_value = buffer }};"))
		},
	},
	"Number.plus" => BuiltinFunction {
		compile_time: |args| {
			let this = args
				.first()
				.ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the two numbers to add), but no arguments were given", "Number.plus".bold().cyan()))?
				.as_number()?;
			let other = args
				.get(1)
				.ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the two numbers to add), but only one argument was given", "Number.plus".bold().cyan()))?
				.as_number()?;

			Ok(object! {
				Number {
					internal_fields = {
						internal_value = InternalValue::Number(this + other)
					}
				}
			})
		},
		to_c: |parameter_names| {
			let this = parameter_names.first().ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the two numbers to add), but no parameter names were given", "Number.plus".bold().cyan()))?;
			let other = parameter_names.get(1).ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the two numbers to add), but only one parameter name was given (\"{this}\")", "Number.plus".bold().cyan()))?;
			Ok(format!("return (Number_u) {{ .internal_value = {this}->internal_value + {other}->internal_value }};"))
		},
	},
	"Number.minus" => BuiltinFunction {
		compile_time: |args| {
			let this = args
				.first()
				.ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the two numbers to add), but no arguments were given", "Number.plus".bold().cyan()))?
				.as_number()?;
			let other = args
				.get(1)
				.ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the two numbers to add), but only one argument was given", "Number.plus".bold().cyan()))?
				.as_number()?;

			Ok(object! {
				Number {
					internal_fields = {
						internal_value = InternalValue::Number(this - other)
					}
				}
			})
		},
		to_c: |parameter_names| {
			let this = parameter_names.first().ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the two numbers to add), but no parameter names were given", "Number.plus".bold().cyan()))?;
			let other = parameter_names.get(1).ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the two numbers to add), but only one parameter name was given (\"{this}\")", "Number.plus".bold().cyan()))?;
			Ok(format!("return (Number_u) {{ .internal_value = {this}->internal_value - {other}->internal_value }};"))
		},
	},
	"Number.times" => BuiltinFunction {
		compile_time: |args| {
			let this = args
				.first()
				.ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the two numbers to add), but no arguments were given", "Number.times".bold().cyan()))?
				.as_number()?;
			let other = args
				.get(1)
				.ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the two numbers to add), but only one argument was given", "Number.times".bold().cyan()))?
				.as_number()?;

			Ok(object! {
				Number {
					internal_fields = {
						internal_value = InternalValue::Number(this * other)
					}
				}
			})
		},
		to_c: |parameter_names| {
			let this = parameter_names.first().ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the two numbers to add), but no parameter names were given", "Number.plus".bold().cyan()))?;
			let other = parameter_names.get(1).ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the two numbers to add), but only one parameter name was given (\"{this}\")", "Number.plus".bold().cyan()))?;
			Ok(format!("return (Number_u) {{ .internal_value = {this}->internal_value * {other}->internal_value }};"))
		},
	},
	"Number.divided_by" => BuiltinFunction {
		compile_time: |args| {
			let this = args
				.first()
				.ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the two numbers to add), but no arguments were given", "Number.divided_by".bold().cyan()))?
				.as_number()?;
			let other = args
				.get(1)
				.ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the two numbers to add), but only one argument was given", "Number.divided_by".bold().cyan()))?
				.as_number()?;

			Ok(object! {
				Number {
					internal_fields = {
						internal_value = InternalValue::Number(this / other)
					}
				}
			})
		},
		to_c: |parameter_names| {
			let this = parameter_names.first().ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the two numbers to add), but no parameter names were given", "Number.plus".bold().cyan()))?;
			let other = parameter_names.get(1).ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the two numbers to add), but only one parameter name was given (\"{this}\")", "Number.plus".bold().cyan()))?;
			Ok(format!("return (Number_u) {{ .internal_value = {this}->internal_value / {other}->internal_value }};"))
		},
	},
	"Number.equals" => BuiltinFunction {
		compile_time: |args| {
			let this = args
				.first()
				.ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the two numbers to compare), but no arguments were given", "Number.equals".bold().cyan()))?
				.as_number()?;
			let other = args
				.get(1)
				.ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the two numbers to compare), but only one argument was given", "Number.equals".bold().cyan()))?
				.as_number()?;

			Ok(if (this - other).abs() < f64::EPSILON {
				global_var!("true")
			} else {
				global_var!("false")
			})
		},
		to_c: |parameter_names| {
			let this = parameter_names.first().ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the two numbers to compare), but no parameter names were given", "Number.equals".bold().cyan()))?;
			let other = parameter_names.get(1).ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the two numbers to compare), but only one parameter name was given (\"{this}\")", "Number.equals".bold().cyan()))?;
			Ok(format!("if ({this}->internal_value == {other}->internal_value) {{\n\treturn true_u;\n}}\nreturn false_u;"))
		},
	},
	"List.length" => BuiltinFunction {
		compile_time: |args| {
			let length = args
				.first_mut()
				.ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes one arguments (the list to get the length of), but no arguments were given", "List.length".bold().cyan()))?
				.as_list()
				.map_err(|_error| anyhow::anyhow!("The argument to \"{}\" must be a list", "List.length".bold().cyan()))?.len();

			Ok(number!(length))
		},
		to_c: |parameter_names| {
			let list = parameter_names.first().ok_or_else(|| anyhow::anyhow!("Expected one argument to List.length, but found none"))?;
			Ok(format!("return (Number_u) {{ .internal_value = {list}->size }};"))
		},
	},
	"List.append" => BuiltinFunction {
		compile_time: |args| {
			let element = args
				.get_mut(1)
				.ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the list to append to and the element to append), but no arguments were given", "List.append".bold().cyan()))?
				.clone();

			let list = args
				.first_mut()
				.ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the list to append to and the element to append), but no arguments were given", "List.append".bold().cyan()))?
				.as_list()
				.map_err(|_error| anyhow::anyhow!("The first argument to \"{}\" must be a list", "List.append".bold().cyan()))?;

			list.push(element);

			Ok(void!())
		},
		// TODO: All lists store heap values
		to_c: |parameter_names| {
			let list = parameter_names.first().ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the list to append to and the element to append), but no parameter names were given", "List.append".bold().cyan()))?;
			let element = parameter_names.get(1).ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the list to append to and the element to append), but only one parameter name was given", "List.append".bold().cyan()))?;
			Ok(format!("if ({list}->size == {list}->capacity) {{\n\t{list}->capacity *= 2;\n}}\n{list}->data = realloc({list}->data, {list}->capacity * sizeof(ElementType_u));\n{list}->data[{list}->size++] = {element};"))
		},
	},
	"List.prepend" => BuiltinFunction {
		compile_time: |args| { // TODO: This
			let element = args
				.get_mut(1)
				.ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the list to append to and the element to append), but no arguments were given", "List.append".bold().cyan()))?
				.clone();

			let list = args
				.first_mut()
				.ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the list to append to and the element to append), but no arguments were given", "List.append".bold().cyan()))?
				.as_list()
				.map_err(|_error| anyhow::anyhow!("The first argument to \"{}\" must be a list", "List.append".bold().cyan()))?;

			let mut new_list = vec![element];
			new_list.append(list);
			*list = new_list;

			Ok(void!())
		},
		to_c: |parameter_names| {
			let list = parameter_names.first().ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the list to append to and the element to append), but no parameter names were given", "List.append".bold().cyan()))?;
			let element = parameter_names.get(1).ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the list to append to and the element to append), but only one parameter name was given", "List.append".bold().cyan()))?;
			Ok(format!("if ({list}->size == {list}->capacity) {{\n\t{list}->capacity *= 2;\n}}\n{list}->data = 1 + realloc({list}->data, {list}->capacity * sizeof(ElementType_u));\n{list}->data[0] = {element};\n{list}->size++;"))
		},
	},
	"List.set" => BuiltinFunction {
		compile_time: |args| {
			let index = args // TODO: this doesn't check that it's an integer, it just rounds down
				.get_mut(1)
				.ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the list to append to and the element to append), but no arguments were given", "List.append".bold().cyan()))?
				.as_number()
				.map_err(|_error| anyhow::anyhow!("The second argument to \"{}\" must be a number.", "List.set".bold().cyan()))? as usize;

			let element = args
				.get_mut(2)
				.ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the list to append to and the element to append), but no arguments were given", "List.append".bold().cyan()))?
				.clone();

			let list = args
				.first_mut()
				.ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the list to append to and the element to append), but no arguments were given", "List.append".bold().cyan()))?
				.as_list()
				.map_err(|_error| anyhow::anyhow!("The first argument to \"{}\" must be a list", "List.append".bold().cyan()))?;

			list[index] = element;

			Ok(void!())
		},
		to_c: |parameter_names| {
			let list = parameter_names.first().ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes three arguments (the list to append, the index to set, and the element to set it to), but no parameter names were given", "List.set".bold().cyan()))?;
			let index = parameter_names.get(1).ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes three arguments (the list to append, the index to set, and the element to set it to), but only one parameter name was given", "List.set".bold().cyan()))?;
			let value = parameter_names.get(2).ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes three arguments (the list to append, the index to set, and the element to set it to), but only two parameter names were given", "List.set".bold().cyan()))?;
			Ok(format!("{list}->data[(int) {index}->internal_value] = {value};"))
		},
	},
	"List.get" => BuiltinFunction {
		#[allow(clippy::as_conversions)]
		compile_time: |args| {
			let index = args // TODO: this doesn't check that it's an integer, it just rounds down
				.get_mut(1)
				.ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the list to append to and the element to append), but no arguments were given", "List.append".bold().cyan()))?
				.as_number()
				.map_err(|_error| anyhow::anyhow!("The second argument to \"{}\" must be a number.", "List.set".bold().cyan()))? as usize;

			let list = args
				.first_mut()
				.ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the list to append to and the element to append), but no arguments were given", "List.append".bold().cyan()))?
				.as_list()
				.map_err(|_error| anyhow::anyhow!("The first argument to \"{}\" must be a list", "List.append".bold().cyan()))?;

			Ok(list.get(index).unwrap().clone()) // TODO: return option for this
		},
		to_c: |parameter_names| {
			let list = parameter_names.first().ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the list to append to and the element to get), but no parameter names were given", "List.get".bold().cyan()))?;
			let index = parameter_names.get(1).ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the list to append to and the element to get), but only one parameter name was given", "List.get".bold().cyan()))?;
			Ok(format!("return (Number_u*) {list}->data[(int) {index}->internal_value];"))
		},
	},
	"File.read" => BuiltinFunction {
		compile_time: |args| {
			let path = args
				.first()
				.ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the list to append to and the element to append), but no arguments were given", "List.append".bold().cyan()))?
				.as_string()
				.map_err(|_error| anyhow::anyhow!("The first argument to \"{}\" must be a list", "List.append".bold().cyan()))?;

			Ok(string!(std::fs::read_to_string(&path).map_err(|error| anyhow::anyhow!("Error reading file: {error}\n\n\t{}", format!("while calling built-in the function \"{}\" at compile-time with the path \"{}\"", "File.read".bold().cyan(), path.bold().cyan()).dimmed()))?))
		},
		to_c: |parameter_names| {
			let path = parameter_names
				.first()
				.ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (The path of the file to read and the return address), but no arguments were given\n", "File.read".bold().cyan()))?;

			let return_address = parameter_names
				.get(1)
				.ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (The path of the file to read and the return address), but only one argument was given\n", "File.read".bold().cyan()))?;

			Ok(unindent::unindent(&format!(
				r#"
				FILE* f = fopen({path}->internal_value, "rb");
				char* buffer = 0;

				if (f) {{
					fseek(f, 0, SEEK_END);
					long length = ftell(f);
					fseek(f, 0, SEEK_SET);
					buffer = malloc(length);
					if (buffer) fread(buffer, 1, length, f);
					fclose(f);
				}}

				*{return_address} = (Text_u) {{ .internal_value = buffer }};
				"#
			)))
		},
	},
	"File.write" => BuiltinFunction {
		compile_time: |args| {
			let path = args
				.first()
				.ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the list to append to and the element to append), but no arguments were given", "List.append".bold().cyan()))?
				.as_string()
				.map_err(|_error| anyhow::anyhow!("The first argument to \"{}\" must be a list", "List.append".bold().cyan()))?;

			let content = args
				.get(1)
				.ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the list to append to and the element to append), but no arguments were given", "List.append".bold().cyan()))?
				.as_string()
				.map_err(|_error| anyhow::anyhow!("The first argument to \"{}\" must be a list", "List.append".bold().cyan()))?;

			std::fs::write(&path, content).map_err(|error| anyhow::anyhow!("Error reading file: {error}\n\n\t{}", format!("while calling built-in the function \"{}\" at compile-time with the path \"{}\"", "File.write".bold().cyan(), path.bold().cyan()).dimmed()))?;

			Ok(void!())
		},
		to_c: |parameter_names| {
			let path = parameter_names
				.first()
				.ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the list to append to and the element to append), but no arguments were given", "List.append".bold().cyan()))?;

			Ok(unindent::unindent(&format!(
				r#"
				FILE* file = fopen({path}->internal_value, "w");
				fprintf(file, "Some text");
				fclose(file);
				"#
			)))
		},
	},
	"File.file_exists" => BuiltinFunction {
		compile_time: |args| {
			let path = args
				.first()
				.ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the list to append to and the element to append), but no arguments were given", "List.append".bold().cyan()))?
				.as_string()
				.map_err(|_error| anyhow::anyhow!("The first argument to \"{}\" must be a list", "List.append".bold().cyan()))?;

			Ok(boolean!(Path::new(&path).is_file()))
		},
		to_c: |parameter_names| {
			let path = parameter_names.first().ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes one arguments (the file path to check if it exists), but no parameter names were given", "File.file_exists".bold().cyan()))?;
			let return_address = parameter_names.get(1).ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes one arguments (the file path to check if it exists), but no parameter names were given", "File.file_exists".bold().cyan()))?;
			Ok(unindent::unindent(&format!(
				"
				struct stat buffer;
				*{return_address} = stat({path}->internal_value, &buffer) == 0;
				"
			)))
		},
	},
	"File.directory_exists" => BuiltinFunction {
		compile_time: |args| {
			let path = args
				.first()
				.ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the list to append to and the element to append), but no arguments were given", "List.append".bold().cyan()))?
				.as_string()
				.map_err(|_error| anyhow::anyhow!("The first argument to \"{}\" must be a list", "List.append".bold().cyan()))?;

			Ok(boolean!(Path::new(&path).is_dir()))
		},
		to_c: |parameter_names| {
			let _path = parameter_names.first().ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes one arguments (the file path to check if it exists), but no parameter names were given", "File.file_exists".bold().cyan()))?;
			Ok(String::new()) // TODO: find a cross platform way to do this
		},

	},

	"Anything.is" => BuiltinFunction {
		compile_time: |args| {
			let Expression::Literal(this) = args
				.first()
				.ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the list to append to and the element to append), but no arguments were given", "List.append".bold().cyan()))? else {
					anyhow::bail!("First argument to Anything.is is not a literal");
				};

			let Expression::Literal(other) = args
				.get(1)
				.ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes two arguments (the list to append to and the element to append), but no arguments were given", "List.append".bold().cyan()))? else {
					anyhow::bail!("Second argument to Anything.is is not a literal");
				};

			// Ok(boolean!(this.is(other)))
			todo!()
		},
		to_c: |parameter_names| {
			let _path = parameter_names.first().ok_or_else(|| anyhow::anyhow!("The function \"{}\" takes one arguments (the file path to check if it exists), but no parameter names were given", "File.file_exists".bold().cyan()))?;
			Ok(String::new()) // TODO: find a cross platform way to do this
		},

	}
};

/// Calls a builtin function with the given name and arguments and returns the result. This should only be used
/// during the compile-time phase of the compiler to call a builtin function at compile-time and return the result.
/// To convert a builtin function to C, see `builtin_to_c`.
///
/// # Parameters
/// - `name` - The name of the builtin function to call.
/// - `args` - The arguments to pass to the builtin function.
///
/// # Returns
/// The return value of the builtin function.
pub fn call_builtin_at_compile_time(name: &str, args: &mut [Expression]) -> anyhow::Result<Expression> {
	BUILTINS
		.get(name)
		.map_or_else(|| anyhow::bail!("Unknown builtin: {name}"), |builtin| (builtin.compile_time)(args))
}

/// Converts a builtin function to C code. This should be used during the transpilation step of the compiler, when converting the
/// Cabin code to C code. To call a builtin function at compile-time, use `call_builtin_at_compile_time`.
///
/// # Parameters
/// - `name` - The name of the builtin function to convert to C. If it is not associated with any builtin, an `Err` is returned.
/// - `parameter_names` - The names of the parameters of the function to convert to C. This is used to generate the parameter
/// names in the C code.
///
/// # Returns
/// The builtin function declaration as a string of valid C code. If the given name is not associated with any known builtin functions,
/// an `Err` is returned. Otherwise, the result is guaranteed to be `Ok()`.
pub fn builtin_to_c(name: &str, parameter_names: &[String]) -> anyhow::Result<String> {
	BUILTINS
		.get(name)
		.map_or_else(
			|| {
				anyhow::bail!(
					"This function has a \"{}\" tag with the value \"{}\", but no built-in function with that name exists",
					"builtin".bold().cyan(),
					name.bold().cyan()
				)
			},
			|builtin| (builtin.to_c)(parameter_names),
		)
		.map_err(|error| anyhow::anyhow!("{error}\n\t{}", format!("while converting the built-in function \"{}\" to C", name.bold().cyan()).dimmed()))
}
