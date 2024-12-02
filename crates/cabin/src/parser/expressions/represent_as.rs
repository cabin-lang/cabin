use std::collections::HashMap;

use crate::{
	api::{
		context::context,
		scope::{ScopeId, ScopeType},
		traits::TryAs,
	},
	comptime::{memory::VirtualPointer, CompileTime},
	if_then_else_default, if_then_some,
	lexer::{Span, TokenType},
	mapped_err, parse_list,
	parser::{
		expressions::{
			name::Name,
			object::{Field, Fields as _},
			parameter::Parameter,
			Expression, Spanned,
		},
		statements::tag::TagList,
		ListType, Parse, TokenQueue, TokenQueueFunctionality as _,
	},
};

use super::{
	field_access::FieldAccessType,
	literal::{LiteralConvertible, LiteralObject},
	object::InternalFieldValue,
	Typed,
};

#[derive(Debug, Clone)]
pub struct RepresentAs {
	type_to_represent: Box<Expression>,
	type_to_represent_as: Box<Expression>,
	fields: Vec<Field>,
	name: Name,
	span: Span,
	compile_time_parameters: Vec<VirtualPointer>,
	inner_scope_id: ScopeId,
	outer_scope_id: ScopeId,
}

impl Parse for RepresentAs {
	type Output = VirtualPointer;

	fn parse(tokens: &mut TokenQueue) -> anyhow::Result<Self::Output> {
		let start = tokens.pop(TokenType::KeywordRepresent)?.span;
		let outer_scope_id = context().scope_data.unique_id();

		context().scope_data.enter_new_scope(ScopeType::RepresentAs);
		let inner_scope_id = context().scope_data.unique_id();

		let compile_time_parameters = if_then_else_default!(tokens.next_is(TokenType::LeftAngleBracket), {
			let mut parameters = Vec::new();
			let _ = parse_list!(tokens, ListType::AngleBracketed, {
				let parameter = Parameter::parse(tokens)?;
				context().scope_data.declare_new_variable(
					Parameter::from_literal(parameter.virtual_deref()).unwrap().name().to_owned(),
					Expression::Pointer(parameter),
				)?;
				parameters.push(parameter);
			});
			parameters
		});

		let type_to_represent = Box::new(Expression::parse(tokens)?);
		let _ = tokens.pop(TokenType::KeywordAs)?;
		let type_to_represent_as = Box::new(Expression::parse(tokens)?);

		let mut fields = Vec::new();
		let end = parse_list!(tokens, ListType::Braced, {
			// Parse tags
			let tags = if_then_some!(tokens.next_is(TokenType::TagOpening), TagList::parse(tokens)?);

			// Name
			let name = Name::parse(tokens).map_err(mapped_err! {
				while = "attempting to parse an object constructor",
			})?;

			// Value
			let _ = tokens.pop(TokenType::Equal)?;
			let mut value = Expression::parse(tokens)?;

			// Set tags
			if let Some(tags) = tags {
				value.set_tags(tags);
			}

			// Add field
			fields.add_or_overwrite_field(Field {
				name,
				value: Some(value),
				field_type: None,
			});
		})
		.span;

		context().scope_data.exit_scope()?;

		Ok(RepresentAs {
			type_to_represent,
			type_to_represent_as,
			fields,
			span: start.to(end),
			name: "anonymous_represent_as".into(),
			compile_time_parameters,
			inner_scope_id,
			outer_scope_id,
		}
		.to_literal()
		.store_in_memory())
	}
}

impl CompileTime for RepresentAs {
	type Output = RepresentAs;

	fn evaluate_at_compile_time(self) -> anyhow::Result<Self::Output> {
		let _scope_reverter = context().scope_data.set_current_scope(self.inner_scope_id);

		let type_to_represent = Box::new(self.type_to_represent.evaluate_at_compile_time().map_err(mapped_err! {
			while = "evaluating the type to represent in a represent-as declaration at compile-time",
		})?);
		let type_to_represent_as = Box::new(self.type_to_represent_as.evaluate_at_compile_time().map_err(mapped_err! {
			while = "evaluating the type to represent as in a represent-as declaration at compile-time",
		})?);

		let mut fields = Vec::new();

		for field in self.fields {
			let field_value = field.value.unwrap().evaluate_at_compile_time().map_err(mapped_err! {
				while = format!(
					"evaluating the value of the field \"{}\" of a represent-as declaration at compile-time",
					field.name.unmangled_name().bold().cyan()
				),
			})?;

			fields.add_or_overwrite_field(Field {
				name: field.name,
				value: Some(field_value),
				field_type: None,
			});
		}

		// Evaluate compile-time parameters
		let compile_time_parameters = self
			.compile_time_parameters
			.into_iter()
			.map(|parameter| parameter.evaluate_at_compile_time())
			.collect::<anyhow::Result<Vec<_>>>()
			.map_err(mapped_err! {
				while = "evaluating the compile-time parameters of a represent-as declaration at compile-time",
			})?;

		Ok(RepresentAs {
			type_to_represent,
			type_to_represent_as,
			name: self.name,
			span: self.span,
			fields,
			inner_scope_id: self.inner_scope_id,
			outer_scope_id: self.outer_scope_id,
			compile_time_parameters,
		})
	}
}

