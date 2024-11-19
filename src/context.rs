use crate::{comptime::memory::VirtualMemory, parser::scope::ScopeData};

pub struct Context {
	pub scope_data: ScopeData,
	pub scope_label: Option<String>,
	pub virtual_memory: VirtualMemory,
	side_effects_stack: Vec<bool>,
}

impl Context {
	pub fn new() -> Context {
		Context {
			scope_data: ScopeData::global(),
			scope_label: None,
			virtual_memory: VirtualMemory::empty(),
			side_effects_stack: Vec::new(),
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
}
