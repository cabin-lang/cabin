use cabin::{context::Context, parser::Program};

mod nodes;
pub trait TranspileToZig {
    fn to_zig(&self, context: &mut Context) -> String;
}

impl TranspileToZig for Program {
    fn to_zig(&self, context: &mut Context) -> String {
        self.statements
            .iter()
            .map(|statement| statement.to_zig(context))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
