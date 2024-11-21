use colored::Colorize as _;

use crate::{
	api::{context::Context, macros::number, traits::TryAs as _},
	comptime::{memory::Pointer, CompileTime},
	mapped_err,
	parser::expressions::{function_call::FunctionCall, name::Name, object::ObjectConstructor, operators::FieldAccess, Expression},
	string_literal,
};

pub struct BuiltinFunction {
	evaluate_at_compile_time: fn(&mut Context, usize, &[Expression]) -> anyhow::Result<Expression>,
}

static BUILTINS: phf::Map<&str, BuiltinFunction> = phf::phf_map! {
	"terminal.print" => BuiltinFunction {
		evaluate_at_compile_time: |context: &mut Context, _caller_scope_id: usize, arguments: &[Expression]| {
			let pointer: Pointer = arguments.first().ok_or_else(|| anyhow::anyhow!("Missing argument to print"))?.try_clone_pointer()?.try_into().unwrap();
			let object = pointer.virtual_deref(context);
			let returned_object = FunctionCall {
				function: Box::new(Expression::FieldAccess(FieldAccess {
					left: Box::new(Expression::Pointer(pointer)),
					right: Name::from("to_string"),
					scope_id: object.declared_scope_id()
				})),
				compile_time_arguments: None,
				arguments: None,
				scope_id: object.declared_scope_id()
			}.evaluate_at_compile_time(context)?;
			let string_value = returned_object.try_as_literal(context)?.try_as::<String>()?;
			println!("{string_value}");
			Ok(Expression::Void(()))
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
			let first = arguments.first().ok_or_else(|| anyhow::anyhow!("Missing argument to Number.plus"))?.try_as_literal(context)?.expect_as::<f64>();
			let second = arguments.get(1).ok_or_else(|| anyhow::anyhow!("Missing argument to Number.plus"))?.try_as_literal(context)?.expect_as::<f64>();
			Ok(number(first + second, context))
		}
	},
	"Anything.to_string" => BuiltinFunction {
		evaluate_at_compile_time: |context: &mut Context, _caller_scope_id: usize, arguments: &[Expression]| {
			let this = arguments.first().ok_or_else(|| anyhow::anyhow!("Missing argument to Anything.to_string"))?.try_as_literal(context)?;
			Ok(string_literal!(&match this.type_name.unmangled_name().as_str() {
				"Number" => this.expect_as::<f64>().to_string(),
				_ => todo!()
			}, context))
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
	.map_err(mapped_err! {
		while = format!("calling the built-in function \"{}\" at compile-time", name.bold().cyan()).dimmed(),
		context = context,
	})
}
