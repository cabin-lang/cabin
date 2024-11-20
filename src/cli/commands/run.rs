use super::CabinCommand;
use crate::{comptime::CompileTime, context::Context, lexer::tokenize, parser::parse, step};

#[derive(clap::Parser)]
pub struct RunCommand {
	pub filename: String,
}

impl CabinCommand for RunCommand {
	fn execute(&self) -> anyhow::Result<()> {
		let mut context = Context::new(&self.filename);
		println!();

		let source_code = step!(std::fs::read_to_string(&self.filename), context, "Reading", "source file");
		let mut tokens = step!(tokenize(&source_code), context, "Tokenizing", "source code");
		let ast = step!(parse(&mut tokens, &mut context), context, "Parsing", "token stream");
		let _comptime_ast = step!(ast.evaluate_at_compile_time(&mut context), context, "Evaluating", "abstract syntax tree");

		Ok(())
	}
}
