use crate::{
	cli::commands::CabinCommand,
	compiler::{compile_c_to, transpile, write_c},
	context::Context,
	lexer::tokenize,
	log,
	parser::parse,
	step, PRELUDE,
};

use std::path::Path;

use colored::Colorize as _;

/// Compiles a Cabin project or file and outputs the build as a native executable. If an argument is passed,
/// it will be interpreted as a file path of the file to build, and this will be built as a single script file.
/// If no file is passed, the command is assumed to be running in a Cabin project with a standard file structure,
/// and it attempts to run the file at `./src/main.cbn`
///
/// By default, if you run a file named `file.cbn`, The output will be called `file` on Unix systems and `file.exe`
/// on Windows. If the command was run on a specific single-file, The compiled binary will be placed in the same directory
/// as the source file. If there was no file specified and the command was run in a Cabin project, the file will be
/// placed as `builds/file-<VERSION>` (`.exe` on Windows).
#[derive(clap::Parser)]
pub struct BuildCommand {
	/// The name of the Cabin file to compile into a native binary. This is optional. If given, it will be used as a
	/// single-file standalone script. If not provided, the file at `./src/main.cbn` will be used. If this file does
	/// not exist and none was provided, and error is returned and compilation is aborted.
	filename: Option<String>,

	/// The file path to output the executable binary at. This is optional, and the behavior if none was given is as follows:
	///
	/// By default, if you run a file named `file.cbn`, The output will be called `file` on Unix systems and `file.exe`
	/// on Windows. If the command was run on a specific single-file, The compiled binary will be placed in the same directory
	/// as the source file. If there was no file specified and the command was run in a Cabin project, the file will be
	/// placed as `builds/file-<VERSION>` (`.exe` on Windows).
	output: Option<String>,

	/// Run the program in "self.quiet mode". This prevents the Cabin compiler from outputting debug messages such as
	/// "Tokenizing source code" etc. The only output shown from the Cabin compiler will be errors/warnings/info
	/// generated from your program itself, not static compiler progress messages.
	#[arg(short, long)]
	quiet: bool,

	/// Emit transpiled C into a file. If the program is tokenized, parsed, compile_time-evaluated, and transpiled successfully, the transpiled C
	/// code will be outputted to this file. The program will still be built to a native executable; This does not cancel compilation.
	#[arg(long, short = 'c')]
	emit_c: Option<String>,
}

impl CabinCommand for BuildCommand {
	fn execute(&self) -> anyhow::Result<()> {
		let file_name = std::fs::canonicalize(self.filename.clone().unwrap_or_else(|| "./src/main.cbn".to_owned()))
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
				format!("{} {}...", "Building".green(), self.filename.as_ref().unwrap_or(&project_name.to_owned()))
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

		// Compile-time evaluation
		log!(self.quiet, "{}", format!("\t{} compile-time code... ", "Running".green()).bold())?;
		let compile_time_ast = step!(ast.compile_time_evaluate(&mut context, true), "Compile-Time Evaluation Error", self.quiet, context, false);

		// Transpilation
		log!(self.quiet, "{}", format!("\t{} to C... ", "Transpiling".green()).bold())?;
		let c_code = step!(transpile(&compile_time_ast, &mut context), "Transpilation Error", self.quiet, context, true);
		let c_file = write_c(&c_code)?;
		if let Some(emit_c_file) = &self.emit_c {
			std::fs::write(emit_c_file, &c_code)?;
		}

		// Compilation
		log!(self.quiet, "{}", format!("\t{} generated C code... ", "Compiling".green()).bold())?;
		let output_file = self.output.clone().unwrap_or(if self.filename.is_some() {
			format!("{file_directory}/{file_basename}", file_directory = file_directory.display())
		} else {
			std::fs::create_dir_all("./builds/native")?;
			format!("./builds/native/{project_name}-v{project_version}")
		});
		let output_file_with_extension = compile_c_to(&c_file, &output_file, &mut context)?;
		std::fs::remove_file(c_file)?;
		println!("{}", "Done!".bold().green());

		println!("{} Build ready at {}", "Done!".green().bold(), output_file_with_extension.cyan().bold());

		Ok(())
	}
}
