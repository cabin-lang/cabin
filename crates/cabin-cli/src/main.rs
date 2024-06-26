use crate::commands::SubCommand;

/// The commands module, which handles running Cabin subcommands, like `cabin run`, `cabin new`, etc.
pub mod commands;

/// Bring the `Parser` trait into scope from `clap`, which allows parsing argument structs from the command line. We assign it to underscore to indicate
/// clearly that it's not used outside of bringing its trait methods into scope.
use clap::Parser as _;

/// The command-line arguments for the compiler.
#[derive(clap::Parser)]
pub struct CabinCompilerArguments {
	/// The command to run, such as `run` or `build`.
	#[command(subcommand)]
	pub command: SubCommand,
}

use crate::cli::{commands::CabinCommand, CabinCompilerArguments};

/// The main entry point for the Cabin compiler. This parses the arguments passed at the command-line, and runs the
/// given subcommand (`run`, `new`, `build`, etc.)
fn main() -> anyhow::Result<()> {
	CabinCompilerArguments::parse().command.execute()
}
