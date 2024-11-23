use std::collections::HashMap;

use try_as::traits as try_as_traits;

use crate::{
	api::{context::Context, traits::TryAs as _},
	comptime::{memory::VirtualPointer, CompileTime},
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
		ListType, Parse, ToCabin, TokenQueue, TokenQueueFunctionality,
	},
	transpiler::TranspileToC,
};

use super::Spanned;

#[derive(Debug, Clone)]
pub struct ObjectConstructor {
	pub type_name: Name,
	pub fields: Vec<Field>,
	pub internal_fields: HashMap<String, InternalFieldValue>,
	pub scope_id: usize,
	pub object_type: ObjectType,
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
	fn add_field(&mut self, field: Field);
}

impl Fields for Vec<Field> {
	fn add_field(&mut self, field: Field) {
		while let Some(index) = self.iter().enumerate().find_map(|(index, other)| (other.name == field.name).then_some(index)) {
			self.remove(index);
		}
		self.push(field);
	}
}

impl ObjectConstructor {
	pub fn get_field(&self, name: &Name) -> Option<&Expression> {
		self.fields.iter().find_map(|field| if &field.name == name { field.value.as_ref() } else { None })
	}

	pub fn from_string(string: &str, context: &mut Context) -> VirtualPointer {
		LiteralObject::try_from_object_constructor(
			ObjectConstructor {
				type_name: Name::from("Text"),
				fields: Vec::new(),
				internal_fields: HashMap::from([("internal_value".to_owned(), InternalFieldValue::String(string.to_owned()))]),
				scope_id: 0,
				object_type: ObjectType::Normal,
				name: Name::non_mangled("anonymous_string_literal"),
				span: Span::zero(),
				tags: TagList::default(),
			},
			context,
		)
		.unwrap()
		.store_in_memory(context)
	}

	pub fn from_number(number: f64) -> ObjectConstructor {
		ObjectConstructor {
			type_name: Name::from("Number"),
			fields: Vec::new(),
			internal_fields: HashMap::from([("internal_value".to_owned(), InternalFieldValue::Number(number))]),
			scope_id: 0,
			object_type: ObjectType::Normal,
			name: "anonymous_number".into(),
			span: Span::zero(),
			tags: TagList::default(),
		}
	}

	pub fn pop_internal_field(&mut self, name: &str) -> Option<InternalFieldValue> {
		self.internal_fields.remove(name)
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

	pub fn get_internal_field(&self, name: &str) -> Option<&InternalFieldValue> {
		self.internal_fields.get(name)
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
			fields.add_field(Field {
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
			scope_id: context.scope_data.unique_id(),
			internal_fields: HashMap::new(),
			object_type: ObjectType::Normal,
			name: Name::non_mangled("anonymous_object"),
			span: start.to(&end),
			tags: TagList::default(),
		})
	}
}

impl ToCabin for ObjectConstructor {
	fn to_cabin(&self) -> String {
		if self.type_name == "number".into() {
			return self.get_internal_field("internal_value").unwrap().expect_as::<f64>().unwrap().to_string();
		}

		if self.type_name == "Text".into() {
			return self.get_internal_field("internal_value").unwrap().expect_as::<String>().unwrap().to_owned();
		}

		todo!()
	}
}

impl CompileTime for ObjectConstructor {
	type Output = Expression;

	fn evaluate_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self::Output> {
		let mut fields = Vec::new();

		// Get object type
		let object_type = if self.type_name == "Group".into() {
			None
		} else {
			Some(
				GroupDeclaration::from_literal(
					&self
						.type_name
						.clone()
						.evaluate_at_compile_time(context)
						.map_err(mapped_err! {
							while = format!("evaluating the type of an object constructor at compile time"),
							context = context,
						})?
						.try_as_literal_or_name(context)
						.cloned()
						.map_err(mapped_err! {
							while = format!("interpreting an object constructor's type (\"{}\") as a literal", self.type_name.unmangled_name().bold().cyan()),
							context = context,
						})?,
				)
				.map_err(mapped_err! {
					while = "converting an object constructor's type from a literal to a group declaration",
					context = context,
				})?,
			)
		};

		// Get `Anything`
		let anything = GroupDeclaration::from_literal(&Name::from("Anything").evaluate_at_compile_time(context)?.try_as_literal_or_name(context).cloned().map_err(
			mapped_err! {
				while = format!("interpreting the value of \"{}\" as a literal", "Anything".bold().cyan()),
				context = context,
			},
		)?)
		.unwrap();

		// Anything fields
		for field in anything.fields {
			if let Some(value) = field.value {
				fields.add_field(Field {
					name: field.name,
					value: Some(value),
					field_type: None,
				});
			}
		}

		// Default fields
		if let Some(object_type) = object_type {
			for field in object_type.fields {
				if let Some(value) = field.value {
					fields.add_field(Field {
						name: field.name,
						value: Some(value),
						field_type: None,
					});
				}
			}
		}

		// Explicit fields
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

		// Return the new object
		let constructor = ObjectConstructor {
			type_name: self.type_name,
			fields,
			scope_id: self.scope_id,
			internal_fields: self.internal_fields,
			object_type: self.object_type,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectType {
	Normal,
	Group,
	OneOf,
	Either,
	Function,
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
