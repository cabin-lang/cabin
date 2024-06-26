use crate::{
	cli::commands::CabinCommand,
	compile_time::builtin::IS_FIRST_PRINT,
	compiler::{compile_c_to, run_native_executable, temp_dir, transpile, write_c},
	context::Context,
	lexer::tokenize,
	log,
	parser::parse,
	step, PRELUDE,
};

use std::sync::atomic::Ordering;

use colored::Colorize as _;

/// The run command, which builds and runs the given file.
#[derive(clap::Parser)]
pub struct RunCommand {
	/// The name of the file to run.
	pub filename: Option<String>,

	/// Run the program in "self.quiet mode". This prevents the Cabin compiler from outputting debug messages such as
	/// "Tokenizing source code" etc. The only output shown from the Cabin compiler will be errors/warnings/info
	/// generated from your program itself, not static compiler progress messages.
	#[arg(long, short)]
	pub quiet: bool,

	/// Emit transpiled C into a file. If the program is tokenized, parsed, compile_time-evaluated, and transpiled successfully, the transpiled C
	/// code will be outputted to this file. The program will still be built to a native executable and run.
	#[arg(long, short = 'c')]
	pub emit_c: Option<String>,
}

impl CabinCommand for RunCommand {
	fn execute(&self) -> anyhow::Result<()> {
		let config_string = std::fs::read_to_string("./cabin.toml")
			.map_err(|error| anyhow::anyhow!("Error getting configuration file: {error}. If you were trying to set this option globally, use the --global flag."))?;
		let mut config: toml_edit::DocumentMut = config_string.parse()?;

		let Some(toml_edit::Item::Table(information_config)) = config.get_mut("information") else {
			anyhow::bail!("Error reading configuration file: Could not find \"information\" table.");
		};
		let Some(toml_edit::Item::Value(toml_edit::Value::String(name))) = information_config.get("name") else {
			anyhow::bail!("Error reading configuration file: Could not find \"name\" value in project configuration under [information].");
		};

		if !self.quiet {
			println!(
				"{} {}\t\t{}",
				"Running".green().bold(),
				format!("{}...", self.filename.as_ref().unwrap_or(&name.to_string()).trim().cyan()).bold(),
				"(Run with --quiet or -q to silence this output)".truecolor(100, 100, 100)
			);
		}

		// Context
		let file_name = self.filename.clone().unwrap_or_else(|| "./src/main.cbn".to_owned());

		// Input file
		log!(self.quiet, "{}", format!("\t{} source code... ", "Reading".green()).bold())?;
		let source_code = PRELUDE.to_owned() + "\n\n" + &step!(std::fs::read_to_string(&file_name), "Input reading error", self.quiet);
		let mut context = Context::new(file_name);

		// Tokenization
		log!(self.quiet, "{}", format!("\t{} source code... ", "Tokenizing".green()).bold())?;
		let tokens = step!(tokenize(source_code), "Tokenization Error", self.quiet, context, true);

		// Parsing
		log!(self.quiet, "{}", format!("\t{} token stream... ", "Parsing".green()).bold())?;
		let ast = step!(parse(&mut tokens.into_iter().collect(), &mut context), "Parsing Error", self.quiet, context, true);

		// compile_time
		log!(self.quiet, "{}", format!("\t{} compile-time code... ", "Running".green()).bold())?;
		let compile_time_ast = step!(ast.compile_time_evaluate(&mut context, true), "Compile-Time Evaluation Error", self.quiet, context, false);
		if IS_FIRST_PRINT.load(Ordering::Relaxed) {
			println!("{}", "Done!".bold().green());
		}

		// Transpilation
		log!(self.quiet, "{}", format!("\t{} to C... ", "Transpiling".green()).bold())?;
		let c_code = step!(transpile(&compile_time_ast, &mut context), "Transpilation Error", self.quiet, context, true);
		let c_file = write_c(&c_code)?;
		if let Some(emit_c_file) = &self.emit_c {
			std::fs::write(emit_c_file, &c_code)?;
		}

		// Compilation
		log!(self.quiet, "{}", format!("\t{} generated C code... ", "Compiling".green()).bold())?;
		let exe_file = step!(
			compile_c_to(&c_file, &(temp_dir() + "/cabin_output"), &mut context),
			"C Compilation Error",
			self.quiet,
			context,
			true
		);

		if !context.warnings.is_empty() {
			println!();
		}

		for warning in &context.warnings {
			println!("{}", warning.lines().map(|line| format!("\t{line}")).collect::<Vec<_>>().join("\n"));
		}

		if !context.warnings.is_empty() {
			println!();
		}

		// Run executable
		if !self.quiet {
			println!("{}", format!("{} Running compiled executable.\n", "Done!".green()).bold());
		}

		run_native_executable(&exe_file)?;

		std::fs::remove_file(&c_file)?;

		Ok(())
	}
}
