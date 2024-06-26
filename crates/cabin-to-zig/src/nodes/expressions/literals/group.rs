use crate::TranspileToZig;

use std::fmt::Write as _;

use cabin::parser::expressions::literals::group::GroupDeclaration;

impl TranspileToZig for GroupDeclaration {
	fn to_zig(&self, context: &mut cabin::context::Context) -> String {
		let mut zig = "struct {".to_owned();
		for field in &self.fields {
			writeln!(
				zig,
				"\t{}: {}{},\n",
				field.type_annotation.as_ref().unwrap().to_zig(context),
				field.value.as_ref().map(|value| format!(" = {}", value.to_zig(context))).unwrap_or_default();
			)
			.unwrap();
		}
		zig.push('}');
		zig
	}
}
