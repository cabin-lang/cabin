use std::collections::HashMap;

use crate::parser::expressions::literals::Literal;

pub struct VirtualMemory {
	memory: HashMap<usize, Literal>,
}

impl VirtualMemory {
	#[must_use]
	pub fn new() -> Self {
		Self { memory: HashMap::new() }
	}

	#[must_use]
	pub fn get(&self, address: usize) -> Option<&Literal> {
		self.memory.get(&address)
	}

	pub fn insert(&mut self, value: Literal) -> anyhow::Result<()> {
		if self.get(value.virtual_address()).is_some() {
			anyhow::bail!(
				"Attempted to insert a value into a virtual memory address {}, but that address already contains memory",
				value.virtual_address()
			);
		}
		self.memory.insert(value.virtual_address(), value);
		Ok(())
	}

	pub fn overwrite(&mut self, value: Literal) -> anyhow::Result<()> {
		if self.get(value.virtual_address()).is_none() {
			anyhow::bail!(
				"Attempted to overwrite a value into a virtual memory address {}, but that address already contains memory",
				value.virtual_address()
			);
		}
		self.memory.insert(value.virtual_address(), value);
		Ok(())
	}
}
