use std::process::{Command, Stdio};

use colored::Colorize as _;

use crate::{
	api::context::Context,
	cli::{commands::CabinCommand, RunningContext},
	step,
};

#[derive(clap::Parser)]
pub struct AddCommand {
	library: String,
}

impl CabinCommand for AddCommand {
	fn execute(self) -> anyhow::Result<()> {
		let mut context = Context::new(&std::env::current_dir().unwrap())?;

		let RunningContext::Project(project) = &mut context.running_context else {
			anyhow::bail!(expression_formatter::format!(
				r#"
				{"Error:".bold().red()} The {"add".bold().cyan()} command can only be used from within a Cabin project. No cabin.toml was found in the current directory.
				"#
			));
		};

		let library_name = self.library.split('/').last().unwrap();

		println!();
		expression_formatter::println!(r#"{"Adding".bold().green()} library {library_name.bold().cyan()}..."#);

		std::fs::create_dir_all(project.root_directory().join("cache").join("libraries"))?;

		let output_dir = project.root_directory().join("cache").join("libraries").join(library_name);
		_ = std::fs::remove_dir_all(&output_dir);

		step!(
			(|| {
				let status = Command::new("git")
					.arg("clone")
					.arg("-q")
					.arg(expression_formatter::format!("https://github.com/{self.library}.git"))
					.arg(&output_dir)
					.stderr(Stdio::null())
					.status()?;

				if status.success() {
					Ok(())
				} else {
					anyhow::bail!("Failed to download library code");
				}
			})(),
			context,
			"Downloading",
			"library code"
		);

		let commit = step!(
			String::from_utf8(
				Command::new("git")
					.arg("log")
					.arg("-n")
					.arg("1")
					.arg("--pretty=format:\"%H\"")
					.current_dir(output_dir)
					.output()?
					.stdout,
			),
			context,
			"Getting",
			"version information"
		);

		let commit = commit.get(1..commit.len() - 1).unwrap();

		let libraries = project
			.config
			.get_mut("libraries")
			.ok_or_else(|| anyhow::anyhow!("No libraries found in cabin.toml"))?
			.as_table_mut()
			.unwrap();

		let mut table = toml_edit::InlineTable::new();
		table.insert("repository", self.library.clone().into());
		table.insert("commit", commit.into());

		step!(anyhow::Ok(()), context, "Updating", "cabin.toml");
		libraries.insert(library_name, table.into());
		project.write_config();

		step!(anyhow::Ok(()), context, "Validating", "library code");

		expression_formatter::println!(r#"{"Done!".bold().green()}"#);
		println!();

		Ok(())
	}
}
