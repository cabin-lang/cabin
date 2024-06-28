use crate::{
	cli::theme::Styled,
	compile_time::{CompileTime, TranspileToC},
	context::Context,
	formatter::{ColoredCabin, ToCabin},
	lexer::{Token, TokenType},
	object, parse_list,
	parser::{
		expressions::{
			literals::{Literal, LiteralValue},
			run::ParentExpression,
			util::{name::Name, types::Typed},
			Expression,
		},
		Parse, TokenQueue,
	},
	var_literal,
};

use std::{collections::VecDeque, fmt::Write as _};

use colored::Colorize as _;

/// An `either` declaration. This is analogous to an `enum` in other languages; It contains a list of property-less objects, and the variable that the `either` declaration is assigned to acts as a union
/// type among those variants.
#[derive(Debug, Clone)]
pub struct Either {
	/// The variants stored in this `either`.
	variants: Vec<(Name, Expression)>,
}

impl Either {
	/// Returns a reference to the variants stored in this `either`.
	///
	/// # Returns
	/// a slice of the variants stored in this `either`
	pub fn variants(&self) -> &[(Name, Expression)] {
		&self.variants
	}
}

impl Parse for Either {
	type Output = Self;

	fn parse(tokens: &mut VecDeque<Token>, context: &mut Context) -> anyhow::Result<Self::Output> {
		tokens.pop(TokenType::KeywordEither, context).map_err(|error| {
			anyhow::anyhow!(
				"{error}\n\t{}",
				format!(
					"while attempting to parse the keyword \"{either}\" at the start of an {either} declaration",
					either = "either".bold().cyan()
				)
				.dimmed()
			)
		})?;

		tokens.pop(TokenType::LeftBrace, context).map_err(|error| {
			anyhow::anyhow!(
				"{error}\n\t{}",
				format!(
					"while attempting to parse the opening left brace at the start of an {either} declaration",
					either = "either".bold().cyan()
				)
				.dimmed()
			)
		})?;

		let mut variants = Vec::new();
		if !tokens.next_is(TokenType::RightBrace) {
			parse_list!(tokens, context, {
				variants.push(Name(tokens.pop(TokenType::Identifier, context).map_err(|error| {
					anyhow::anyhow!("{error}\n\t{}", format!("while attempting to parse an {} variant", "either".bold().cyan()).dimmed())
				})?));
			});
		}

		tokens.pop(TokenType::RightBrace, context).map_err(|error| {
			anyhow::anyhow!(
				"{error}\n\t{}",
				format!(
					"while attempting to parse the closing right brace at the end of an {either} declaration",
					either = "either".bold().cyan()
				)
				.dimmed()
			)
		})?;

		Ok(Self {
			variants: variants.into_iter().map(|variant| (variant, object! { Object {}})).collect::<Vec<_>>(),
		})
	}
}

impl CompileTime for Either {
	fn compile_time_evaluate(&self, _context: &mut Context, _with_side_effects: bool) -> anyhow::Result<Expression> {
		Ok(Expression::Literal(Literal::new(LiteralValue::Either(self.clone()))))
	}
}

impl ParentExpression for Either {
	fn evaluate_children_at_compile_time(&self, context: &mut Context) -> anyhow::Result<Expression> {
		self.compile_time_evaluate(context, true)
	}
}

impl TranspileToC for Either {
	fn c_prelude(&self, _context: &mut Context) -> anyhow::Result<String> {
		Ok(String::new())
	}

	fn to_c(&self, _context: &mut Context) -> anyhow::Result<String> {
		let mut c = "{".to_owned();
		for field in self.variants() {
			write!(c, "\n\t{},", field.0.c_name()).unwrap();
		}
		c.push_str("\n}");
		Ok(c)
	}
}

impl ToCabin for Either {
	fn to_cabin(&self) -> String {
		let mut cabin = "either {".to_owned();
		for variant in self.variants() {
			write!(cabin, "\t{}", variant.0.cabin_name()).unwrap();
		}
		cabin.push('}');
		cabin
	}
}

impl ColoredCabin for Either {
	fn to_colored_cabin(&self, context: &mut Context) -> String {
		let mut cabin = format!("{} {{", "either".style(context.theme().keyword()));
		for variant in self.variants() {
			write!(cabin, "\t{}", variant.0.to_colored_cabin(context)).unwrap();
		}
		cabin.push('}');
		cabin
	}
}

impl Typed for Either {
	fn get_type(&self, _context: &mut Context) -> anyhow::Result<Literal> {
		Ok(var_literal!("Either", 0))
	}
}
