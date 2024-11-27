use colored::Colorize;

use crate::{api::context::Context, cli::commands::CabinCommand};

/// Sets a compiler option and stores it in the project's cabin.toml.
#[derive(clap::Parser)]
pub struct SetCommand {
	option: String,
	value: String,
}

impl CabinCommand for SetCommand {
	fn execute(self) -> anyhow::Result<()> {
		let mut context = Context::new(&std::env::current_dir().unwrap())?;
		println!();

		context.cabin_toml_mut()?.options_mut().try_set(&self.option, &self.value)?;

		println!("Set option {} to {}\n", self.option.bold().yellow(), self.value.bold().cyan());

		Ok(())
	}
}
