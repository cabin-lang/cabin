use cabin::parser::expressions::Expression;

use crate::TranspileToZig;

mod literals;

impl TranspileToZig for Expression {
    fn to_zig(&self, context: &mut cabin::context::Context) -> String {
        match self {
            Self::Literal(literal) => literal.to_zig(context),
            _ => todo!(),
        }
    }
}
