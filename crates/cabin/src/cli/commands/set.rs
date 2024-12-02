use colored::Colorize;

use crate::{api::context::context, cli::commands::CabinCommand};

/// Sets a compiler option and stores it in the project's cabin.toml.
#[derive(clap::Parser)]
pub struct SetCommand {
	option: String,
	value: String,
}

impl CabinCommand for SetCommand {
	fn execute(self) {
		println!();

		context()
			.cabin_toml_mut()
			.unwrap_or_else(|_| {
				eprintln!(
					"{} Options can only be set from within a project; No {} was found.",
					"Error:".bold().red(),
					"cabin.toml".green().bold()
				);
				std::process::exit(1);
			})
			.options_mut()
			.try_set(&self.option, &self.value)
			.unwrap_or_else(|_| {
				eprintln!("{} No option with the name \"{}\" exists.", "Error:".bold().red(), self.option.bold().red().underline());
				std::process::exit(1);
			});

		println!("Set option {} to {}\n", self.option.bold().yellow(), self.value.bold().cyan());
	}
}
