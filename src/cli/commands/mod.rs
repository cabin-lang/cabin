use colored::Colorize as _;
use new::NewCommand;
use package::add::AddCommand;
use run::RunCommand;
use set::SetCommand;

use crate::api::context::context;

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
	Add(AddCommand),
}

#[macro_export]
macro_rules! step {
	(
		$expression: expr, $action: expr, $object: expr $(,)?
	) => {{
		use colored::Colorize as _;
		use std::io::Write as _;
		use $crate::api::traits::TerminalOutput as _;

		fn move_cursor_up_and_over(up: usize, right: usize) {
			print!("\x1b[{}A", up);
			print!("\x1b[{}C", right);
			std::io::stdout().flush().unwrap();
		}

		fn move_cursor_down_and_left(down: usize, left: usize) {
			print!("\x1b[{}B", down);
			print!("\x1b[{}D", left);
			std::io::stdout().flush().unwrap();
		}

		let here = $crate::here!();

		if !$crate::api::context::context().config().options().quiet() {
			print!("{}{} {}... ", $crate::api::context::context().config().options().tabs(1), $action.bold().green(), $object);
			std::io::stdout().flush().unwrap();
		}

		match $expression {
			Ok(return_value) => {
				if !$crate::api::context::context().config().options().quiet() {
					if $object.starts_with("compile-time") && $crate::api::context::context().lines_printed != 0 {
						move_cursor_up_and_over($crate::api::context::context().lines_printed, ($crate::api::context::context().config().options().tabs(1) + "evaluating abstract syntax tree... ").len());
					}

					println!("{}", "Done!".bold().green());

					if $object.starts_with("compile-time") && $crate::api::context::context().lines_printed != 0 {
						move_cursor_down_and_left($crate::api::context::context().lines_printed, 0);
					}
				}
				return_value
			},

			// Error during this step of compilation
			Err(error) => {
				if $object.starts_with("compile-time") && $crate::api::context::context().lines_printed != 0 {
					move_cursor_up_and_over($crate::api::context::context().lines_printed, ($crate::api::context::context().config().options().tabs(1) + "evaluating abstract syntax tree... ").len());
				}

				if $action != "Running" {
					println!("{}", "Error:".bold().red());
				}

				if $object.starts_with("compile-time") && $crate::api::context::context().lines_printed != 0 {
					move_cursor_down_and_left($crate::api::context::context().lines_printed, 0);
				}

				// Print error message
				eprintln!(
					"\n{} {}",
					"Error:".bold().red(),
					if $crate::api::context::context().config().options().quiet() {
						format!("{}", error).lines().next().unwrap().to_owned()
					} else {
						format!("{}\n", error)
					}
				);

				// Print the program
				if let Some(error_position) = $crate::api::context::context().error_position() {
					let (error_line, _column) = $crate::api::context::context().line_column(error_position);

					eprintln!(
						"In {}{}:",
						format!("{}", $crate::api::context::context().running_context.entry_point().display()).bold().cyan(),
						format!(" on {}", format!("line {error_line}").bold().cyan())
					);

					eprintln!("\n\n{}\n", $crate::api::context::context().colored_program());
				}

				// Print additional error information
				if !$crate::api::context::context().config().options().quiet() && $crate::api::context::context().config().options().detailed_errors() {
					if let Some(error_details) = $crate::api::context::context().error_details() {
						eprintln!("{}\n\n{error_details}\n", "More information:".bold().bright_blue().underline());
					}
				}

				// Print compiler bug location
				if $crate::api::context::context().config().options().developer_mode() {
					println!("{}\n", "Developer Information:".bold().purple().underline());
					println!("{}\n", "This error occurred in the Cabin compiler with the following stack trace:".bold());
					for (index, position) in $crate::api::context::context().get_compiler_error_position().iter().enumerate() {
						let trace = format!(
							"{}in {} at {}",
							$crate::api::context::context().config().options().tabs(if index == 0 { 1 } else { 2 }),
							position.function_name().cyan(),
							format!("{}:{}:{}", position.file_name(), position.line(), position.column()).purple()
						);
						let trace = if index == 0 { trace } else { format!("{}", trace.dimmed()) };
						println!("{trace}");
					}
					println!(
						"{}",
						format!(
							"{}in {} at {}",
							$crate::api::context::context().config().options().tabs(2),
							here.function_name().cyan(),
							format!("{}:{}:{}", here.file_name(), here.line(), here.column()).purple()
						)
						.dimmed()
					);
					if !$crate::api::context::context().get_compiler_error_position().is_empty() {
						println!();
					}

					println!("{}\n",
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
	}};
}

pub fn start(action: &str) {
	if !context().config().options().quiet() {
		println!(
			"\n{} {}...                    {}",
			action.bold().green(),
			format!("{}", context().running_context.file_or_project_name().display()).bold(),
			"(Run with --quiet or -q to silence this output)".dimmed().italic()
		);
	}
}

pub fn finish() {
	println!("{}", "Done!".bold().green());
}
