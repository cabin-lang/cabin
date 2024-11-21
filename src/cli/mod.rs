use std::path::PathBuf;

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
}
