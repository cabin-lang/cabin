use std::collections::HashMap;

use colored::Colorize;

use crate::{
	comptime::CompileTime,
	context::Context,
	lexer::TokenType,
	list, object, parse_list,
	parser::{statements::tag::TagList, ListType, TokenQueue, TokenQueueFunctionality},
	string,
};

use super::{super::Parse, name::Name, Expression};

#[derive(Debug, Clone)]
pub struct ObjectConstructor {
	pub type_name: Name,
	pub fields: Vec<Field>,
	pub internal_fields: HashMap<String, InternalFieldValue>,
	pub scope_id: usize,
	pub object_type: ObjectType,
}

#[derive(Debug, Clone)]
pub struct Field {
	pub name: Name,
	pub field_type: Option<Expression>,
	pub value: Option<Expression>,
}

impl ObjectConstructor {
	pub fn get_field(&self, name: &Name) -> Option<&Expression> {
		self.fields.iter().find_map(|field| if &field.name == name { field.value.as_ref() } else { None })
	}

	pub fn from_string(string: &str) -> ObjectConstructor {
		ObjectConstructor {
			type_name: Name::from("Text"),
			fields: Vec::new(),
			internal_fields: HashMap::from([("internal_value".to_owned(), InternalFieldValue::String(string.to_owned()))]),
			scope_id: 0,
			object_type: ObjectType::Normal,
		}
	}

	pub fn pop_internal_field(&mut self, name: &str) -> Option<InternalFieldValue> {
		self.internal_fields.remove(name)
	}

	pub fn as_string(&self) -> anyhow::Result<String> {
		let Some(InternalFieldValue::String(internal_value)) = self.internal_fields.get("internal_value") else {
			anyhow::bail!("Attempted to coerce a non-string into a string");
		};
		Ok(internal_value.to_owned())
	}

	pub fn group(fields: Vec<Field>, scope_id: usize, context: &mut Context) -> usize {
		let fields = fields
			.iter()
			.map(|field| {
				object! {
					Field {
						name = string!(&field.name.unmangled_name()),
						value = object! {
							Object {}, scope_id
						}
					},
					scope_id
				}
			})
			.collect();

		let constructor = ObjectConstructor {
			fields: vec![Field {
				name: "fields".into(),
				value: Some(list!(context, scope_id, fields)),
				field_type: None,
			}],
			scope_id,
			internal_fields: HashMap::new(),
			type_name: "Group".into(),
			object_type: ObjectType::Group,
		};

		let literal = LiteralObject::try_from_object_constructor(constructor, context).unwrap();
		context.virtual_memory.store(literal)
	}

	pub fn oneof(types: Vec<Expression>, scope_id: usize, context: &mut Context) -> usize {
		let constructor = ObjectConstructor {
			fields: vec![Field {
				name: "variants".into(),
				value: Some(list!(context, scope_id, types)),
				field_type: None,
			}],
			scope_id,
			internal_fields: HashMap::new(),
			type_name: "Either".into(),
			object_type: ObjectType::OneOf,
		};

		let literal = LiteralObject::try_from_object_constructor(constructor, context).unwrap();
		context.virtual_memory.store(literal)
	}

	fn is_literal(&self) -> bool {
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
		tokens.pop(TokenType::KeywordNew)?;

		// Name
		let name = Name::parse(tokens, context)?;

		// Fields
		let mut fields = Vec::new();
		parse_list!(tokens, ListType::Braced, {
			// Parse tags
			let tags = if tokens.next_is(TokenType::TagOpening) {
				Some(TagList::parse(tokens, context)?)
			} else {
				None
			};

			// Name
			let name = Name::parse(tokens, context).map_err(|error| anyhow::anyhow!("{error}\n\t{}", "while attempting to parse an object constructor".dimmed()))?;

			// Value
			tokens.pop(TokenType::Equal)?;
			let mut value = Expression::parse(tokens, context)?;

			// Set tags
			if let Some(expression_tags) = value.tags() {
				if let Some(declaration_tags) = &tags {
					*expression_tags = declaration_tags.clone();
				}
			}

			// Add field
			fields.push(Field {
				name,
				value: Some(value),
				field_type: None,
			});
		});

		// Return
		Ok(ObjectConstructor {
			type_name: name,
			fields,
			scope_id: context.scope_data.unique_id(),
			internal_fields: HashMap::new(),
			object_type: ObjectType::Normal,
		})
	}
}

