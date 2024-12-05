use clap::Parser as _;

use crate::cli::commands::{CabinCommand as _, SubCommand};

pub mod api;
pub mod cli;
pub mod compiler;
pub mod comptime;
pub mod lexer;
pub mod parser;
pub mod transpiler;

pub const STDLIB: &str = include_str!("../std/stdlib.cabin");
pub const PRELUDE: &str = include_str!("../std/prelude.cabin");

/// The Cabin compiler.
#[derive(clap::Parser)]
pub struct CabinCompilerArguments {
	/// The command to run.
	#[command(subcommand)]
	pub command: SubCommand,
}

/// The main entry point for the Cabin executable. All this does is delegate the work to one of the various
/// subcommands.
fn main() {
	CabinCompilerArguments::parse().command.execute();
}
