use crate::{cli::commands::CabinCommand, compiler::transpile, context::Context, lexer::tokenize, log, parser::parse, step, PRELUDE};

use std::path::Path;

use colored::Colorize as _;

/// Transpile Cabin code into another language. Currently, only C is supported, but the interface used to transpile Cabin to C is extendible enough to
/// add in other languages without much difficulty, so in the future other languages may be supported. To compile Cabin to a native executable,
/// use `cabin build` instead.
#[derive(clap::Parser)]
pub struct TranspileCommand {
	/// The name of the file to transpile, or left out to transpile an entire project.
	file_name: Option<String>,

	/// Run the program in "self.quiet mode". This prevents the Cabin compiler from outputting debug messages such as
	/// "Tokenizing source code" etc. The only output shown from the Cabin compiler will be errors/warnings/info
	/// generated from your program itself, not static compiler progress messages.
	#[arg(long, short)]
	quiet: bool,

	/// The language to transpile to. Currently only C is supported. This is C by default.
	///
	/// Possible values:
	/// - C
	#[arg(long, default_value = "C")]
	to: String,
}

impl CabinCommand for TranspileCommand {
	#[allow(clippy::iter_on_single_items)]
	fn execute(&self) -> anyhow::Result<()> {
		if self.to.to_lowercase() != "c" {
			eprintln!(
				"{} Cabin currently does not support transpiling to {}. Currently supported languages are:{}",
				"Error: Unsupported transpilation language:".bold().red(),
				self.to.bold().cyan(),
				["C"].into_iter().map(|language| format!("\n\t* {language}")).collect::<String>().bold().cyan()
			);
			std::process::exit(1);
		}

		let file_name = std::fs::canonicalize(self.file_name.clone().unwrap_or_else(|| "./src/main.cbn".to_owned()))
			.map_err(|error| anyhow::anyhow!("Error canonicalizing source file path: {error}"))?;
		let file_name_path = file_name.to_str().unwrap();
		let file_path = Path::new(&file_name_path);
		let file_directory = file_path.parent().ok_or_else(|| anyhow::anyhow!("Error retrieving containing directory of source file"))?;
		let file_basename = {
			let no_extension = file_path.with_extension("");
			no_extension.file_name().unwrap().to_str().unwrap().to_owned()
		};

		let file_name_string = file_name.display().to_string();

		let config_string = std::fs::read_to_string("./cabin.toml")
			.map_err(|error| anyhow::anyhow!("Error getting configuration file: {error}. If you were trying to set this option globally, use the --global flag."))?;
		let mut config: toml_edit::DocumentMut = config_string.parse()?;

		let Some(toml_edit::Item::Table(information_config)) = config.get_mut("information") else {
			anyhow::bail!("Error reading configuration file: Could not find \"information\" table.");
		};

		// The name of the user's Cabin project as specified in ./cabin.toml
		let project_name = information_config
			.get("name")
			.ok_or_else(|| anyhow::anyhow!("Error reading configuration: Required string field \"name\" in [information] is missing"))?
			.as_str()
			.ok_or_else(|| anyhow::anyhow!("Error reading configuration: field \"name\" is present in [information], but it is not a string"))?;

		// The version (as a semver) of the user's Cabin project as specified in ./cabin.toml
		let project_version = information_config
			.get("version")
			.ok_or_else(|| anyhow::anyhow!("Error reading configuration: Required string field \"version\" in [information] is missing"))?
			.as_str()
			.ok_or_else(|| anyhow::anyhow!("Error reading configuration: field \"version\" is present in [information], but it is not a string"))?;

		// Startup message
		if !self.quiet {
			println!(
				"{}",
				format!("{} {}...", "Building".green(), self.file_name.as_ref().unwrap_or(&project_name.to_owned()))
					.trim()
					.bold()
			);
		}

		// Input file
		log!(self.quiet, "{}", format!("\t{} source code... ", "Reading".green()).bold())?;
		let source_code = PRELUDE.to_owned() + "\n\n" + &step!(std::fs::read_to_string(&file_name_string), "Input reading error", self.quiet);
		let mut context = Context::new(file_name_string);

		// Tokenization
		log!(self.quiet, "{}", format!("\t{} source code... ", "Tokenizing".green()).bold())?;
		let tokens = step!(tokenize(source_code), "Tokenization Error", self.quiet, context, true);

		// Parsing
		log!(self.quiet, "{}", format!("\t{} token stream... ", "Parsing".green()).bold())?;
		let ast = step!(parse(&mut tokens.into_iter().collect(), &mut context), "Parsing Error", self.quiet, context, true);

		// compile_time
		log!(self.quiet, "{}", format!("\t{} compile-time code... ", "Running".green()).bold())?;
		let compile_time_ast = step!(ast.compile_time_evaluate(&mut context, true), "Compile-Time Evaluation Error", self.quiet, context, false);

		// Transpilation
		log!(self.quiet, "{}", format!("\t{} to C... ", "Transpiling".green()).bold())?;
		let c_code = step!(transpile(&compile_time_ast, &mut context), "Transpilation Error", self.quiet, context, true);
		let output_file = self.file_name.clone().unwrap_or(if self.file_name.is_some() {
			format!("{file_directory}/{file_basename}", file_directory = file_directory.display())
		} else {
			std::fs::create_dir_all("./builds/c")?;
			format!("./builds/c/{project_name}-v{project_version}.c")
		});
		std::fs::write(&output_file, c_code)?;

		println!("{} C file ready at {}", "Done!".green().bold(), output_file.cyan().bold());
		Ok(())
	}
}
