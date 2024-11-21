use colored::Colorize as _;
use run::RunCommand;

use crate::api::context::Context;

pub mod run;

#[enum_dispatch::enum_dispatch]
pub trait CabinCommand {
	fn execute(&self) -> anyhow::Result<()>;
}

#[derive(clap::Subcommand)]
#[enum_dispatch::enum_dispatch(CabinCommand)]
pub enum SubCommand {
	Run(RunCommand),
}

pub fn step<T, E: std::fmt::Display>(expression: Result<T, E>, context: &Context, action: &str, object: &str) -> T {
	use colored::Colorize as _;
	use std::io::Write as _;

	if !context.config.quiet {
		print!("{}{} {}... ", context.config.tab(), action.bold().green(), object);
		std::io::stdout().flush().unwrap();
	}

	match expression {
		Ok(return_value) => {
			if !context.config.quiet {
				println!("{}", "Done!".bold().green());
			}
			return_value
		},

		// Error during this step of compilation
		Err(error) => {
			if !context.config.quiet {
				eprintln!("{}", "Error".bold().red());
			}

			// Print error message
			eprintln!(
				"\n{} {}",
				"Error:".bold().red(),
				if context.config.quiet {
					format!("{}", error).lines().next().unwrap().to_owned()
				} else {
					format!("{}", error)
				}
			);

			// Print error location
			eprintln!(
				"\nThis error occurred in {}{}:\n",
				context.file_name().bold().cyan(),
				if let Some(position) = context.error_position() {
					format!(" on {}", format!("line {}", position.line).bold().cyan())
				} else {
					String::new()
				}
			);

			// Print additional error information
			if !context.config.quiet {
				if let Some(error_details) = context.error_details() {
					eprintln!("\n{}\n\n{error_details}", "More information:".bold().bright_blue().underline());
				}
				println!();
			}

			// Exit
			std::process::exit(1);
		},
	}
}

pub fn start(action: &str, context: &Context) {
	println!("\n{} {}...", action.bold().green(), context.file_name().bold());
}
