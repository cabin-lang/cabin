use std::path::PathBuf;

use crate::{
	api::context::Context,
	cli::commands::{start, CabinCommand},
	compiler::{compile, run_native_executable},
	comptime::CompileTime as _,
	lexer::{tokenize, tokenize_without_prelude},
	parser::{expressions::Expression, parse},
	step,
	transpiler::transpile,
	STDLIB,
};

#[derive(clap::Parser)]
pub struct RunCommand {
	pub path: Option<String>,
}

impl CabinCommand for RunCommand {
	fn execute(self) -> anyhow::Result<()> {
		let path = self.path.map(PathBuf::from).unwrap_or_else(|| std::env::current_dir().unwrap());
		let mut context = Context::new(&path)?;

		// Standard Library
		let mut stdlib_tokens = tokenize_without_prelude(STDLIB, &mut context)?;
		let stdlib_ast = parse(&mut stdlib_tokens, &mut context)?;
		let evaluated_stdlib = stdlib_ast.evaluate_at_compile_time(&mut context)?;
		let stdlib_module = evaluated_stdlib.into_literal(&mut context).store_in_memory(&mut context);
		context.scope_data.declare_new_variable("cabin", Expression::Pointer(stdlib_module))?;

		// User code
		start("Running", &context);
		let source_code = step!(std::fs::read_to_string(context.running_context.entry_point()), &context, "Reading", "source file");
		let mut tokens = step!(tokenize(&source_code, &mut context), &context, "Tokenizing", "source code");
		let ast = step!(parse(&mut tokens, &mut context), &context, "Parsing", "token stream");
		let comptime_ast = step!(ast.evaluate_at_compile_time(&mut context), &context, "Evaluating", "compile-time code");
		let c_code = step!(transpile(&comptime_ast, &mut context), &context, "Transpiling", "evaluated AST to C");

		std::fs::write("../output.c", &c_code)?;

		let binary_location = step!(compile(&c_code), &context, "Compiling", "generated C code");
		step!(run_native_executable(binary_location), &context, "Running", "compiled executable");

		Ok(())
	}
}
