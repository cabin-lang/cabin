use clap::Parser as _;

use crate::cli::commands::{CabinCommand as _, SubCommand};

pub mod api;
pub mod cli;
pub mod compiler;
pub mod comptime;
pub mod lexer;
pub mod parser;
pub mod transpiler;

pub const PRELUDE: &str = include_str!("../std/prelude.cabin");

#[derive(clap::Parser)]
pub struct CabinCompilerArguments {
	#[command(subcommand)]
	pub command: SubCommand,
}

fn main() -> anyhow::Result<()> {
	CabinCompilerArguments::parse().command.execute()
}
