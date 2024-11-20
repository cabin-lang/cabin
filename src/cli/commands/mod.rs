use run::RunCommand;

pub mod run;

#[enum_dispatch::enum_dispatch]
pub trait CabinCommand {
	fn execute(&self) -> anyhow::Result<()>;
}

#[derive(clap::Subcommand)]
#[enum_dispatch::enum_dispatch(CabinCommand)]
pub enum SubCommand {
	Run(RunCommand),
}

#[macro_export]
macro_rules! step {
	(
        $expression: expr, $context: expr, $action: expr, $object: expr
    ) => {{
		use std::io::Write as _;

		use colored::Colorize as _;

		print!("{} {}... ", $action.bold().green(), $object);
		std::io::stdout().flush().unwrap();
		match $expression {
			Ok(return_value) => {
				println!("{}", "Done!".bold().green());
				return_value
			},
			Err(error) => {
				eprintln!("{}", "Error\n".bold().red());
				eprintln!("{} {error}", "Error:".bold().red());
				println!();
				println!("This error occurred in {}.", $context.file_name().bold().cyan());
				println!();
				std::process::exit(1);
			},
		}
	}};
}
