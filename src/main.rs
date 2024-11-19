use comptime::CompileTime as _;
use context::Context;
use lexer::tokenize;
use parser::parse;

pub mod builtin;
pub mod cli;
pub mod comptime;
pub mod context;
pub mod lexer;
pub mod parser;

pub const PRELUDE: &str = include_str!("../std/prelude.cabin");

fn main() -> anyhow::Result<()> {
	let mut context = Context::new();

	let source_code = include_str!("../std/prelude.cabin");
	let mut tokens = tokenize(source_code)?;
	let ast = parse(&mut tokens, &mut context)?;
	let _comptime_ast = ast.evaluate_at_compile_time(&mut context)?;

	Ok(())
}
