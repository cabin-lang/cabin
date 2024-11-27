use std::collections::HashMap;

use colored::Colorize;
use try_as::traits as try_as_traits;

use crate::{
	api::{context::Context, scope::ScopeId, traits::TryAs},
	comptime::{memory::VirtualPointer, CompileTime},
	debug_log, if_then_some,
	lexer::{Span, TokenType},
	mapped_err, parse_list,
	parser::{
		expressions::{
			group::GroupDeclaration,
			literal::{LiteralConvertible as _, LiteralObject},
			name::Name,
			Expression,
		},
		statements::tag::TagList,
		ListType, Parse, TokenQueue, TokenQueueFunctionality,
	},
	transpiler::TranspileToC,
};

use super::{field_access::FieldAccessType, Spanned};

#[derive(Debug, Clone)]
pub struct ObjectConstructor {
	pub type_name: Name,
	pub fields: Vec<Field>,
	pub internal_fields: HashMap<String, InternalFieldValue>,
	pub outer_scope_id: ScopeId,
	pub inner_scope_id: ScopeId,
	pub field_access_type: FieldAccessType,
	pub name: Name,
	pub span: Span,
	pub tags: TagList,
}

#[derive(Debug, Clone)]
pub struct Field {
	pub name: Name,
	pub field_type: Option<Expression>,
	pub value: Option<Expression>,
}

pub trait Fields {
	fn add_or_overwrite_field(&mut self, field: Field);
}

impl Fields for Vec<Field> {
	fn add_or_overwrite_field(&mut self, field: Field) {
		while let Some(index) = self.iter().enumerate().find_map(|(index, other)| (other.name == field.name).then_some(index)) {
			self.remove(index);
		}
		self.push(field);
	}
}

impl ObjectConstructor {
	pub fn untyped(fields: Vec<Field>, span: Span, context: &Context) -> ObjectConstructor {
		ObjectConstructor {
			type_name: "Object".into(),
			name: "anonymous_object".into(),
			field_access_type: FieldAccessType::Normal,
			internal_fields: HashMap::new(),
			inner_scope_id: context.scope_data.unique_id(),
			outer_scope_id: context.scope_data.unique_id(),
			fields,
			tags: TagList::default(),
			span,
		}
	}

	pub fn typed<T: Into<Name>>(type_name: T, fields: Vec<Field>, span: Span, context: &Context) -> ObjectConstructor {
		ObjectConstructor {
			type_name: type_name.into(),
			name: "anonymous_object".into(),
			field_access_type: FieldAccessType::Normal,
			internal_fields: HashMap::new(),
			inner_scope_id: context.scope_data.unique_id(),
			outer_scope_id: context.scope_data.unique_id(),
			fields,
			tags: TagList::default(),
			span,
		}
	}

	pub fn string(string: &str, span: Span, context: &Context) -> ObjectConstructor {
		ObjectConstructor {
			type_name: Name::from("Text"),
			fields: Vec::new(),
			internal_fields: HashMap::from([("internal_value".to_owned(), InternalFieldValue::String(string.to_owned()))]),
			outer_scope_id: context.scope_data.file_id(),
			inner_scope_id: context.scope_data.file_id(),
			field_access_type: FieldAccessType::Normal,
			name: Name::non_mangled("anonymous_string_literal"),
			span,
			tags: TagList::default(),
		}
	}

	pub fn number(number: f64, span: Span, context: &Context) -> ObjectConstructor {
		ObjectConstructor {
			type_name: Name::from("Number"),
			fields: Vec::new(),
			internal_fields: HashMap::from([("internal_value".to_owned(), InternalFieldValue::Number(number))]),
			outer_scope_id: context.scope_data.file_id(),
			inner_scope_id: context.scope_data.file_id(),
			field_access_type: FieldAccessType::Normal,
			name: "anonymous_number".into(),
			span,
			tags: TagList::default(),
		}
	}

	pub fn is_literal(&self) -> bool {
		for field in &self.fields {
			let value = field.value.as_ref().unwrap();
			if let Expression::Pointer(_) = value {
				continue;
			}

			let Expression::ObjectConstructor(constructor) = value else {
				return false;
			};

			if !constructor.is_literal() {
				return false;
			}
		}

		true
	}
}

impl Parse for ObjectConstructor {
	type Output = ObjectConstructor;

	fn parse(tokens: &mut TokenQueue, context: &mut Context) -> anyhow::Result<Self::Output> {
		let start = tokens.pop(TokenType::KeywordNew)?.span;

		// Name
		let name = Name::parse(tokens, context)?;

		// Fields
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
			fields.add_or_overwrite_field(Field {
				name,
				value: Some(value),
				field_type: None,
			});
		})
		.span;

		// Return
		Ok(ObjectConstructor {
			type_name: name,
			fields,
			outer_scope_id: context.scope_data.unique_id(),
			inner_scope_id: context.scope_data.unique_id(),
			internal_fields: HashMap::new(),
			field_access_type: FieldAccessType::Normal,
			name: Name::non_mangled("anonymous_object"),
			span: start.to(&end),
			tags: TagList::default(),
		})
	}
}