impl CompileTime for ObjectConstructor {
	type Output = Expression;

	fn evaluate_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self::Output> {
		let mut fields = Vec::new();
		for field in self.fields {
			let field_value = field.value.unwrap().evaluate_at_compile_time(context).map_err(|error| {
				anyhow::anyhow!(
					"{error}\n\t{}",
					format!(
						"while evaluating the value of the field \"{}\" of an object at compile-time",
						field.name.unmangled_name().bold().cyan()
					)
					.dimmed()
				)
			})?;

			fields.push(Field {
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

#[derive(Debug, Clone)]
pub enum InternalFieldValue {
	Number(f64),
	String(String),
	Boolean(bool),
	List(Vec<Expression>),
	Expression(Expression),
	OptionalExpression(Option<Expression>),
}

impl InternalFieldValue {
	pub fn as_optional_expression(self) -> anyhow::Result<Option<Expression>> {
		if let Self::OptionalExpression(expression) = self {
			Ok(expression)
		} else {
			anyhow::bail!("Attempted to convert a non-optional-expression internal field value into an optional expression internal field value");
		}
	}
}

#[derive(Debug)]
pub struct LiteralObject {
	pub type_name: Name,
	fields: HashMap<Name, usize>,
	internal_fields: HashMap<String, InternalFieldValue>,
	object_type: ObjectType,
	pub scope_id: usize,
}

impl LiteralObject {
	pub fn try_from_object_constructor(object: ObjectConstructor, context: &mut Context) -> anyhow::Result<Self> {
		let mut fields = HashMap::new();
		for field in object.fields {
			let value = field.value.unwrap();
			if let Expression::Pointer(address) = value {
				fields.insert(field.name, address);
				continue;
			}

			let Expression::ObjectConstructor(field_object) = value else {
				anyhow::bail!(
					"{}\n\t{}",
					"A value that's not fully known at compile-time was used as a type.".bold().white(),
					format!("while checking the field \"{}\" of the value at compile-time", field.name.unmangled_name().bold().cyan()).dimmed()
				);
			};

			let literal = LiteralObject::try_from_object_constructor(field_object, context)?;
			let value_address = context.virtual_memory.store(literal);
			fields.insert(field.name, value_address);
		}

		Ok(LiteralObject {
			type_name: object.type_name,
			fields,
			internal_fields: object.internal_fields,
			object_type: object.object_type,
			scope_id: object.scope_id,
		})
	}

	pub fn get_field(&self, name: &Name) -> Option<Expression> {
		self.fields.get(name).map(|address| Expression::Pointer(*address))
	}

	pub fn get_field_literal<'a>(&'a self, name: &Name, context: &'a Context) -> Option<&'a LiteralObject> {
		self.fields.get(name).and_then(|address| context.virtual_memory.get(*address))
	}

	pub fn get_internal_field(&self, name: &str) -> Option<&InternalFieldValue> {
		self.internal_fields.get(name)
	}

	pub fn pop_internal_field(&mut self, name: &str) -> Option<InternalFieldValue> {
		self.internal_fields.remove(name)
	}

	pub fn object_type(&self) -> &ObjectType {
		&self.object_type
	}

	pub fn list_elements(&self) -> anyhow::Result<&[Expression]> {
		let InternalFieldValue::List(elements) = self.get_internal_field("elements").unwrap() else {
			unreachable!()
		};
		Ok(elements)
	}

	pub fn as_string(&self) -> anyhow::Result<&String> {
		let InternalFieldValue::String(internal_value) = self.get_internal_field("internal_value").unwrap() else {
			unreachable!()
		};
		Ok(internal_value)
	}

	/// Stores this value in virtual memory and returns the address of the location stored.
	pub fn store_in_memory(self, context: &mut Context) -> usize {
		context.virtual_memory.store(self)
	}
}

pub trait LiteralConvertible: Sized {
	fn to_literal(self, context: &mut Context) -> anyhow::Result<LiteralObject>;
	fn from_literal(literal: &LiteralObject, context: &Context) -> anyhow::Result<Self>;
}
