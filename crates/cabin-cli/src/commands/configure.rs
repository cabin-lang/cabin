use crate::cli::commands::CabinCommand;

/// Configure the Cabin compiler. If this is run without the `--global` or `-g` flag, it will modify the configuration file (`./cabin.toml`) to include
/// the passed options. When running the cabin compiler for that project, those options should be used. If run with the `--global` flag, this will affect
/// your global compiler settings `~/.config/cabin/cabin.toml`, which are used as a default when always using the compiler
#[derive(clap::Parser)]
pub struct ConfigureCommand {
	/// Run the program in "quiet mode". This prevents the Cabin compiler from outputting debug messages such as
	/// "Tokenizing source code" etc. The only output shown from the Cabin compiler will be errors/warnings/info
	/// generated from your program itself, not static compiler progress messages.
	#[arg(long, short)]
	quiet: bool,

	/// Change the global Cabin configuration. This will edit the configuration in `~/.config/cabin/cabin.toml` instead of a project-specific file, and
	/// this configuration is used as the default when running any cabin code.
	#[arg(long, short)]
	global: bool,
}

impl CabinCommand for ConfigureCommand {
	fn execute(&self) -> anyhow::Result<()> {
		let file_path = if self.global {
			std::env::var("HOME")? + "/.config/cabin/cabin.toml"
		} else {
			"./cabin.toml".to_owned()
		};
		let config_string = std::fs::read_to_string(file_path.clone())
			.map_err(|error| anyhow::anyhow!("Error getting configuration file: {error}. If you were trying to set this option globally, use the --global flag."))?;
		let mut config: toml_edit::DocumentMut = config_string.parse()?;
		let Some(toml_edit::Item::Table(table)) = config.get_mut("options") else {
			anyhow::bail!("Error reading configuration file: Could not find \"options\" table.");
		};

		if self.quiet {
			table.insert("quiet", toml_edit::Item::Value(toml_edit::Value::Boolean(toml_edit::Formatted::new(true))));
		}

		std::fs::write(file_path, config.to_string())?;

		Ok(())
	}
}
