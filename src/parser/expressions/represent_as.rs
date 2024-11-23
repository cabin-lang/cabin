use crate::{
	api::context::Context,
	comptime::CompileTime,
	lexer::{Span, TokenType},
	mapped_err, parse_list,
	parser::{expressions::object::Fields as _, statements::tag::TagList, ListType, Parse, TokenQueue, TokenQueueFunctionality as _},
};

use super::{name::Name, object::Field, Expression, Spanned};

#[derive(Debug, Clone)]
pub struct RepresentAs {
	type_to_represent: Expression,
	type_to_represent_as: Expression,
	fields: Vec<Field>,
	name: Name,
	span: Span,
}

impl Parse for RepresentAs {
	type Output = RepresentAs;

	fn parse(tokens: &mut TokenQueue, context: &mut Context) -> anyhow::Result<Self::Output> {
		let start = tokens.pop(TokenType::KeywordRepresent)?.span;
		let type_to_represent = Expression::parse(tokens, context)?;
		tokens.pop(TokenType::KeywordAs)?;
		let type_to_represent_as = Expression::parse(tokens, context)?;

		let mut fields = Vec::new();
		let end = parse_list!(tokens, ListType::Braced, {
			// Parse tags
			let tags = if tokens.next_is(TokenType::TagOpening) {
				Some(TagList::parse(tokens, context)?)
			} else {
				None
			};

			// Name
			let name = Name::parse(tokens, context).map_err(mapped_err! {
				while = "attempting to parse an object constructor",
				context = context,
			})?;

			// Value
			tokens.pop(TokenType::Equal)?;
			let mut value = Expression::parse(tokens, context)?;

			// Set tags
			if let Some(tags) = tags.clone() {
				value.set_tags(tags, context);
			}

			// Add field
			fields.add_field(Field {
				name,
				value: Some(value),
				field_type: None,
			});
		})
		.span;

		Ok(RepresentAs {
			type_to_represent,
			type_to_represent_as,
			fields,
			span: start.to(&end),
			name: "anonymous_represent_as".into(),
		})
	}
}

impl CompileTime for RepresentAs {
	type Output = RepresentAs;

	fn evaluate_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self::Output> {
		let type_to_represent = self.type_to_represent.evaluate_at_compile_time(context)?;
		let type_to_represent_as = self.type_to_represent_as.evaluate_at_compile_time(context)?;

		let mut fields = Vec::new();

		for field in self.fields {
			let field_value = field.value.unwrap().evaluate_at_compile_time(context).map_err(mapped_err! {
				while = format!(
					"evaluating the value of the field \"{}\" of an object at compile-time",
					field.name.unmangled_name().bold().cyan()
				),
				context = context,
			})?;

			fields.add_field(Field {
				name: field.name,
				value: Some(field_value),
				field_type: None,
			});
		}

		Ok(RepresentAs {
			type_to_represent,
			type_to_represent_as,
			name: self.name,
			span: self.span,
			fields,
		})
	}
}

impl RepresentAs {
	pub fn type_to_represent(&self) -> &Expression {
		&self.type_to_represent
	}

	pub fn type_to_represent_as(&self) -> &Expression {
		&self.type_to_represent_as
	}

	pub fn fields(&self) -> &[Field] {
		&self.fields
	}
}

impl Spanned for RepresentAs {
	fn span(&self, _context: &Context) -> Span {
		self.span.clone()
	}
}
