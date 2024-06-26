use cabin::parser::statements::Statement;

use crate::TranspileToZig;

mod declaration;

impl TranspileToZig for Statement {
	fn to_zig(&self, context: &mut Context) -> String {
		match self {
			Self::Declaration(declaration) => declaration.to_zig(context),
			_ => todo!(),
		}
	}
}
