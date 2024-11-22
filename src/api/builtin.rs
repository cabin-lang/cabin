use std::collections::VecDeque;

use colored::Colorize as _;

use crate::{
	api::{
		context::Context,
		macros::{number, string},
		traits::TryAs as _,
	},
	comptime::memory::Pointer,
	mapped_err,
	parser::expressions::{object::ObjectConstructor, Expression},
};

pub struct BuiltinFunction {
	evaluate_at_compile_time: fn(&mut Context, usize, Vec<Expression>) -> anyhow::Result<Expression>,
	to_c: fn(&Context, &[String]) -> String,
}

static BUILTINS: phf::Map<&str, BuiltinFunction> = phf::phf_map! {
	"terminal.print" => BuiltinFunction {
		evaluate_at_compile_time: |context: &mut Context, caller_scope_id: usize, arguments: Vec<Expression>| {
			let pointer = VecDeque::from(arguments).pop_front().ok_or_else(|| anyhow::anyhow!("Missing argument to print"))?;
			let returned_object = call_builtin_at_compile_time("Anything.to_string", context, caller_scope_id, vec![pointer])?;
			let string_value = returned_object.try_as_literal(context)?.try_as::<String>()?;

			let mut first_print = false;
			if context.lines_printed == 0 {
				println!("\n");
				first_print = true;
			}

			println!("{string_value}");
			context.lines_printed = string_value.chars().filter(|character| character == &'\n').count() + 3;

			if first_print {
				println!();
				context.lines_printed += 1;
			}

			Ok(Expression::Void(()))
		},
		to_c: |context, parameter_names| {
			let text_address = context.scope_data.expect_global_variable("Text").expect_as::<Pointer>().unwrap().value();
			format!("printf(\"%s\\n\", ((group_u_Text_{text_address}*) {})->internal_value);", parameter_names.first().unwrap())
		}
	},
	"terminal.input" => BuiltinFunction {
		evaluate_at_compile_time: |context: &mut Context, _caller_scope_id: usize, _arguments: Vec<Expression>| {
			let mut line = String::new();
			std::io::stdin().read_line(&mut line)?;
			line = line.get(0..line.len() - 1).unwrap().to_owned();
			Ok(Expression::Pointer(ObjectConstructor::from_string(&line, context)))
		},
		to_c: |context, parameter_names| {
			let return_address = parameter_names.first().unwrap();
			let text_address = context.scope_data.expect_global_variable("Text").expect_as::<Pointer>().unwrap().value();
			format!("char* buffer = malloc(sizeof(char) * 256);\nfgets(buffer, 256, stdin);\n*{return_address} = (group_u_Text_{text_address}) {{ .internal_value = buffer }};")
		}
	},
	"Number.plus" => BuiltinFunction {
		evaluate_at_compile_time: |context: &mut Context, _caller_scope_id: usize, arguments: Vec<Expression>| {
			let first = arguments.first().ok_or_else(|| anyhow::anyhow!("Missing argument to Number.plus"))?.try_as_literal(context)?.expect_as::<f64>()?;
			let second = arguments.get(1).ok_or_else(|| anyhow::anyhow!("Missing argument to Number.plus"))?.try_as_literal(context)?.expect_as::<f64>()?;
			Ok(number(first + second, context))
		},
		to_c: |_context, _parameter_names| {
			String::new()
		}
	},
	"Number.minus" => BuiltinFunction {
		evaluate_at_compile_time: |context: &mut Context, _caller_scope_id: usize, arguments: Vec<Expression>| {
			let first = arguments.first().ok_or_else(|| anyhow::anyhow!("Missing argument to Number.plus"))?.try_as_literal(context)?.expect_as::<f64>()?;
			let second = arguments.get(1).ok_or_else(|| anyhow::anyhow!("Missing argument to Number.plus"))?.try_as_literal(context)?.expect_as::<f64>()?;
			Ok(number(first - second, context))
		},
		to_c: |_context, _parameter_names| {
			String::new()
		}
	},
	"Anything.to_string" => BuiltinFunction {
		evaluate_at_compile_time: |context: &mut Context, _caller_scope_id: usize, arguments: Vec<Expression>| {
			let this = arguments.first().ok_or_else(|| anyhow::anyhow!("Missing argument to Anything.to_string"))?.try_as_literal(context)?;
			Ok(string(&match this.type_name.unmangled_name().as_str() {
				"Number" => this.expect_as::<f64>()?.to_string(),
				"Text" => this.expect_as::<String>()?.to_owned(),
				_ => anyhow::bail!("Unsupported expression: {this:?}")
			}, context))
		},
		to_c: |_context, _parameter_names| {
			String::new()
		}
	},
};

pub fn call_builtin_at_compile_time(name: &str, context: &mut Context, caller_scope_id: usize, arguments: Vec<Expression>) -> anyhow::Result<Expression> {
	(BUILTINS
		.get(name)
		.ok_or_else(|| {
			anyhow::anyhow!(
				"Attempted to call the built-in function \"{}\", but no built-in function with that name exists.",
				name.bold().cyan()
			)
		})?
		.evaluate_at_compile_time)(context, caller_scope_id, arguments)
	.map_err(mapped_err! {
		while = format!("calling the built-in function \"{}\" at compile-time", name.bold().cyan()).dimmed(),
		context = context,
	})
}

pub fn transpile_builtin_to_c(name: &str, context: &Context, parameters: &[String]) -> anyhow::Result<String> {
	Ok((BUILTINS
		.get(name)
		.ok_or_else(|| {
			anyhow::anyhow!(
				"Attempted to call the built-in function \"{}\", but no built-in function with that name exists.",
				name.bold().cyan()
			)
		})?
		.to_c)(context, parameters))
}
