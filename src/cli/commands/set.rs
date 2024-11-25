use colored::Colorize;

use crate::{
	api::context::Context,
	cli::{commands::CabinCommand, RunningContext},
};

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

		let RunningContext::Project(project) = &mut context.running_context else {
			anyhow::bail!(expression_formatter::format!(
				r#"
				{"Error:".bold().red()} The {"set".bold().cyan()} command can only be used from within a Cabin project. No cabin.toml was found in the current directory.
				"#
			));
		};

		context.cabin_toml_mut()?.options_mut().try_set(&self.option, &self.value)?;

		println!("Set option {} to {}\n", self.option.bold().yellow(), self.value.bold().cyan());

		Ok(())
	}
}
