use std::{
	ops::{Deref, DerefMut},
	path::PathBuf,
};

use convert_case::{Case, Casing};
use semver::Version;

use crate::{choose, toml};

toml! {
	cabin: "cabin.toml":

	[description]
	name: String = "unnamed";
	description: String = "An example cabin project.";
	version: Version = Version::parse("0.1.0").unwrap();

	[options]
	quiet: bool = false;
	debug_mode: bool = true;
	developer_mode: bool = true;
	detailed_errors: bool = true;
	tab_size: i64 = 4;
	theme: String = choose!("catppuccin-mocha", "one-midnight").default("catppuccin-mocha");
}

impl CabinOptions {
	pub fn tabs(&self, count: usize) -> String {
		" ".repeat(self.tab_size() as usize * count)
	}
}