impl RepresentAs {
	pub const fn type_to_represent(&self) -> &Expression {
		&self.type_to_represent
	}

	pub const fn type_to_represent_as(&self) -> &Expression {
		&self.type_to_represent_as
	}

	pub fn fields(&self) -> &[Field] {
		&self.fields
	}

	pub fn can_represent(&self, object: &Expression) -> anyhow::Result<bool> {
		let _scope_reverter = context().scope_data.set_current_scope(self.inner_scope_id);

		if let Expression::Pointer(pointer) = self.type_to_represent.as_ref() {
			let literal = pointer.virtual_deref();
			if literal.type_name() == &"Parameter".into() {
				let parameter = Parameter::from_literal(literal).unwrap();
				let anything: VirtualPointer = *context().scope_data.get_variable("Anything").unwrap().try_as::<VirtualPointer>()?;
				let parameter_type = parameter.get_type()?;
				if parameter_type == anything || object.is_assignable_to_type(parameter_type)? {
					return Ok(true);
				}
			}
		}

		Ok(false)
	}

	pub fn representables(&self) -> anyhow::Result<String> {
		let _scope_reverter = context().scope_data.set_current_scope(self.inner_scope_id);

		if let Expression::Name(name) = self.type_to_represent.as_ref() {
			if let Expression::Parameter(parameter) = context().scope_data.get_variable(name).unwrap() {
				let parameter_type = parameter.get_type()?;
				return Ok(parameter_type.virtual_deref().name().unmangled_name().to_owned());
			}
		}

		Ok("unknown".to_owned())
	}

	pub fn set_name(&mut self, name: Name) {
		self.name = name.clone();
		self.fields.iter_mut().for_each(|field| {
			field.name = format!("{}_{}", name.unmangled_name(), field.name.unmangled_name()).into();
			if let Some(value) = &mut field.value {
				value.try_set_name(field.name.clone());
			}
		});
	}
}

impl LiteralConvertible for RepresentAs {
	fn to_literal(self) -> LiteralObject {
		LiteralObject {
			address: None,
			fields: HashMap::from([]),
			internal_fields: HashMap::from([
				("fields".to_owned(), InternalFieldValue::FieldList(self.fields)),
				("type_to_represent".to_owned(), InternalFieldValue::Expression(*self.type_to_represent)),
				("type_to_represent_as".to_owned(), InternalFieldValue::Expression(*self.type_to_represent_as)),
				("compile_time_parameters".to_owned(), InternalFieldValue::PointerList(self.compile_time_parameters)),
			]),
			name: self.name,
			field_access_type: FieldAccessType::Normal,
			outer_scope_id: self.outer_scope_id,
			inner_scope_id: Some(self.inner_scope_id),
			span: self.span,
			type_name: "RepresentAs".into(),
			tags: TagList::default(),
		}
	}

	fn from_literal(literal: &LiteralObject) -> anyhow::Result<Self> {
		Ok(RepresentAs {
			fields: literal.get_internal_field::<Vec<Field>>("fields")?.to_owned(),
			type_to_represent: Box::new(literal.get_internal_field::<Expression>("type_to_represent")?.to_owned()),
			type_to_represent_as: Box::new(literal.get_internal_field::<Expression>("type_to_represent_as")?.to_owned()),
			compile_time_parameters: literal.get_internal_field::<Vec<VirtualPointer>>("compile_time_parameters")?.to_owned(),
			outer_scope_id: literal.outer_scope_id(),
			inner_scope_id: literal.inner_scope_id.unwrap(),
			name: literal.name.clone(),
			span: literal.span,
		})
	}
}

impl Spanned for RepresentAs {
	fn span(&self) -> Span {
		self.span
	}
}
