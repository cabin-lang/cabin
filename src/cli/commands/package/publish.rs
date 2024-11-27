use std::process::Command;

use crate::{api::context::context, cli::commands::CabinCommand, step};

use semver::Version;

#[derive(clap::Parser)]
pub struct PublishCommand {
	#[arg(group = "version")]
	major: bool,

	#[arg(group = "version")]
	minor: bool,

	#[arg(group = "version")]
	patch: bool,
}

impl CabinCommand for PublishCommand {
	fn execute(self) -> anyhow::Result<()> {
		let _commit = step!(
			String::from_utf8(Command::new("git").arg("log").arg("-n").arg("1").arg("--pretty=format:\"%H\"").output()?.stdout,),
			"Getting",
			"version information"
		);

		let mut version = context().config().description().version();
		version = if self.major {
			Version::new(version.major + 1, 0, 0)
		} else if self.minor {
			Version::new(version.major, version.minor + 1, 0)
		} else if self.patch {
			Version::new(version.major, version.minor, version.patch + 1)
		} else {
			unreachable!()
		};

		context().cabin_toml_mut()?.options_mut().try_set("version", &version.to_string()).unwrap();

		Ok(())
	}
}