impl CompileTime for ObjectConstructor {
	type Output = Expression;

	fn evaluate_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self::Output> {
		debug_log!(context, "Evaluating an object of type {} at compile-time", self.type_name.unmangled_name().bold().yellow());

		let mut fields = Vec::new();

		// Get object type
		let object_type = if_then_some!(!matches!(self.type_name.unmangled_name().as_str(), "Group" | "Module" | "Object"), {
			GroupDeclaration::from_literal(
				&self
					.type_name
					.clone()
					.evaluate_at_compile_time(context)
					.map_err(mapped_err! {
						while = format!("evaluating the type of an object constructor at compile time"),
						context = context,
					})?
					.try_as_literal(context)
					.cloned()
					.map_err(mapped_err! {
						while = format!("interpreting an object constructor's type (\"{}\") as a literal", self.type_name.unmangled_name().bold().cyan()),
						context = context,
					})?,
			)
			.map_err(mapped_err! {
				while = "converting an object constructor's type from a literal to a group declaration",
				context = context,
			})?
		});

		// Default fields
		if let Some(object_type) = object_type {
			for field in object_type.fields() {
				if let Some(value) = &field.value {
					fields.add_or_overwrite_field(Field {
						name: field.name.clone(),
						value: Some(value.clone()),
						field_type: None,
					});
				}
			}
		}

		// Explicit fields
		for field in self.fields {
			let field_value = field.value.unwrap();

			let previous = if self.type_name == "Module".into() {
				context.scope_data.set_current_scope(field_value.expect_as::<ObjectConstructor>().unwrap().inner_scope_id)
			} else {
				context.scope_data.unique_id()
			};

			let field_value = field_value.evaluate_at_compile_time(context).map_err(mapped_err! {
				while = format!(
					"evaluating the value of the field \"{}\" of an object at compile-time",
					field.name.unmangled_name().bold().cyan()
				),
				context = context,
			})?;

			context.scope_data.set_current_scope(previous);

			fields.add_or_overwrite_field(Field {
				name: field.name,
				value: Some(field_value),
				field_type: None,
			});
		}

		// Return the new object
		let constructor = ObjectConstructor {
			type_name: self.type_name,
			fields,
			outer_scope_id: self.outer_scope_id,
			inner_scope_id: self.inner_scope_id,
			internal_fields: self.internal_fields,
			field_access_type: self.field_access_type,
			name: self.name,
			span: self.span,
			tags: TagList::default(),
		};

		if constructor.is_literal() {
			let literal = LiteralObject::try_from_object_constructor(constructor, context)?;
			let address = context.virtual_memory.store(literal);
			Ok(Expression::Pointer(address))
		} else {
			Ok(Expression::ObjectConstructor(constructor))
		}
	}
}

#[derive(Debug, Clone, try_as::macros::TryInto, try_as::macros::TryAsRef)]
pub enum InternalFieldValue {
	Number(f64),
	String(String),
	Boolean(bool),
	ExpressionList(Vec<Expression>),
	Expression(Expression),
	OptionalExpression(Option<Expression>),
	FieldList(Vec<Field>),
	NameList(Vec<Name>),
	LiteralMap(Vec<(Name, VirtualPointer)>),
	ParameterList(Vec<(Name, Expression)>),
}

impl TranspileToC for ObjectConstructor {
	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		// Type name
		let name = if self.type_name == "Object".into() {
			format!("type_{}_UNKNOWN", self.name.to_c(context)?) // TODO
		} else {
			self.type_name.clone().evaluate_at_compile_time(context)?.to_c(context)?
		};

		let mut builder = format!("({}) {{", name);

		// Fields
		for field in &self.fields {
			builder += &format!("\n\t.{} = {},", field.name.to_c(context)?, field.value.as_ref().unwrap().to_c(context)?);
		}

		builder += "\n}";
		Ok(builder)
	}
}

impl Spanned for ObjectConstructor {
	fn span(&self, _context: &Context) -> Span {
		self.span.clone()
	}
}

#[macro_export]
macro_rules! object {
	(
		$context: expr,
		$type_name: ident {
			$(
				$field_name: ident = $field_value: expr
			),* $(,)?
		}
	) => {
		$crate::parser::expressions::object::ObjectConstructor {
			type_name: stringify!($type_name).into(),
			fields: vec![$($crate::parser::expressions::object::Field {
				name: stringify!($field_name),
				field_type: None,
				value: Some($field_value),
			}),*],
			internal_fields: std::collections::HashMap::new(),
			name: $crate::parser::expressions::name::Name::non_mangled("anonymous_object"),
			span: $crate::lexer::Span::unknown(),
			outer_scope_id: $context.scope_data.unique_id(),
			inner_scope_id: $context.scope_data.unique_id(),
			tags: $crate::parser::statements::tag::TagList::default(),
			field_access_type: $crate::parser::expressions::field_access::FieldAccessType::Normal,
		}
	};
}
