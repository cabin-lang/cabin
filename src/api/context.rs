use std::sync::LazyLock;

use colored::{ColoredString, Colorize as _};

use crate::{
	api::{
		config_files::{CabinToml, CabinTomlWriteOnDrop},
		scope::ScopeData,
	},
	cli::{
		theme::{Styled as _, Theme},
		RunningContext,
	},
	comptime::memory::VirtualMemory,
	lexer::Span,
	mapped_err,
	parser::expressions::{name::Name, Expression},
};

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
	error_location: Option<Span>,
	error_details: Option<String>,
	compiler_error_position: Vec<SourceFilePosition>,
	options: CabinToml,
	debug_indent: Vec<String>,
}

impl Default for Context {
	fn default() -> Self {
		Context {
			options: CabinToml::default(),
			scope_data: ScopeData::global(),
			scope_label: None,
			virtual_memory: VirtualMemory::empty(),
			side_effects_stack: Vec::new(),
			error_location: None,
			error_details: None,
			compiler_error_position: Vec::new(),
			lines_printed: 0,
			running_context: RunningContext::try_from(std::env::current_dir().unwrap()).unwrap(),
			theme: Theme::default(),
			colored_program: Vec::new(),
			debug_indent: Vec::new(),
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

	pub fn set_error_position(&mut self, position: &Span) {
		if self.error_location.is_none() {
			self.error_location = Some(position.clone());
		}
	}

	pub fn set_error_details(&mut self, error_details: &str) {
		if self.error_details.is_none() {
			self.error_details = Some(error_details.to_owned());
		}
	}

	pub fn error_details(&self) -> Option<String> {
		self.error_details.clone()
	}

	pub fn error_position(&self) -> Option<Span> {
		self.error_location.clone()
	}

	pub fn set_compiler_error_position(&mut self, position: SourceFilePosition) {
		self.compiler_error_position.push(position);
	}

	pub fn get_compiler_error_position(&self) -> Vec<SourceFilePosition> {
		self.compiler_error_position.clone()
	}

	#[must_use]
	pub fn start_debug_sequence(&mut self, message: &str) -> DebugSection {
		self.debug_indent.push(message.to_owned());
		DebugSection
	}

	#[must_use]
	fn end_debug_sequence(&mut self) -> String {
		self.debug_indent.pop().unwrap()
	}

	pub fn debug_indent(&self) -> usize {
		self.debug_indent.len()
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

	/// The fully qualified path name of the Rust function this location takes place in. This is
	/// generally obtained via the `function!()` macro.
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

// forgive me father for i have sinned - The Great Context Refactor (tm) - violet, 11/27/24 @ 5:33AM (no i havent slept yet)

/// Global, mutable, stateful data about the compiler. This can be accessed from anywhere via the `context()` function,
/// which returns a non-borrow-checked mutable reference to the value inside this `LazyLocked`. The context is
/// used for *numerous* things all throughout the program, such as holding scope data, storing error traces, virtual
/// memory, and more.
///
/// Yes, yes, I know, global mutable state bad. But I've experienced the alternative first-hand, and it was worse.
/// Originally, `context` wasn't global, and it was passed around as a parameter to like, every function. No like,
/// seriously, like, *all* of them. it sucked, like a lot, and the fact that EVERYTHING relied on having context
/// made a lot of things impossible &mdash; like implementing `Drop` or `Debug` for things that need to reference
/// the context. Not to mention an excessive amount of cloning to make the borrow checker happy &mdash; overall,
/// it was a poor syntactic layer over what was essentially just global mutable state anyway. Sue me.
static CONTEXT: LazyLock<Context> = LazyLock::new(Context::default);

/// Returns a non-borrow-checked static mutable reference to the program's `Context`, which holds global state
/// data about the compiler.
pub fn context() -> &'static mut Context {
	unsafe { (&*CONTEXT as *const Context as *mut Context).as_mut().unwrap() }
}

pub struct DebugSection;

impl DebugSection {
	pub fn finish(self) {
		let message = context().end_debug_sequence();
		if context().config().options().debug_info() == "some" {
			println!("{}{} {}", "│\t".repeat(context().debug_indent()).dimmed(), "Finished".green().bold(), message);
		}
	}
}
