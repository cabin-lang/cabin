use std::path::Path;

use colored::Colorize;

use super::CabinCommand;

#[derive(clap::Parser)]
pub struct NewCommand {
	project_name: String,
}

impl CabinCommand for NewCommand {
	fn execute(self) -> anyhow::Result<()> {
		let root_dir = Path::new(&self.project_name);
		println!("{} a new Cabin project at {}", "Creating".bold().green(), format!("{}", root_dir.display()).bold().cyan());
		std::fs::create_dir_all(root_dir)?;
		let root_dir = root_dir.canonicalize()?;

		// Config
		std::fs::write(
			root_dir.join("cabin.toml"),
			format!(
				"[information]\nname = \"{}\"\n\n[options]",
				root_dir.components().last().unwrap().as_os_str().to_str().unwrap()
			),
		)?;

		// Source
		let source_dir = root_dir.join("src");
		std::fs::create_dir_all(&source_dir)?;
		std::fs::write(source_dir.join("main.cabin"), "run terminal.print(\"Hello world!\");")?;

		Ok(())
	}
}
