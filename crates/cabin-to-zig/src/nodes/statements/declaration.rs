use cabin::{context::Context, parser::statements::declaration::Declaration};

use crate::TranspileToZig;

impl TranspileToZig for Declaration {
	fn to_zig(&self, context: &mut Context) -> String {
		let value = context.scope_data.get_variable_from_id(&self.name, self.declared_scope_id).unwrap().value.clone().unwrap();
		format!("var {} = {};", self.name.c_name(), value.to_zig(context))
	}
}
