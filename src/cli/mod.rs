use std::path::PathBuf;

use crate::api::context::CompilerConfiguration;

pub mod commands;

pub enum RunningContext {
	SingleFile(PathBuf),
	Project(Project),
}

impl RunningContext {
	pub fn file_or_project_name(&self) -> PathBuf {
		match self {
			Self::SingleFile(file_name) => file_name.clone(),

			// pathbuf i hate you so much why must u be like this
			Self::Project(project) => project.root_directory().components().last().unwrap().as_os_str().to_str().unwrap().to_owned().into(),
		}
	}

	pub fn entry_point(&self) -> PathBuf {
		match self {
			Self::SingleFile(file) => file.to_owned(),
			Self::Project(project) => project.main_file(),
		}
	}

	pub fn config(&self) -> CompilerConfiguration {
		match self {
			Self::SingleFile(_) => CompilerConfiguration::default(),
			Self::Project(project) => project.config(),
		}
	}
}

pub struct Project {
	root_directory: PathBuf,
	pub config: toml_edit::DocumentMut,
}

impl Project {
	pub fn new(root_directory: &PathBuf) -> anyhow::Result<Project> {
		Ok(Self {
			root_directory: root_directory.to_owned(),
			config: std::fs::read_to_string(PathBuf::from(root_directory).join("cabin.toml"))?.parse()?,
		})
	}

	pub fn root_directory(&self) -> &PathBuf {
		&self.root_directory
	}

	pub fn main_file(&self) -> PathBuf {
		self.root_directory.join("src").join("main.cabin")
	}

	pub fn write_config(&mut self) {
		std::fs::write(self.root_directory.join("cabin.toml"), self.config.to_string()).unwrap()
	}

	pub fn config(&self) -> CompilerConfiguration {
		let options = self.config.get("options").unwrap().as_table().unwrap();
		CompilerConfiguration {
			quiet: options.get("quiet").map(|value| value.as_bool().unwrap()).unwrap_or(false),
			developer_mode: options.get("developer-mode").map(|value| value.as_bool().unwrap()).unwrap_or(false),
			code_tab_size: options.get("terminal-tab-size").map(|value| value.as_integer().unwrap()).unwrap_or(4).try_into().unwrap(),
		}
	}
}
