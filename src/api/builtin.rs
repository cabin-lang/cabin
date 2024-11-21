use std::collections::VecDeque;

use colored::Colorize as _;

use crate::{
	api::{context::Context, macros::number, traits::TryAs as _},
	mapped_err,
	parser::expressions::{object::ObjectConstructor, Expression},
};

use super::macros::string;

pub struct BuiltinFunction {
	evaluate_at_compile_time: fn(&mut Context, usize, Vec<Expression>) -> anyhow::Result<Expression>,
	to_c: fn(&[String]) -> String,
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
		to_c: |parameter_names| {
			format!("printf(\"%s\", {});", parameter_names.first().unwrap())
		}
	},
	"terminal.input" => BuiltinFunction {
		evaluate_at_compile_time: |context: &mut Context, _caller_scope_id: usize, _arguments: Vec<Expression>| {
			let mut line = String::new();
			std::io::stdin().read_line(&mut line)?;
			line = line.get(0..line.len() - 1).unwrap().to_owned();
			Ok(Expression::Pointer(ObjectConstructor::from_string(&line, context)))
		},
		to_c: |parameter_names| {
			let return_address = parameter_names.first().unwrap();
			format!("char* buffer;\nsize_t length;\ngetline(&buffer, &size, stdin);\n*{return_address} = buffer;")
		}
	},
	"Number.plus" => BuiltinFunction {
		evaluate_at_compile_time: |context: &mut Context, _caller_scope_id: usize, arguments: Vec<Expression>| {
			let first = arguments.first().ok_or_else(|| anyhow::anyhow!("Missing argument to Number.plus"))?.try_as_literal(context)?.expect_as::<f64>();
			let second = arguments.get(1).ok_or_else(|| anyhow::anyhow!("Missing argument to Number.plus"))?.try_as_literal(context)?.expect_as::<f64>();
			Ok(number(first + second, context))
		},
		to_c: |parameter_names| {
			format!("printf(\"%s\", {});", parameter_names.first().unwrap())
		}
	},
	"Number.minus" => BuiltinFunction {
		evaluate_at_compile_time: |context: &mut Context, _caller_scope_id: usize, arguments: Vec<Expression>| {
			let first = arguments.first().ok_or_else(|| anyhow::anyhow!("Missing argument to Number.plus"))?.try_as_literal(context)?.expect_as::<f64>();
			let second = arguments.get(1).ok_or_else(|| anyhow::anyhow!("Missing argument to Number.plus"))?.try_as_literal(context)?.expect_as::<f64>();
			Ok(number(first - second, context))
		},
		to_c: |parameter_names| {
			format!("printf(\"%s\", {});", parameter_names.first().unwrap())
		}
	},
	"Anything.to_string" => BuiltinFunction {
		evaluate_at_compile_time: |context: &mut Context, _caller_scope_id: usize, arguments: Vec<Expression>| {
			let this = arguments.first().ok_or_else(|| anyhow::anyhow!("Missing argument to Anything.to_string"))?.try_as_literal(context)?;
			Ok(string(&match this.type_name.unmangled_name().as_str() {
				"Number" => this.expect_as::<f64>().to_string(),
				"Text" => this.expect_as::<String>().to_owned(),
				_ => anyhow::bail!("Unsupported expression: {this:?}")
			}, context))
		},
		to_c: |parameter_names| {
			let this = parameter_names.first().unwrap();
			let return_address = parameter_names.get(1).unwrap();
			format!("*{return_address} = {this};")
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

pub fn transpile_builtin_to_c(name: &str, parameters: &[String]) -> anyhow::Result<String> {
	Ok((BUILTINS
		.get(name)
		.ok_or_else(|| {
			anyhow::anyhow!(
				"Attempted to call the built-in function \"{}\", but no built-in function with that name exists.",
				name.bold().cyan()
			)
		})?
		.to_c)(parameters))
}
