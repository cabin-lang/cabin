use std::collections::HashMap;

use crate::{api::context::Context, parser::expressions::literal::LiteralObject};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pointer(usize);

impl Pointer {
	pub fn virtual_deref<'a>(&self, context: &'a Context) -> &'a LiteralObject {
		context.virtual_memory.get(self.to_owned()).unwrap()
	}
}

pub struct VirtualMemory {
	memory: HashMap<usize, LiteralObject>,
}

impl VirtualMemory {
	pub fn empty() -> VirtualMemory {
		VirtualMemory { memory: HashMap::new() }
	}

	pub fn store(&mut self, value: LiteralObject) -> Pointer {
		let address = self.next_unused_virtual_address();
		self.memory.insert(address, value);
		Pointer(address)
	}

	pub fn get(&self, address: Pointer) -> Option<&LiteralObject> {
		self.memory.get(&address.0)
	}

	pub fn get_mut(&mut self, address: Pointer) -> Option<&mut LiteralObject> {
		self.memory.get_mut(&address.0)
	}

	fn next_unused_virtual_address(&self) -> usize {
		let mut next_unused_virtual_address = 0;
		while self.memory.contains_key(&next_unused_virtual_address) {
			next_unused_virtual_address += 1;
		}
		next_unused_virtual_address
	}
}
