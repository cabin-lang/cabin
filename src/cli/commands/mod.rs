use std::{fmt::Display, io::Write as _};

use colored::Colorize as _;
use new::NewCommand;
use run::RunCommand;
use set::SetCommand;

use crate::{
	api::{
		context::{context, Phase},
		traits::TerminalOutput as _,
	},
	debug_start, if_then_some,
};

pub mod new;

/// The package manager module. This module contains the compiler's subcommands for interacting with package management, such as publishing,
/// adding dependencies, removing dependencies, etc.
pub mod package;
pub mod run;
pub mod set;

#[enum_dispatch::enum_dispatch]
pub trait CabinCommand {
	/// Executes this subcommand.
	fn execute(self) -> anyhow::Result<()>;
}

#[derive(clap::Subcommand)]
#[enum_dispatch::enum_dispatch(CabinCommand)]
pub enum SubCommand {
	Run(RunCommand),
	Set(SetCommand),
	New(NewCommand),
}

pub fn step<T, E: Display, F: FnOnce() -> Result<T, E>>(expression: F, phase: Phase) -> T {
	context().phase = phase;

	fn move_cursor_up_and_right(up: usize, right: usize) {
		print!("\x1b[{}A", up);
		print!("\x1b[{}C", right);
		std::io::stdout().flush().unwrap();
	}

	fn move_cursor_down_and_left(down: usize, left: usize) {
		print!("\x1b[{}B", down);
		print!("\x1b[{}D", left);
		std::io::stdout().flush().unwrap();
	}

	let debug_section = if_then_some!(
		context().config().options().debug_info() == "some",
		debug_start!("{} {}", phase.action().0, phase.action().1)
	);

	if !context().config().options().quiet() && context().config().options().debug_info() == "none" {
		print!("{}{} {}... ", context().config().options().tabs(1), phase.action().0.bold().green(), phase.action().1);
		std::io::stdout().flush().unwrap();
	}

	match expression() {
		Ok(return_value) => {
			if !context().config().options().quiet() && context().config().options().debug_info() == "none" {
				if phase == Phase::CompileTimeEvaluation {
					if phase == Phase::CompileTimeEvaluation && context().lines_printed != 0 {
						move_cursor_up_and_right(
							context().lines_printed + 1,
							(context().config().options().tabs(1) + phase.action().0 + phase.action().1 + "... ").len() + 1,
						);
					}
					if !context().warnings().is_empty() {
						println!("{}", "Warning:".bold().yellow());
					}

					if context().lines_printed != 0 {
						move_cursor_down_and_left(context().lines_printed + 1, 0);
						println!();
					}

					for warning in context().warnings() {
						println!("{} {}\n", "Warning:".bold().yellow(), warning);
					}

					if context().warnings().is_empty() {
						println!("{}", "Done!".bold().green());
					}
				} else {
					println!("{}", "Done!".bold().green());
				}
			}
			if let Some(debug_section) = debug_section {
				debug_section.finish();
			}
			return_value
		},

		// Error during this step of compilation
		Err(error) => {
			if phase == Phase::CompileTimeEvaluation && context().lines_printed != 0 {
				move_cursor_up_and_right(
					context().lines_printed,
					(context().config().options().tabs(1) + phase.action().0 + phase.action().1 + "... ").len() + 1,
				);
			}

			if phase != Phase::RunningBinary {
				println!("{}", "Error:".bold().red());
			}

			if phase != Phase::CompileTimeEvaluation && context().lines_printed != 0 {
				move_cursor_down_and_left(context().lines_printed, 0);
			}

			// Print error message
			eprintln!(
				"\n{} {}",
				"Error:".bold().red(),
				if context().config().options().quiet() {
					format!("{}", error).lines().next().unwrap().to_owned()
				} else {
					format!("{}\n", error)
				}
			);

			// Print the program
			if let Some(error_position) = context().error_position() {
				let (error_line, _column) = context().line_column(error_position);

				eprintln!(
					"In {} on {}:",
					format!("{}", context().running_context.entry_point().display()).bold().cyan(),
					format!("line {error_line}").bold().cyan()
				);

				eprintln!("\n\n{}\n", context().colored_program());
			}

			// Print additional error information
			if !context().config().options().quiet() && context().config().options().detailed_errors() {
				if let Some(error_details) = context().error_details() {
					eprintln!("{}\n\n{error_details}\n", "More information:".bold().bright_blue().underline());
				}
			}

			// Print compiler bug location
			if context().config().options().developer_mode() {
				println!("{}\n", "Developer Information:".bold().purple().underline());
				println!("{}\n", "This error occurred in the Cabin compiler with the following stack trace:".bold());
				for (index, position) in context().get_compiler_error_position().iter().enumerate() {
					let trace = format!(
						"{}in {} at {}",
						context().config().options().tabs(if index == 0 { 1 } else { 2 }),
						position.function_name().cyan(),
						format!("{}:{}:{}", position.file_name(), position.line(), position.column()).purple()
					);
					let trace = if index == 0 { trace } else { format!("{}", trace.dimmed()) };
					println!("{trace}");
				}
				if !context().get_compiler_error_position().is_empty() {
					println!();
				}

				println!(
					"{}\n",
					expression_formatter::format!(
						r#"
						This information is showing because you have the {"developer-mode".yellow().bold()} option set to 
						{"true".bold().cyan()}. If you don't want to see this, either disable developer information manually 
						in your cabin.toml, or automatically by running {"cabin set developer-mode false".bold().green()}.
						"#
					)
					.as_terminal_output()
					.dimmed()
					.italic()
				);
			}

			// Exit
			std::process::exit(1);
		},
	}
}

pub fn start(action: &str) {
	if !context().config().options().quiet() && context().config().options().debug_info() == "none" {
		println!(
			"\n{} {}...    {}",
			action.bold().green(),
			format!("{}", context().running_context.file_or_project_name().display()).bold(),
			"(Run with --quiet or -q to silence this output)".dimmed().italic()
		);
	}
}

pub fn finish() {
	println!("{}", "Done!".bold().green());
}
