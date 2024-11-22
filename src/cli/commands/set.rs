use crate::cli::commands::CabinCommand;

#[derive(clap::Parser)]
pub struct CompilerConfiguration {
	#[arg(long, short, default_value_t = 4)]
	pub tab_size: usize,

	#[arg(long, short)]
	pub quiet: bool,

	#[arg(long, short)]
	pub developer_mode: bool,
}

impl CabinCommand for CompilerConfiguration {
	fn execute(self) -> anyhow::Result<()> {
		Ok(())
	}
}
