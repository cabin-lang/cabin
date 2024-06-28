use crate::{cli::commands::CabinCommand, context::Context, formatter::ToCabin, lexer::tokenize, log, parser::parse, step};

use colored::Colorize as _;

/// Format Cabin code. If a file name is provided, the given file will be formatted using the Cabin formatter. If no file name is provided, it is
/// assumed to be running in a Cabin project, and will format all files ending in `.cbn` in `./src`.
#[derive(clap::Parser)]
pub struct FormatCommand {
	/// The name of the file to format, or left out to format an entire project.
	file_name: Option<String>,

	/// Run the program in "quiet mode". This prevents the Cabin compiler from outputting debug messages such as
	/// "Tokenizing source code" etc. The only output shown from the Cabin compiler will be errors/warnings/info
	/// generated from your program itself, not static compiler progress messages.
	#[arg(long, short)]
	quiet: bool,
}

impl CabinCommand for FormatCommand {
	fn execute(&self) -> anyhow::Result<()> {
		let config_string = std::fs::read_to_string("./cabin.toml")
			.map_err(|error| anyhow::anyhow!("Error getting configuration file: {error}. If you were trying to set this option globally, use the --global flag."))?;
		let mut config: toml_edit::DocumentMut = config_string.parse()?;

		let Some(toml_edit::Item::Table(information_config)) = config.get_mut("information").cloned() else {
			anyhow::bail!("Error reading configuration file: Could not find \"information\" table.");
		};
		let Some(toml_edit::Item::Table(options)) = config.get_mut("options") else {
			anyhow::bail!("Error reading configuration file: Could not find \"options\" table.");
		};

		// The name of the user's Cabin project as specified in ./cabin.toml
		let project_name = information_config
			.get("name")
			.ok_or_else(|| anyhow::anyhow!("Error reading configuration: Required string field \"name\" in [information] is missing"))?
			.as_str()
			.ok_or_else(|| anyhow::anyhow!("Error reading configuration: the field \"name\" is present in [information], but it is not a string"))?;

		let quiet = options.get("quiet").and_then(|config_quiet| config_quiet.as_bool()).unwrap_or(self.quiet);

		log!(
			quiet,
			"{}\n",
			format!("{} {}...", "Formatting".green(), self.file_name.as_ref().unwrap_or(&project_name.to_owned()))
				.trim()
				.bold()
		)?;

		// Get the files to format. If file_name is provided, this is a single vec of just that file. If not,
		// this is a vec of all Cabin files in the project.
		let files = self.file_name.clone().map_or_else(
			|| {
				walkdir::WalkDir::new("./src")
					.into_iter()
					.filter_map(|file| {
						if let Ok(ok_file) = file {
							if ok_file.path().extension().is_some_and(|extension| extension == "cbn") {
								return Some(ok_file.path().display().to_string());
							}
						}
						None
					})
					.collect()
			},
			|filename| vec![filename],
		);

		for file in files {
			// Context
			log!(quiet, "{}", format!("\t{} {}...\n", "Formatting".green(), file.replace('\\', "/")).bold())?;

			// Input file
			log!(quiet, "{}", format!("\t\t{} source code... ", "Reading".green()).bold())?;
			let source_code = step!(std::fs::read_to_string(file.clone()), "Input reading error", quiet);
			let mut context = Context::new(file.clone());

			// Tokenization
			log!(quiet, "{}", format!("\t\t{} source code... ", "Tokenizing".green()).bold())?;
			let tokens = step!(tokenize(source_code), "Tokenization Error", quiet, context, true);

			// Parsing
			log!(quiet, "{}", format!("\t\t{} token stream... ", "Parsing".green()).bold())?;
			let ast = step!(parse(&mut tokens.into_iter().collect(), &mut context), "Parsing Error", quiet, context, true);

			// Formatting
			log!(quiet, "{}", format!("\t\t{} Parsed AST into formatted Cabin code... ", "Converting".green()).bold())?;
			let formatted = ast.to_cabin();
			log!(quiet, "{}\n", "Done!".green().bold())?;

			// Writing
			log!(quiet, "{}", format!("\t\t{} formatted code back onto file... ", "Writing".green()).bold())?;
			step!(std::fs::write(file, formatted), "File Write Error", quiet, context, true);
			log!(quiet, "\t{}\n", "Done!".green().bold())?;
		}
		log!(quiet, "{}\n", "Done!".green().bold())?;

		Ok(())
	}
}
