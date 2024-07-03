use crate::{compile_time::TranspileToC, context::Context, parser::Program};

use std::process::Stdio;

use colored::Colorize as _;

/// A slice of possible C compilers that we can use to compile C code. This is in order of preference, i.e., first we prefer Clang because it is faster,
/// has more features, etc., next gcc, and so on. These are iterated over and checked if any are installed. If the user has none of these, an error will
/// be thrown when attempting to compile Cabin code.
static COMPILERS: &[&str] = &["clang", "gcc", "zig"];

/// Returns the C compiler that the user has installed on their system. This is the name of the command, not a human readable name, so this will return something
/// like `gcc` instead of `GNU C Compiler`. If the user doesn't have a C compiler installed, `None` is returned.
///
/// # Returns
/// The command name of the C compiler installed on the users system, or `None` if the user has no C compiler.
#[must_use]
fn get_c_compiler() -> Option<&'static str> {
	COMPILERS.iter().find(|compiler| which::which(compiler).is_ok()).copied()
}

/// Returns an owned string of the OS-specific temporary directory. For Unix, this might be something like `/tmp`, and for Windows, this might be something like
/// `%localappdata%/Temp`. This is a convenience function to avoid lifetime issues, because often inlining this can give errors about temporary values that
/// need a `let` binding to live longer.
#[must_use]
pub fn temp_dir() -> String {
	std::env::temp_dir().to_str().unwrap().to_owned()
}

/// Transpiles an abstract syntax tree (AST) into C code. This uses the `C` trait defined in `compile_time.rs` to transpile the AST.
///
/// # Parameters
/// - `compile_time_ast` - an AST that has already gone through compile-time evaluation. This can be obtained by taking a regular parsed AST and calling
/// `compile_time_evaluate` on it from the `compile_time` trait.
/// - `context` - The context of the program, which supplies global data such as the scopes and variables declared in the program.
///
/// # Returns
/// Transpiled C code as a string. This will be C code that has a `main` function and can be dumped right into a valid `C` file. If there
/// was an error during transpilation, an `Err` is returned.
pub fn transpile(compile_time_ast: &Program, context: &mut Context) -> anyhow::Result<String> {
	let c_prelude = compile_time_ast
		.c_prelude(context)
		.map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while generating C prelude for the program".dimmed()))?
		.trim()
		.to_owned()
		+ "\n\n";
	let c = c_prelude
		+ &compile_time_ast
			.to_c(context)
			.map_err(|error| anyhow::anyhow!("{error}\n\twhile transpiling the program's global variables into C code"))?;
	let pattern = regex_macro::regex!("\n\n+");
	Ok(pattern.replace_all(&c, "\n\n").to_string())
}

/// Writes outputted C code into the default file, which is in the OS-dependent temporary directory (see `temp_dir()`). This will error
/// if there is some error writing the file, such as insufficient permissions. Otherwise, the path to the file written is returned.
///
/// # Parameters
/// - `c_code` - A string of valid C code. This can be retrieved using `transpile()`.
///
/// # Returns
/// The path to the C file written, or an `Err` if there was an error writing to the file (insufficient permissions, out of storage space, etc.)
pub fn write_c(c_code: &str) -> anyhow::Result<String> {
	std::fs::write(temp_dir() + "/cabin_output.c", c_code).map_err(|error| anyhow::anyhow!("Error writing transpiled C code to file: {error}"))?;
	Ok(temp_dir() + "/cabin_output.c")
}

/// Compiles a C file and outputs the result as a native executable.
///
/// # Parameters
/// - `file_to_compile` - The path to the C file to compile. This file must exist and contain valid C code.
/// - `output_path` - The file to output the C file to. This file will likely be overwritten if it already exists, but technically the behavior is dependent
/// on the C compiler being used.
///
/// # Returns
/// The path to the compiled C code. The path is guaranteed to be a valid path to a file that exists and is a native executable. The file will end with `.exe`
/// on Windows, and have no extension on all other operating systems. If an error occurs, such as the C compiler throwing an error, an `Err` will be returned.
pub fn compile_c_to(file_to_compile: &str, output_path: &str, context: &mut Context) -> anyhow::Result<String> {
	let extension = get_native_executable_extension();
	let mut cmd = std::process::Command::new(get_c_compiler().ok_or_else(|| anyhow::anyhow!("No C compiler found!"))?);
	let output = cmd
		.arg("-ferror-limit=0")
		.arg("-w")
		.arg("-o")
		.arg(format!("{output_path}{extension}"))
		.arg(file_to_compile)
		.stdout(Stdio::piped())
		.stderr(Stdio::piped())
		.output()
		.map_err(|error| anyhow::anyhow!("Error during C compilation: Unable to spawn C compiler: {error}."))?;
	let status = output.status;

	if !status.success() {
		context.encountered_compiler_bug = true;
		if context.show_c_errors {
			let out = String::from_utf8(output.stdout).unwrap();
			let err = String::from_utf8(output.stderr).unwrap();
			anyhow::bail!("Error during C compilation: Compilation failed with {status}.\nSTDOUT:{out}\nSTDERR:\n{err}");
		}
		else {
			anyhow::bail!("Error during C compilation: Compilation failed with {status}.");
		}
	}

	Ok(format!("{output_path}{extension}"))
}

/// Returns the operating system of the user, as specified by `Os`.
#[must_use]
pub fn get_os() -> Os {
	if std::env::consts::OS == "windows" {
		Os::Windows
	} else {
		Os::Unix
	}
}

/// Returns the extension, including the dot, for a native executable on the user's machine, based on their operating
/// system. On Windows, this returns `.exe`, and on Unix, this returns the empty string.
#[must_use]
pub fn get_native_executable_extension() -> &'static str {
	match get_os() {
		Os::Windows => ".exe",
		Os::Unix => "",
	}
}

/// Runs a native executable binary file at the given path. This is used to run a native executable generated by compiling the C code generated from
/// transpiling Cabin.
///
/// # Parameters
/// - `file_path` - The path of the native executable to run. This must be a valid file path to a file that exists and is natively executable. If it's not,
/// an `Err` will be returned.
///
/// # Returns
/// An error if there was an error locating or running the file.
pub fn run_native_executable(file_path: &str) -> anyhow::Result<()> {
	std::process::Command::new(file_path)
		.spawn()
		.map_err(|error| anyhow::anyhow!("Error while attempting to run C binary executable: {error}"))?
		.wait()?;
	Ok(())
}

/// An operating system that the Cabin programming language supports. This is used to handle OS-specific operations,
/// like getting the file extension for a native executable.
#[derive(PartialEq, Eq)]
pub enum Os {
	/// The Windows operating systems. This covers all versions of Windows.
	Windows,
	/// The Unix operating systems. This covers all Unix systems, such as Linux and MacOs.
	Unix,
}
