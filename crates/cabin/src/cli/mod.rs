use crate::cli::commands::SubCommand;

/// The theme module, which handles the creation of code themes, which are used to pretty-print code snippets during error messages.
pub mod theme;

/// The commands module, which handles running Cabin subcommands, like `cabin run`, `cabin new`, etc.
pub mod commands;

/// The command-line arguments for the compiler.
#[derive(clap::Parser)]
pub struct CabinCompilerArguments {
	/// The command to run, such as `run` or `build`.
	#[command(subcommand)]
	pub command: SubCommand,
}
