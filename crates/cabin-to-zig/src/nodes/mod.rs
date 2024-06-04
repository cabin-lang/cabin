use cabin::{context::Context, parser::statements::Statement};

use crate::TranspileToZig;

mod expressions;

mod statements;

impl TranspileToZig for Statement {
    fn to_zig(&self, context: &mut Context) -> String {
        match self {
            Self::Declaration(declaration) => declaration.to_zig(context),
            _ => todo!(),
        }
    }
}
