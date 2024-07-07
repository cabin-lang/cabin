use crate::{
	cli::theme::Styled,
	compile_time::{CompileTime, CompileTimeStatement, TranspileToC},
	context::Context,
	formatter::{ColoredCabin, ToCabin},
	parser::{
		expressions::{util::name::Name, Expression},
		statements::Statement,
	},
};

#[derive(Debug, Clone)]
pub struct Reassignment {
	pub name: Name,
	pub value: Expression,
}

impl CompileTimeStatement for Reassignment {
	fn compile_time_evaluate_statement(&self, context: &mut Context, with_side_effects: bool) -> anyhow::Result<Statement> {
		Ok(Statement::Reassignment(Self {
			name: self.name.clone(),
			value: self.value.compile_time_evaluate(context, with_side_effects)?,
		}))
	}
}

impl TranspileToC for Reassignment {
	fn c_prelude(&self, context: &mut Context) -> anyhow::Result<String> {
		self.value.c_prelude(context)
	}

	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		Ok(format!("{} = {};", self.name.mangled_name(), self.value.to_c(context)?))
	}
}

impl ToCabin for Reassignment {
	fn to_cabin(&self) -> String {
		format!("{} = {};", self.name.unmangled_name(), self.value.to_cabin())
	}
}

impl ColoredCabin for Reassignment {
	fn to_colored_cabin(&self, context: &mut Context) -> String {
		format!(
			"{} = {};",
			self.name.unmangled_name().style(context.theme().variable_name()),
			self.value.to_colored_cabin(context)
		)
	}
}
