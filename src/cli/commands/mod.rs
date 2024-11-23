use colored::Colorize as _;
use new::NewCommand;
use run::RunCommand;
use set::SetCommand;

use crate::api::context::Context;

pub mod new;
pub mod run;
pub mod set;

#[enum_dispatch::enum_dispatch]
pub trait CabinCommand {
	fn execute(self) -> anyhow::Result<()>;
}

#[derive(clap::Subcommand)]
#[enum_dispatch::enum_dispatch(CabinCommand)]
pub enum SubCommand {
	Run(RunCommand),
	Set(SetCommand),
	New(NewCommand),
}

#[macro_export]
macro_rules! step {
	(
		$expression: expr, $context: expr, $action: expr, $object: expr $(,)?
	) => {{
		use colored::Colorize as _;
		use std::io::Write as _;
		use $crate::api::macros::TerminalOutput as _;

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

		if !$context.config.quiet {
			print!("{}{} {}... ", $context.config.tab(), $action.bold().green(), $object);
			std::io::stdout().flush().unwrap();
			if $action == "Running" {
				println!("\n");
			}
		}

		match $expression {
			Ok(return_value) => {
				if !$context.config.quiet {
					if $action == "Evaluating" && $context.lines_printed != 0 {
						move_cursor_up_and_over($context.lines_printed, ($context.config.tab() + "evaluating abstract syntax tree... ").len());
					}

					if $action != "Running" {
						println!("{}", "Done!".bold().green());
					}

					if $action == "Evaluating" && $context.lines_printed != 0 {
						move_cursor_down_and_left($context.lines_printed, 0);
					}
				}
				return_value
			},

			// Error during this step of compilation
			Err(error) => {
				if $action == "Evaluating" && $context.lines_printed != 0 {
					move_cursor_up_and_over($context.lines_printed, ($context.config.tab() + "evaluating abstract syntax tree... ").len());
				}

				if $action != "Running" {
					println!("{}", "Error:".bold().red());
				}

				if $action == "Evaluating" && $context.lines_printed != 0 {
					move_cursor_down_and_left($context.lines_printed, 0);
				}

				// Print error message
				eprintln!(
					"\n{} {}",
					"Error:".bold().red(),
					if $context.config.quiet {
						format!("{}", error).lines().next().unwrap().to_owned()
					} else {
						format!("{}\n", error)
					}
				);

				// Print error location

				// Print the program
				if let Some(error_position) = $context.error_position() {
					let (error_line, _column) = $context.line_column(error_position);

					eprintln!(
						"In {}{}:",
						format!("{}", $context.running_context.entry_point().display()).bold().cyan(),
						format!(" on {}", format!("line {error_line}").bold().cyan())
					);

					eprintln!("\n\n{}\n", $context.colored_program());
				}

				// Print additional error information
				if !$context.config.quiet {
					if let Some(error_details) = $context.error_details() {
						eprintln!("\n{}\n\n{error_details}", "More information:".bold().bright_blue().underline());
					}
				}

				// Print compiler bug location
				if $context.config.developer_mode {
					println!("{}\n", "Developer Information:".bold().purple().underline());
					println!("{}\n", "This error occurred in the Cabin compiler with the following stack trace:".bold());
					for (index, position) in $context.get_compiler_error_position().iter().enumerate() {
						let trace = format!(
							"{}in {} at {}",
							$context.config.tabs(if index == 0 { 1 } else { 2 }),
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
							$context.config.tabs(2),
							here.function_name().cyan(),
							format!("{}:{}:{}", here.file_name(), here.line(), here.column()).purple()
						)
						.dimmed()
					);
					if !$context.get_compiler_error_position().is_empty() {
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

pub fn start(action: &str, context: &Context) {
	if !context.config.quiet {
		println!(
			"\n{} {}...                    {}",
			action.bold().green(),
			format!("{}", context.running_context.file_or_project_name().display()).bold(),
			"(Run with --quiet or -q to silence this output)".dimmed().italic()
		);
	}
}

pub fn finish() {
	println!("{}", "Done!".bold().green());
}
