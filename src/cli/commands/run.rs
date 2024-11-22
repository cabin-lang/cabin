use std::path::PathBuf;

use crate::{
	api::context::Context,
	cli::commands::{start, CabinCommand},
	compiler::{compile, run_native_executable},
	comptime::CompileTime as _,
	lexer::tokenize,
	parser::parse,
	step,
	transpiler::transpile,
};

#[derive(clap::Parser)]
pub struct RunCommand {
	pub path: Option<String>,
}

impl CabinCommand for RunCommand {
	fn execute(self) -> anyhow::Result<()> {
		let path = self.path.map(PathBuf::from).unwrap_or_else(|| std::env::current_dir().unwrap());
		let mut context = Context::new(&path)?;
		start("Running", &context);

		let source_code = crate::PRELUDE.to_owned() + "\n\n" + &step!(std::fs::read_to_string(context.running_context.entry_point()), &context, "Reading", "source file");
		let mut tokens = step!(tokenize(&source_code), &context, "Tokenizing", "source code");
		let ast = step!(parse(&mut tokens, &mut context), &context, "Parsing", "token stream");
		let comptime_ast = step!(ast.evaluate_at_compile_time(&mut context), &context, "Evaluating", "abstract syntax tree");
		let c_code = step!(transpile(&comptime_ast, &mut context), &context, "Transpiling", "evaluated AST to C");

		std::fs::write("../output.c", &c_code)?;

		let binary_location = step!(compile(&c_code), &context, "Compiling", "generated C code");
		step!(run_native_executable(binary_location), &context, "Running", "compiled executable");

		Ok(())
	}
}
