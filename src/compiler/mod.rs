use std::{env, path::PathBuf};

pub fn compile(c_code: &str) -> anyhow::Result<PathBuf> {
	let c_path = env::temp_dir().join("cabin_transpiled.c");
	std::fs::write(c_path, c_code)?;
	std::process::Command::new("clang")
		.arg("cabin_transpiled.c")
		.arg("-o")
		.arg("cabin_output")
		.current_dir(env::temp_dir())
		.spawn()?;
	Ok(env::temp_dir().join("cabin_output"))
}

pub fn run_native_executable(path: PathBuf) -> anyhow::Result<()> {
	std::process::Command::new(path).spawn()?;
	Ok(())
}
