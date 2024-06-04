use crate::cli::commands::{build::BuildCommand, configure::ConfigureCommand, format::FormatCommand, new::NewCommand, run::RunCommand, transpile::TranspileCommand};

/// The build module, which handles the `cabin build` command.
pub mod build;

/// The check module, which handles the `cabin check` command.
pub mod check;

/// The configure module, which handles the `cabin configure` command.
pub mod configure;

/// The format module, which handles the `cabin format` command.
pub mod format;

/// The new module, which handles the `cabin new` command.
pub mod new;

/// The run module, which handles the `cabin run` command.
pub mod run;

/// The transpile module, which handles the `cabin transpile` command.
pub mod transpile;

/// A cabin subcommand. This provides the join functionality to execute the command that's `enum_dispatched` to all variants of `SubCommand`.
#[enum_dispatch::enum_dispatch]
pub trait CabinCommand {
	/// Executes this subcommand, using the arguments given at the command line and parsed by clap into this struct.
	fn execute(&self) -> anyhow::Result<()>;
}

/// The subcommands in the language, such as `run` or `build`.
#[derive(clap::Subcommand)]
#[enum_dispatch::enum_dispatch(CabinCommand)]
pub enum SubCommand {
	/// The run command, which builds and runs the given file.
	Run(RunCommand),

	/// Creates a new Cabin project with the given name. If the given name is a
	/// path, then the project will be created at that path. For example, If you run `cabin new projects/example`,
	/// This will create a new project called "example" in the "projects" directory.
	///
	/// The project is automatically initialized with a `main.cbn` file in a `src` folder with hello world code.
	New(NewCommand),

	/// Compiles a Cabin project or file and outputs the build as a native executable. If an argument is passed,
	/// it will be interpreted as a file path of the file to build, and this will be built as a single script file.
	/// If no file is passed, the command is assumed to be running in a Cabin project with a standard file structure,
	/// and it attempts to run the file at `./src/main.cbn`
	///
	/// By default, if you run a file named `file.cbn`, The output will be called `file` on Unix systems and `file.exe`
	/// on Windows. If the command was run on a specific single-file, The compiled binary will be placed in the same directory
	/// as the source file. If there was no file specified and the command was run in a Cabin project, the file will be
	/// placed as `builds/file-<VERSION>` (`.exe` on Windows).
	Build(BuildCommand),

	/// Configure the Cabin compiler. If this is run without the `--global` or `-g` flag, it will modify the configuration file (`./cabin.toml`) to include
	/// the passed options. When running the cabin compiler for that project, those options should be used. If run with the `--global` flag, this will affect
	/// your global compiler settings `~/.config/cabin/cabin.toml`, which are used as a default when always using the compiler
	Configure(ConfigureCommand),

	/// Format Cabin code. If a file name is provided, the given file will be formatted using the Cabin formatter. If no file name is provided, it is
	/// assumed to be running in a Cabin project, and will format all files ending in `.cbn` in `./src`.
	Format(FormatCommand),

	/// Transpile Cabin code into another language. Currently, only C is supported, but the interface used to transpile Cabin to C is extendible enough to
	/// add in other languages without much difficulty, so in the future other languages may be supported. To compile Cabin to a native executable,
	/// use `cabin build` instead.
	Transpile(TranspileCommand),
}

/// Performs a step in compiler execution. The given expression is evaluated and must return a `Result`. If the value is `Ok`, "Done!\n" is logged and the
/// value inside the `Ok` is returned, otherwise (in the case of an error), "Error:\n" is logged, along with the error, and the program is exited with
/// an exit code of 1. The error message will be printed even in quiet mode.
///
/// # Parameters
/// - `expression` The expression to run
#[macro_export]
macro_rules! step {
	// Context provided - print error notes
	(
		$expression: expr, $error_type: literal, $quiet: expr, $context: expr, $print_done: expr
	) => {
		match $expression {
			Ok(ast) => {
				if $print_done {
					log!($quiet, "{}", "Done!\n".green().bold())?;
				}
				ast
			},
			Err(err) => {
				log!($quiet, "{}", "Error:\n\n".red().bold())?;
				eprintln!("{}: {err}", $error_type.red().bold().underline());
				if !$context.error_details.is_empty() {
					println!();
				}
				for note in &$context.error_details {
					eprintln!("{} {note}\n", "Error Details:".bold().bright_purple().underline())
				}

				if $context.encountered_compiler_bug {
					println!(
						"\n{} Please run the program again with the {} flag, and report the issue at {}.\n",
						"This is an internal error with the Cabin compiler.".bold().red(),
						"--show-c-errors".bold().cyan(),
						"https://github.com/cabin-lang/cabin/issues".bold().bright_blue().underline()
					);
				}
				std::process::exit(1);
			},
		}
	};

	// context not provided - no notes
	(
		$expression: expr, $error_type: literal, $quiet: expr
	) => {
		match $expression {
			Ok(ast) => {
				log!($quiet, "{}", "Done!\n".green().bold())?;
				ast
			},
			Err(err) => {
				log!($quiet, "{}", "Error:\n".red().bold())?;
				eprintln!("{}: {err}", $error_type.red().bold());
				std::process::exit(1);
			},
		}
	};
}

/// Prints formatted output to stdout. This will only print if `$quiet` is false. The result is printed to stdout *without* a trailing newline,
/// and `stdout` is flushed.
#[macro_export]
macro_rules! log {
    ($quiet: expr, $($arg:tt)*) => {
		{
			if !$quiet {
				print!($($arg)*);
				<std::io::Stdout as std::io::Write>::flush(&mut std::io::stdout())
			} else {
				Ok::<(), std::io::Error>(())
			}
		}
    };
}
