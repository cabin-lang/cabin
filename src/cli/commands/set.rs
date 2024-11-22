use colored::Colorize;

use crate::{
	api::context::Context,
	cli::{commands::CabinCommand, RunningContext},
	compiler_message,
};

enum CompilerOptionType {
	Boolean,
	String,
}

pub struct CompilerOption {
	name: &'static str,
	choices: &'static [&'static str],
	variant: CompilerOptionType,
	default: &'static str,
}

impl CompilerOption {
	pub const fn string(name: &'static str) -> CompilerOption {
		CompilerOption {
			name,
			choices: &[],
			variant: CompilerOptionType::String,
			default: "",
		}
	}

	pub const fn boolean(name: &'static str) -> CompilerOption {
		CompilerOption {
			name,
			choices: &["true", "false"],
			variant: CompilerOptionType::Boolean,
			default: "",
		}
	}

	pub const fn choose(mut self, values: &'static [&'static str]) -> Self {
		self.choices = values;
		self
	}

	pub const fn default(mut self, default: &'static str) -> Self {
		self.default = default;
		self
	}

	pub fn parse(&self, value: &str) -> anyhow::Result<toml_edit::Item> {
		Ok(match self.variant {
			CompilerOptionType::Boolean => value.parse::<bool>()?.into(),
			CompilerOptionType::String => {
				if !self.choices.contains(&value) {
					anyhow::bail!(
						"Invalid value passed to option \"{}\": \"{}\". Valid values for this option are {}.",
						self.name.bold().yellow(),
						value.bold().red().underline(),
						self.choices.iter().map(|choice| format!("{}", choice.bold().cyan())).collect::<Vec<_>>().join(", ")
					)
				}
				value.into()
			},
		})
	}
}

static OPTIONS: phf::Map<&'static str, CompilerOption> = phf::phf_map! {
	"quiet" => CompilerOption::boolean("quiet").default("false"),
	"developer-mode" => CompilerOption::boolean("developer-mode").default("false")
};

#[derive(clap::Parser)]
pub struct SetCommand {
	option: String,
	value: String,
}

impl CabinCommand for SetCommand {
	fn execute(self) -> anyhow::Result<()> {
		let mut context = Context::new(&std::env::current_dir().unwrap())?;

		let RunningContext::Project(project) = &mut context.running_context else {
			anyhow::bail!(compiler_message!(
				"
				{} The {} command can only be used from within a Cabin project. No cabin.toml was found in the current directory.
				",
				"Error:".bold().red(),
				"set".bold().cyan()
			));
		};

		let options = project
			.config
			.get_mut("options")
			.ok_or_else(|| anyhow::anyhow!("No options found in cabin.toml"))?
			.as_table_mut()
			.unwrap();

		// Get the option
		let option = OPTIONS
			.get(&self.option)
			.ok_or_else(|| anyhow::anyhow!("{} No compiler option called \"{}\" exists.", "Error:".red().bold(), self.option.bold().red().underline()))?;

		// Reset to default
		if self.value == "default" {
			options.remove(option.name);
			project.write_config();
			println!("\nReset option {} to it's default value ({})\n", self.option.bold().yellow(), option.default.bold().cyan());
			return Ok(());
		}

		let Ok(value) = option.parse(&self.value) else {
			eprintln!(
				"\n{} \"{}\" is an invalid value for the option {}.\n",
				"Error:".bold().red(),
				self.value.bold().red().underline(),
				option.name.bold().yellow()
			);
			return Ok(());
		};

		options.insert(option.name, value);
		project.write_config();

		println!("\nSet option {} to {}\n", self.option.bold().yellow(), self.value.bold().cyan());

		Ok(())
	}
}
