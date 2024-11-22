use std::collections::HashMap;

use crate::{
	api::context::Context,
	parser::expressions::{literal::LiteralObject, Type},
	transpiler::TranspileToC,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pointer(usize);

impl Pointer {
	pub fn virtual_deref<'a>(&self, context: &'a Context) -> &'a LiteralObject {
		context.virtual_memory.get(self).unwrap()
	}

	pub fn unchecked(address: usize) -> Pointer {
		Pointer(address)
	}

	pub fn value(&self) -> usize {
		self.0
	}
}

impl TranspileToC for Pointer {
	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		Ok(format!("{}_{}", self.virtual_deref(context).clone().name.to_c(context)?, self.value()))
	}
}

impl Type for Pointer {
	fn get_type(&self, context: &mut Context) -> anyhow::Result<Pointer> {
		self.virtual_deref(context).clone().get_type(context)
	}
}

pub struct VirtualMemory {
	memory: HashMap<usize, LiteralObject>,
}

impl VirtualMemory {
	pub fn empty() -> VirtualMemory {
		VirtualMemory { memory: HashMap::new() }
	}

	pub fn store(&mut self, mut value: LiteralObject) -> Pointer {
		let address = self.next_unused_virtual_address();
		value.address = Some(address);
		self.memory.insert(address, value);
		Pointer(address)
	}

	pub fn get(&self, address: &Pointer) -> Option<&LiteralObject> {
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

	pub fn entries(&self) -> Vec<(usize, LiteralObject)> {
		self.memory
			.iter()
			.collect::<Vec<_>>()
			.into_iter()
			.map(|(address, object)| (*address, object.clone()))
			.collect()
	}
}
