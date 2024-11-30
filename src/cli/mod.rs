use std::path::PathBuf;

pub mod commands;
pub mod theme;

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
			Self::Project(project) => pathdiff::diff_paths(project.main_file(), std::env::current_dir().unwrap()).unwrap(),
		}
	}
}

impl TryFrom<&PathBuf> for RunningContext {
	type Error = anyhow::Error;

	fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
		Ok(if PathBuf::from(path).is_dir() {
			RunningContext::Project(Project::new(path)?)
		} else if PathBuf::from(path).is_file() {
			RunningContext::SingleFile(path.to_owned())
		} else {
			anyhow::bail!("Invalid path");
		})
	}
}

pub struct Project {
	root_directory: PathBuf,
}

impl Project {
	pub fn new(root_directory: &PathBuf) -> anyhow::Result<Project> {
		Ok(Self {
			root_directory: root_directory.to_owned(),
		})
	}

	pub const fn root_directory(&self) -> &PathBuf {
		&self.root_directory
	}

	pub fn main_file(&self) -> PathBuf {
		self.root_directory.join("src").join("main.cabin")
	}
}
