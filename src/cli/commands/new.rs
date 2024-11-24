use std::path::Path;

use colored::Colorize;

use crate::cli::commands::CabinCommand;
use expression_formatter::{format, println};

/// Creates a new "Hello World" Cabin project.
#[derive(clap::Parser)]
pub struct NewCommand {
	project_name: String,
}

impl CabinCommand for NewCommand {
	fn execute(self) -> anyhow::Result<()> {
		let root_dir = Path::new(&self.project_name);
		println!(r#"{"Creating".bold().green()} a new Cabin project at {format!("{root_dir.display()}").bold().cyan()}"#);
		std::fs::create_dir_all(root_dir)?;
		let root_dir = root_dir.canonicalize()?;

		// Config
		std::fs::write(
			root_dir.join("cabin.toml"),
			unindent::unindent(&format!(
				r#"
				[information]
				name = "{root_dir.components().last().unwrap().as_os_str().to_str().unwrap()}"
				description = "An example cabin project generated with cabin new"
				license = "All rights reserved"
				
				[options]

				[libraries]
				"#
			)),
		)?;

		// Source
		let source_dir = root_dir.join("src");
		std::fs::create_dir_all(&source_dir)?;
		std::fs::write(source_dir.join("main.cabin"), "run terminal.print(\"Hello world!\");")?;

		// Cache
		let cache_dir = root_dir.join("cache");
		std::fs::create_dir_all(cache_dir)?;

		// Builds
		let builds_dir = root_dir.join("builds");
		std::fs::create_dir_all(builds_dir)?;

		// Gitignore
		std::fs::write(root_dir.join(".gitignore"), "builds/\ncache/")?;

		Ok(())
	}
}
