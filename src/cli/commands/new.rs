use crate::cli::commands::CabinCommand;

/// Creates a new Cabin project with the given name. If the given name is a
/// path, then the project will be created at that path. For example, If you run `cabin new projects/example`,
/// This will create a new project called "example" in the "projects" directory.
///
/// The project is automatically initialized with a `main.cbn` file in a `src` folder with hello world code.
#[derive(clap::Parser)]
pub struct NewCommand {
	/// The path to the project directory that is being created.
	project_name: String,
}

impl CabinCommand for NewCommand {
	fn execute(&self) -> anyhow::Result<()> {
		let project_name = &self.project_name;

		// Create /src
		std::fs::create_dir_all(format!("{project_name}/src"))?;

		// Main Cabin file
		std::fs::write(format!("{project_name}/src/main.cbn"), "run terminal.print(\"Hello world!\");")?;

		// Project configuration
		std::fs::write(
			format!("{project_name}/cabin.toml"),
			unindent::unindent(&format!(
				r#"
				[information]
				name = "{project_name}"
				version = "0.0.1"
				description = "An example template Cabin project."
				license = "All rights reserved"
				
				[options]
				quiet = false
				"#
			)),
		)?;

		// .gitignore
		std::fs::write(format!("{project_name}/.gitignore"), "builds/")?;

		Ok(())
	}
}
