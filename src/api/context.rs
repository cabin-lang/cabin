use std::sync::{LazyLock, RwLock};

use colored::{ColoredString, Colorize};

use crate::{
	api::scope::ScopeData,
	cli::{
		theme::{Styled as _, Theme, CATPPUCCIN_MOCHA},
		RunningContext,
	},
	comptime::memory::VirtualMemory,
	lexer::Span,
	mapped_err,
	parser::expressions::{name::Name, Expression},
};

use super::config_files::{CabinToml, CabinTomlWriteOnDrop};

pub struct Context {
	// Publicly mutable
	pub scope_data: ScopeData,
	pub scope_label: Option<Name>,
	pub virtual_memory: VirtualMemory,
	pub running_context: RunningContext,
	pub lines_printed: usize,
	pub theme: Theme,
	pub colored_program: Vec<ColoredString>,

	// Privately mutable
	side_effects_stack: Vec<bool>,
	error_location: RwLock<Option<Span>>,
	error_details: RwLock<Option<String>>,
	compiler_error_position: RwLock<Vec<SourceFilePosition>>,
	options: CabinToml,
}

impl Default for Context {
	fn default() -> Self {
		Context {
			options: CabinToml::default(),
			scope_data: ScopeData::global(),
			scope_label: None,
			virtual_memory: VirtualMemory::empty(),
			side_effects_stack: Vec::new(),
			error_location: RwLock::new(None),
			error_details: RwLock::new(None),
			compiler_error_position: RwLock::new(Vec::new()),
			lines_printed: 0,
			running_context: RunningContext::try_from(std::env::current_dir().unwrap()).unwrap(),
			theme: CATPPUCCIN_MOCHA,
			colored_program: Vec::new(),
		}
	}
}

impl Context {
	pub fn toggle_side_effects(&mut self, side_effects: bool) {
		self.side_effects_stack.push(side_effects);
	}

	pub fn untoggle_side_effects(&mut self) {
		self.side_effects_stack.pop();
	}

	pub fn has_side_effects(&self) -> bool {
		self.side_effects_stack.last().cloned().unwrap_or(true)
	}

	pub fn nothing(&mut self) -> anyhow::Result<Expression> {
		Ok(Expression::Pointer(
			self.scope_data
				.get_variable("nothing")
				.unwrap()
				.clone()
				.try_as_literal()
				.cloned()
				.map_err(mapped_err! {
					while = format!("interpreting the value of the global variable {} as a literal", "nothing".bold().yellow()),
				})?
				.address
				.unwrap(),
		))
	}

	pub fn set_error_position(&self, position: &Span) {
		if self.error_location.try_read().unwrap().is_none() {
			*self.error_location.try_write().unwrap() = Some(position.clone());
		}
	}

	pub fn set_error_details(&self, error_details: &str) {
		if self.error_details.try_read().unwrap().is_none() {
			*self.error_details.try_write().unwrap() = Some(error_details.to_owned());
		}
	}

	pub fn error_details(&self) -> Option<String> {
		self.error_details.try_read().unwrap().to_owned()
	}

	pub fn error_position(&self) -> Option<Span> {
		self.error_location.try_read().unwrap().to_owned()
	}

	pub fn set_compiler_error_position(&self, position: SourceFilePosition) {
		self.compiler_error_position.try_write().unwrap().push(position);
	}

	pub fn get_compiler_error_position(&self) -> Vec<SourceFilePosition> {
		self.compiler_error_position.try_read().unwrap().clone()
	}

	pub fn colored_program(&self) -> String {
		let mut builder = String::new();
		for (position, character) in self.colored_program.iter().enumerate() {
			if let Some(error_location) = self.error_position() {
				if error_location.contains(position) {
					builder += &format!("{}", character.clone().red().underline().bold());
				} else {
					builder += &format!("{}", character);
				}
			}
		}

		if let Some(error_location) = self.error_position() {
			let length = error_location.length();
			let (error_line, error_column) = self.line_column(error_location);

			format!(
				"{}",
				builder
					.lines()

					// Iterate over the line numbers in addition to the lines
					.enumerate()

					// Filter - we only want to show lines around the error, so this filters to retain only the lines
					// within 3 lines of the error.
					.filter(|(line_number, _line)| (line_number + 1).abs_diff(error_line) < 3)

					// Map - map each line to a string displaying the line number and line, as well as the error details
					// if it's the appropriate position
					.map(|(line_number, line)| {
						format!(
							"    {line_number}  {line}",
							line_number = {
								let value = (line_number + 1).to_string();
								if line_number + 1 == error_line {
									value.bold().red()
								} else {
									value.style(self.theme.line_numbers())
								}
							},
							line = if line_number + 1 == error_line {
								format!(
									"{line}\n{spacing}{arrows} {message}",
									spacing = " ".repeat(format!("    {line_number}  ").len() + error_column - 1),
									arrows = "^".repeat(length).dimmed(),
									message = "The error is here".dimmed()
								)
							} else {
								line.to_owned()
							}
						)
					})

					// Collect the lines back into a vector of string lines
					.collect::<Vec<_>>()

					// Join them together with a newline separator
					.join("\n")

					// Style the result onto the background color
					.style(self.theme.background())
			)
		}
		// No error: Return the plain program
		else {
			builder
		}
	}

	pub fn program(&self) -> String {
		self.colored_program.iter().map(|part| (**part).to_owned()).collect::<String>()
	}

	pub fn line_column(&self, span: Span) -> (usize, usize) {
		let blank = self.program();

		let mut line = 1;
		let mut column = 1;

		for (position, char) in blank.chars().enumerate() {
			if position == span.start() {
				break;
			}

			if char == '\n' {
				line += 1;
				column = 1;
			} else {
				column += 1;
			}
		}

		(line, column)
	}

	pub fn config(&self) -> &CabinToml {
		&self.options
	}

	/// Returns a mutable reference to the data stored in the project's `cabin.toml`. If the user is running a single
	/// Cabin file and not in a project, an error is returned. When this value is dropped, the `cabin.toml` file is
	/// written to update to the contents of the returned object.
	pub fn cabin_toml_mut(&mut self) -> anyhow::Result<CabinTomlWriteOnDrop> {
		let RunningContext::Project(project) = &self.running_context else {
			anyhow::bail!("Attempted to get a mutable reference to the cabin.toml, but Cabin is not currently running in a project.");
		};
		Ok(CabinTomlWriteOnDrop::new(&mut self.options, project.root_directory().to_owned()))
	}
}

#[derive(Debug, Clone)]
pub struct SourceFilePosition {
	line: u32,
	column: u32,
	name: &'static str,
	function: String,
}

impl SourceFilePosition {
	pub fn new(line: u32, column: u32, name: &'static str, function: String) -> Self {
		Self { line, column, name, function }
	}

	pub fn line(&self) -> u32 {
		self.line
	}

	pub fn column(&self) -> u32 {
		self.column
	}

	pub fn file_name(&self) -> &'static str {
		self.name
	}

	pub fn function_name(&self) -> String {
		self.function.clone()
	}
}

#[macro_export]
macro_rules! here {
	() => {
		$crate::api::context::SourceFilePosition::new(std::line!(), std::column!(), std::file!(), $crate::function!())
	};
}

static CONTEXT: LazyLock<Box<Context>> = LazyLock::new(|| Box::new(Context::default()));

pub fn context() -> &'static mut Context {
	unsafe { (&**crate::api::context::CONTEXT as *const Context as *mut Context).as_mut().unwrap() }
}
