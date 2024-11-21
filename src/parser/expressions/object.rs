use std::collections::HashMap;

use colored::Colorize as _;
use try_as::traits::{self as try_as_traits, TryAsRef};

use crate::{
	api::{context::Context, traits::TryAs as _},
	bail_err, compiler_message,
	comptime::{memory::Pointer, CompileTime},
	lexer::{Position, TokenType},
	mapped_err, parse_list,
	parser::{
		expressions::{group::GroupDeclaration, name::Name, Expression},
		statements::tag::TagList,
		ListType, Parse, ToCabin, TokenQueue, TokenQueueFunctionality,
	},
};

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

	pub fn from_string(string: &str, context: &mut Context) -> Pointer {
		LiteralObject::try_from_object_constructor(
			ObjectConstructor {
				type_name: Name::from("Text"),
				fields: Vec::new(),
				internal_fields: HashMap::from([("internal_value".to_owned(), InternalFieldValue::String(string.to_owned()))]),
				scope_id: 0,
				object_type: ObjectType::Normal,
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
			let name = Name::parse(tokens, context).map_err(mapped_err! {
				while = "while attempting to parse an object constructor",
				context = context,
			})?;

			// Value
			tokens.pop(TokenType::Equal)?;
			let mut value = Expression::parse(tokens, context)?;

			// Set tags
			if let Some(expression_tags) = value.tags_mut() {
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

impl ToCabin for ObjectConstructor {
	fn to_cabin(&self) -> String {
		if self.type_name == "number".into() {
			return self.get_internal_field("internal_value").unwrap().expect_as::<f64>().to_string();
		}

		if self.type_name == "Text".into() {
			return self.get_internal_field("internal_value").unwrap().expect_as::<String>().to_owned();
		}

		todo!()
	}
}

impl CompileTime for ObjectConstructor {
	type Output = Expression;

	fn evaluate_at_compile_time(self, context: &mut Context) -> anyhow::Result<Self::Output> {
		let mut fields = Vec::new();

		// Get object type
		let object_type = GroupDeclaration::from_literal(
			context
				.scope_data
				.get_variable_from_id(self.type_name.clone(), self.scope_id)
				.ok_or_else(|| {
					anyhow::anyhow!(
						"Attempted to create an object of type \"{}\", but no type with that name was found in the scope it was referenced.",
						self.type_name.unmangled_name().bold().cyan()
					)
				})?
				.try_as_literal(context)?,
			context,
		)?;

		// Get `Anything`
		let anything = GroupDeclaration::from_literal(context.scope_data.expect_global_variable("Anything").expect_literal(context), context).unwrap();

		// Anything fields
		for field in anything.fields {
			if let Some(value) = field.value {
				fields.push(Field {
					name: field.name,
					value: Some(value),
					field_type: None,
				});
			}
		}

		// Default fields
		for field in object_type.fields {
			if let Some(value) = field.value {
				fields.push(Field {
					name: field.name,
					value: Some(value),
					field_type: None,
				});
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

#[derive(Debug, Clone, try_as::macros::TryInto, try_as::macros::TryAsRef)]
pub enum InternalFieldValue {
	Number(f64),
	String(String),
	Boolean(bool),
	List(Vec<Expression>),
	Expression(Expression),
	OptionalExpression(Option<Expression>),
}

#[derive(Debug)]
pub struct LiteralObject {
	pub type_name: Name,
	fields: HashMap<Name, Pointer>,
	internal_fields: HashMap<String, InternalFieldValue>,
	object_type: ObjectType,
	scope_id: usize,
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

			let name = value.kind_name();
			let Expression::ObjectConstructor(field_object) = value else {
				bail_err! {
					base = "A value that's not fully known at compile-time was used as a type.",
					while = format!("checking the field \"{}\" of a value at compile-time", field.name.unmangled_name().bold().cyan()),
					context = context,
					position = field.name.position().unwrap_or_else(Position::zero),
					details = compiler_message!(
						"
                        Although Cabin allows arbitrary expressions to be used as types, the expression needs to be able to 
						be fully evaluated at compile-time. The expression that this error refers to must be a literal object, 
						but instead it's a {name}. {}
						", 
						if &name.to_lowercase() == "name" {
							"
							This means that you put a variable name where a type is required, but the value of that variable
							is some kind of expression that can't be fully evaluated at compile-time.
							"
						} else {
							""
						}
					)
				};
			};

			let value_address = LiteralObject::try_from_object_constructor(field_object, context)?.store_in_memory(context);
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

	pub fn get_field(&self, name: impl Into<Name>) -> Option<Expression> {
		self.fields.get(&name.into()).map(|address| Expression::Pointer(*address))
	}

	pub fn get_field_literal<'a>(&'a self, name: impl Into<Name>, context: &'a Context) -> Option<&'a LiteralObject> {
		self.fields.get(&name.into()).and_then(|address| context.virtual_memory.get(*address))
	}

	pub fn expect_field_literal<'a>(&'a self, name: impl Into<Name>, context: &'a Context) -> &'a LiteralObject {
		self.get_field_literal(name, context).unwrap()
	}

	pub fn get_internal_field<T>(&self, name: &str) -> anyhow::Result<&T>
	where
		InternalFieldValue: TryAsRef<T>,
	{
		self.internal_fields
			.get(name)
			.ok_or_else(|| anyhow::anyhow!("Attempted to get an internal field that doesn't exist"))?
			.try_as::<T>()
	}

	pub fn pop_internal_field(&mut self, name: &str) -> Option<InternalFieldValue> {
		self.internal_fields.remove(name)
	}

	pub fn object_type(&self) -> &ObjectType {
		&self.object_type
	}

	/// Stores this value in virtual memory and returns the address of the location stored.
	pub fn store_in_memory(self, context: &mut Context) -> Pointer {
		context.virtual_memory.store(self)
	}

	pub fn declared_scope_id(&self) -> usize {
		self.scope_id
	}
}

impl TryAsRef<String> for LiteralObject {
	fn try_as_ref(&self) -> Option<&String> {
		self.get_internal_field("internal_value").ok()
	}
}

impl TryAsRef<f64> for LiteralObject {
	fn try_as_ref(&self) -> Option<&f64> {
		self.get_internal_field("internal_value").ok()
	}
}
impl TryAsRef<Vec<Expression>> for LiteralObject {
	fn try_as_ref(&self) -> Option<&Vec<Expression>> {
		self.get_internal_field("elements").ok()
	}
}

pub trait LiteralConvertible: Sized {
	fn to_literal(self, context: &mut Context) -> anyhow::Result<LiteralObject>;
	fn from_literal(literal: &LiteralObject, context: &Context) -> anyhow::Result<Self>;
}
