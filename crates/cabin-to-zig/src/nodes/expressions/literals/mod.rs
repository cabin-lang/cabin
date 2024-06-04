use cabin::{
    context::Context,
    parser::expressions::literals::{Literal, LiteralValue},
};

use crate::TranspileToZig;

mod group;

impl TranspileToZig for Literal {
    fn to_zig(&self, context: &mut Context) -> String {
        match self.value() {
            LiteralValue::Group(group) => group.to_zig(context),
            _ => todo!(),
        }
    }
}
