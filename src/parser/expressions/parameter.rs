use std::{collections::HashMap, fmt::Debug};

use crate::{
	api::{context::context, scope::ScopeId},
	bail_err,
	comptime::{memory::VirtualPointer, CompileTime},
	debug_log, debug_start,
	lexer::{Span, TokenType},
	parser::{statements::tag::TagList, Parse, TokenQueue, TokenQueueFunctionality},
};

use super::{
	field_access::FieldAccessType,
	literal::{LiteralConvertible, LiteralObject},
	name::Name,
	object::InternalFieldValue,
	Expression, Spanned, Typed,
};

#[derive(Clone)]
pub struct Parameter {
	name: Name,
	parameter_type: Box<Expression>,
	span: Span,
	scope_id: ScopeId,
}

impl Parse for Parameter {
	type Output = VirtualPointer;

	fn parse(tokens: &mut TokenQueue) -> anyhow::Result<Self::Output> {
		let name = Name::parse(tokens)?;
		tokens.pop(TokenType::Colon)?;
		let parameter_type = Expression::parse(tokens)?;
		Ok(Parameter {
			span: name.span().to(&parameter_type.span()),
			name,
			parameter_type: Box::new(parameter_type),
			scope_id: context().scope_data.unique_id(),
		}
		.to_literal()
		.store_in_memory())
	}
}

impl CompileTime for Parameter {
	type Output = Parameter;

	fn evaluate_at_compile_time(self) -> anyhow::Result<Self::Output> {
		let debug_section = debug_start!("{} a {}", "Compile-Time Evaluating".bold().green(), "parameter".cyan());
		debug_log!("Compile-Time Evaluating the type of a parameter");
		let evaluated = self.parameter_type.evaluate_as_type()?;

		if let Expression::Pointer(_) = &evaluated {
			// nothing to see here...
		} else {
			bail_err! {
				base = "A value that's not fully known at compile-time was used as a parameter type",
			}
		}

		let parameter = Parameter {
			name: self.name.clone(),
			parameter_type: Box::new(evaluated),
			span: self.span,
			scope_id: self.scope_id,
		};

		debug_section.finish();
		Ok(parameter)
	}
}

impl Spanned for Parameter {
	fn span(&self) -> Span {
		self.span.to_owned()
	}
}

impl Typed for Parameter {
	fn get_type(&self) -> anyhow::Result<VirtualPointer> {
		Ok(self.parameter_type.try_as_literal()?.address.unwrap())
	}
}

impl Parameter {
	pub fn name(&self) -> &Name {
		&self.name
	}

	pub fn parameter_type(&self) -> &Expression {
		&self.parameter_type
	}
}

impl LiteralConvertible for Parameter {
	fn to_literal(self) -> LiteralObject {
		LiteralObject {
			address: None,
			fields: HashMap::from([]),
			internal_fields: HashMap::from([("type".to_owned(), InternalFieldValue::Expression(*self.parameter_type))]),
			name: self.name,
			field_access_type: FieldAccessType::Group,
			outer_scope_id: self.scope_id,
			inner_scope_id: Some(self.scope_id),
			span: self.span,
			type_name: "Parameter".into(),
			tags: TagList::default(),
		}
	}

	fn from_literal(literal: &LiteralObject) -> anyhow::Result<Self> {
		Ok(Parameter {
			name: literal.name().to_owned(),
			parameter_type: Box::new(literal.get_internal_field::<Expression>("type")?.to_owned()),
			scope_id: literal.outer_scope_id(),
			span: literal.span(),
		})
	}
}

impl Debug for Parameter {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}: {:?}", self.name, self.parameter_type)
	}
}
