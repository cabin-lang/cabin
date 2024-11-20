use crate::{
	comptime::memory::VirtualMemory,
	lexer::Position,
	parser::{
		expressions::{name::Name, Expression},
		scope::ScopeData,
	},
};

pub struct Context {
	pub scope_data: ScopeData,
	pub scope_label: Option<String>,
	pub virtual_memory: VirtualMemory,
	side_effects_stack: Vec<bool>,
	error_location: Option<Position>,
	filename: String,
	error_details: Option<String>,
}

impl Context {
	pub fn new(filename: &str) -> Context {
		Context {
			scope_data: ScopeData::global(),
			scope_label: None,
			virtual_memory: VirtualMemory::empty(),
			side_effects_stack: Vec::new(),
			error_location: None,
			filename: filename.to_owned(),
			error_details: None,
		}
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
		self.scope_data.get_global_variable(&Name::from("nothing")).unwrap().try_clone_pointer().unwrap()
	}

	pub fn set_error_position(&mut self, position: &Position) {
		if self.error_location.is_none() {
			self.error_location = Some(position.clone());
		}
	}

	pub fn file_name(&self) -> String {
		self.filename.clone()
	}

	pub fn set_error_details(&mut self, error_details: &str) {
		if self.error_details.is_none() {
			self.error_details = Some(error_details.to_owned());
		}
	}
}
