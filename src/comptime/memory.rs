use std::collections::HashMap;

use crate::parser::expressions::object::LiteralObject;

pub struct VirtualMemory {
	memory: HashMap<usize, LiteralObject>,
}

impl VirtualMemory {
	pub fn empty() -> VirtualMemory {
		VirtualMemory { memory: HashMap::new() }
	}

	pub fn store(&mut self, value: LiteralObject) -> usize {
		let address = self.next_unused_virtual_address();
		self.memory.insert(address, value);
		address
	}

	pub fn get(&self, address: usize) -> Option<&LiteralObject> {
		self.memory.get(&address)
	}

	pub fn get_mut(&mut self, address: usize) -> Option<&mut LiteralObject> {
		self.memory.get_mut(&address)
	}

	fn next_unused_virtual_address(&self) -> usize {
		let mut next_unused_virtual_address = 0;
		while self.memory.contains_key(&next_unused_virtual_address) {
			next_unused_virtual_address += 1;
		}
		next_unused_virtual_address
	}
}
