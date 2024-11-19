use colored::Colorize;

use crate::{
	context::Context,
	parser::{
		expressions::{object::ObjectConstructor, Expression},
		util::macros::number,
	},
};

pub struct BuiltinFunction {
	evaluate_at_compile_time: fn(&mut Context, usize, &[Expression]) -> anyhow::Result<Expression>,
}

static BUILTINS: phf::Map<&str, BuiltinFunction> = phf::phf_map! {
	"terminal.print" => BuiltinFunction {
		evaluate_at_compile_time: |context: &mut Context, _caller_scope_id: usize, arguments: &[Expression]| {
			let text = arguments.first().ok_or_else(|| anyhow::anyhow!("Missing argument to print"))?.as_literal(context)?.as_string()?;
			println!("{text}");
			Ok(Expression::Void)
		}
	},
	"terminal.input" => BuiltinFunction {
		evaluate_at_compile_time: |context: &mut Context, _caller_scope_id: usize, _arguments: &[Expression]| {
			let mut line = String::new();
			std::io::stdin().read_line(&mut line)?;
			line = line.get(0..line.len() - 1).unwrap().to_owned();
			Ok(Expression::Pointer(ObjectConstructor::from_string(&line, context)))
		}
	},
	"Number.plus" => BuiltinFunction {
		evaluate_at_compile_time: |context: &mut Context, _caller_scope_id: usize, arguments: &[Expression]| {
			let first = arguments.first().ok_or_else(|| anyhow::anyhow!("Missing argument to Number.plus"))?.as_number(context)?;
			let second = arguments.get(1).ok_or_else(|| anyhow::anyhow!("Missing argument to Number.plus"))?.as_number(context)?;
			Ok(number(first + second, context))
		}
	},
};

pub fn call_builtin_at_compile_time(name: &str, context: &mut Context, caller_scope_id: usize, arguments: &[Expression]) -> anyhow::Result<Expression> {
	(BUILTINS
		.get(name)
		.ok_or_else(|| {
			anyhow::anyhow!(
				"Attempted to call the built-in function \"{}\", but no built-in function with that name exists.",
				name.bold().cyan()
			)
		})?
		.evaluate_at_compile_time)(context, caller_scope_id, arguments)
	.map_err(|error| {
		anyhow::anyhow!(
			"{error}\n\t{}",
			format!("while calling the built-in function \"{}\" at compile-time", name.bold().cyan()).dimmed()
		)
	})
}
