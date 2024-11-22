use std::{cell::RefCell, path::PathBuf};

use smart_default::SmartDefault;

use crate::{
	api::scope::ScopeData,
	cli::{
		theme::{Theme, CATPPUCCIN_MOCHA},
		Project, RunningContext,
	},
	comptime::memory::VirtualMemory,
	lexer::Span,
	parser::expressions::{name::Name, Expression},
};

pub struct Context {
	// Publicly mutable
	pub scope_data: ScopeData,
	pub scope_label: Option<Name>,
	pub virtual_memory: VirtualMemory,
	pub config: CompilerConfiguration,
	pub running_context: RunningContext,
	pub lines_printed: usize,
	pub theme: Theme,
	pub colored_program: Option<String>,

	// Privately mutable
	side_effects_stack: Vec<bool>,
	error_location: RefCell<Option<Span>>,
	error_details: RefCell<Option<String>>,
	compiler_error_position: RefCell<Vec<SourceFilePosition>>,
	// Completely immutable
}

impl Context {
	pub fn new(path: &PathBuf) -> anyhow::Result<Context> {
		let running_context = if PathBuf::from(path).is_dir() {
			RunningContext::Project(Project::new(path)?)
		} else if PathBuf::from(path).is_file() {
			RunningContext::SingleFile(path.to_owned())
		} else {
			anyhow::bail!("Invalid path");
		};

		Ok(Context {
			scope_data: ScopeData::global(),
			scope_label: None,
			virtual_memory: VirtualMemory::empty(),
			side_effects_stack: Vec::new(),
			error_location: RefCell::new(None),
			error_details: RefCell::new(None),
			compiler_error_position: RefCell::new(Vec::new()),
			config: running_context.config(),
			lines_printed: 0,
			running_context,
			theme: CATPPUCCIN_MOCHA,
			colored_program: None,
		})
	}

	pub fn toggle_side_effects(&mut self, side_effects: bool) {
		self.side_effects_stack.push(side_effects);
	}

	pub fn untoggle_side_effects(&mut self) {
		self.side_effects_stack.pop();
	}

	pub fn has_side_effects(&self) -> bool {
		self.side_effects_stack.last().cloned().unwrap_or(true)
	}

	pub fn nothing(&self) -> Expression {
		self.scope_data.expect_global_variable("nothing").expect_clone_pointer(self)
	}

	pub fn set_error_position(&self, position: &Span) {
		if self.error_location.borrow().is_none() {
			*self.error_location.borrow_mut() = Some(position.clone());
		}
	}

	pub fn set_error_details(&self, error_details: &str) {
		if self.error_details.borrow().is_none() {
			*self.error_details.borrow_mut() = Some(error_details.to_owned());
		}
	}

	pub fn error_details(&self) -> Option<String> {
		self.error_details.borrow().clone()
	}

	pub fn error_position(&self) -> Option<Span> {
		self.error_location.borrow().clone()
	}

	pub fn set_compiler_error_position(&self, position: SourceFilePosition) {
		self.compiler_error_position.borrow_mut().push(position);
	}

	pub fn get_compiler_error_position(&self) -> Vec<SourceFilePosition> {
		self.compiler_error_position.borrow().clone()
	}
}

#[derive(SmartDefault)]
pub struct CompilerConfiguration {
	#[default = 4]
	pub code_tab_size: usize,
	#[default = false]
	pub quiet: bool,
	#[default = true]
	pub developer_mode: bool,
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

impl CompilerConfiguration {
	pub fn tab(&self) -> String {
		" ".repeat(self.code_tab_size)
	}

	pub fn tabs(&self, count: usize) -> String {
		self.tab().repeat(count)
	}
}
