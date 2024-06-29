use std::collections::VecDeque;

use crate::{
	cli::theme::Styled,
	compile_time::{CompileTime, CompileTimeStatement, TranspileToC},
	context::Context,
	formatter::{ColoredCabin, ToCabin},
	global_var,
	lexer::{Token, TokenType},
	parser::{
		expressions::{
			block::Block,
			util::{name::Name, tags::TagList},
			Expression,
		},
		statements::Statement,
		Parse, TokenQueue,
	},
};

/// A for-in loop. For loops in Cabin are always `foreach <ITEM> in <ITEMS>`. Ranges are iterated such as `foreach index in 0.to(10)`, which creates a range iterator.
#[derive(Debug, Clone)]
pub struct ForEachLoop {
	/// The name of the variable binding created in the for loop.
	name: Name,

	/// The expression being iterated over. This must be a `List`.
	iterator: Expression,

	/// The body of the for loop.
	body: Block,
}

impl Parse for ForEachLoop {
	type Output = Self;

	fn parse(tokens: &mut VecDeque<Token>, context: &mut Context) -> anyhow::Result<Self::Output> {
		tokens.pop(TokenType::KeywordForEach, context)?;
		let name = Name(tokens.pop(TokenType::Identifier, context)?);
		tokens.pop(TokenType::KeywordIn, context)?;
		let iterator = Expression::parse(tokens, context)?;
		let body = Block::parse(tokens, context)?;
		context
			.scope_data
			.declare_new_variable_from_id(name.clone(), None, global_var!("Parameter"), TagList::default(), body.inner_scope_id)?;
		Ok(Self { name, iterator, body })
	}
}

impl CompileTimeStatement for ForEachLoop {
	fn compile_time_evaluate_statement(&self, context: &mut Context, with_side_effects: bool) -> anyhow::Result<Statement> {
		let iterator = self.iterator.compile_time_evaluate(context, with_side_effects)?;
		let Expression::Block(body) = self.body.compile_time_evaluate(context, with_side_effects)? else {
			unreachable!()
		};

		Ok(Statement::ForEachLoop(Self {
			name: self.name.clone(),
			iterator,
			body,
		}))
	}
}

impl TranspileToC for ForEachLoop {
	fn c_prelude(&self, context: &mut Context) -> anyhow::Result<String> {
		Ok([self.iterator.c_prelude(context)?, self.body.c_prelude(context)?].join("\n"))
	}

	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		Ok(format!(
			"List_u* collection = {};\nfor (int index = 0; index < collection->size; index++) {}",
			self.iterator.to_c(context)?,
			{
				let body = self.body.to_c(context)?;
				format!(
					"{{\n\tvoid* {name};\n\tNumber_u index_u = (Number_u){{ .equals_u=equals_3 }};\n\tcollection->get_u(collection, index_u, &{name});{}",
					body.get(2..body.len() - 1).unwrap().to_owned(),
					name = self.name.c_name(),
				)
			}
		))
	}
}

impl ToCabin for ForEachLoop {
	fn to_cabin(&self) -> String {
		format!("foreach {} in {} {}", self.name.cabin_name(), self.iterator.to_cabin(), self.body.to_cabin())
	}
}

impl ColoredCabin for ForEachLoop {
	fn to_colored_cabin(&self, context: &mut Context) -> String {
		format!(
			"{} {} {} {} {}",
			"foreach".style(context.theme().keyword()),
			self.name.cabin_name(),
			"in".style(context.theme().keyword()),
			self.iterator.to_cabin(),
			self.body.to_cabin()
		)
	}
}
