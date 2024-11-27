use colored::Colorize;

use crate::{api::context::context, cli::commands::CabinCommand};

/// Sets a compiler option and stores it in the project's cabin.toml.
#[derive(clap::Parser)]
pub struct SetCommand {
	option: String,
	value: String,
}

impl CabinCommand for SetCommand {
	fn execute(self) -> anyhow::Result<()> {
		println!();

		context().cabin_toml_mut()?.options_mut().try_set(&self.option, &self.value)?;

		println!("Set option {} to {}\n", self.option.bold().yellow(), self.value.bold().cyan());

		Ok(())
	}
}
