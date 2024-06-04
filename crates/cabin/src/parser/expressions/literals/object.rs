use crate::{
	cli::theme::Styled,
	compile_time::{CompileTime, TranspileToC},
	context::Context,
	formatter::{ColoredCabin, ToCabin},
	lexer::{Token, TokenType},
	parse_list,
	parser::{
		expressions::{
			literals::{Literal, LiteralValue, Name},
			run::ParentExpression,
			util::{tags::TagList, types::Typed},
			Expression,
		},
		Parse, TokenQueue,
	},
	scopes::DeclarationData,
	var_literal,
};

use std::{collections::HashMap, fmt::Write as _, sync::atomic::AtomicUsize};

use colored::Colorize as _;

use super::group::GroupType;

/// An internal value stored in a Cabin object that's not visible to the user (developer). This is used for things
/// like `Text` and `Number` that need to store internal data (the underlying `String` / `f64`) that can't be represented
/// in Cabin code.
#[derive(Debug, Clone)]
pub enum InternalValue {
	/// A string internal value. This is used to store the underlying string on `Text` objects.
	String(String),
	/// A number internal value. This is used to store the underlying float on `Number` objects.
	Number(f64),
	/// A list internal value. This is used to store the underlying elements on `List` objects.
	List(Vec<Expression>),
}

/// A table literal in the language. Table literals are groups of types, similar to a JS object or a Rust struct
#[derive(Clone, Debug)]
pub struct Object {
	/// The fields in this table. This is a map between `String` names and `Expression` values.
	pub fields: Vec<DeclarationData>,
	/// The name of this table. This is the name of the type declaration, similar to how structs are declared in Rust.
	pub name: Name,

	/// "Internal" values stored in this table. These are values that ar not present in the Cabin code, but are used internally. For example,
	/// the `Number` table has an internal value that holds the number it represents.
	pub internal_fields: HashMap<String, InternalValue>,

	/// Returns the ID of this object if it's anonymous, otherwise `None` if it's named. Objects which are created with `new Object` are "anonymous" in the sense that they can contain any
	/// fields without needing a specified type. When converting to C code, these each need a struct definition, so we need to be able to distinguish these with a unique ID.
	anonymous_id: Option<usize>,

	/// Whether this object has already gone through compile-time evaluation. This prevents double compile-time evaluating an object, which in theory wouldn't do anything anyway and
	/// would be unnecessary overhead, but also could cause unexpected bugs and issues.
	has_been_compile_time_evaluated: bool,
}

/// The next unique ID for an anonymous table.
static TABLE_ID: AtomicUsize = AtomicUsize::new(0);

impl Object {
	/// Creates a new empty table.
	///
	/// # Returns
	/// A new empty table.
	pub fn new() -> Self {
		Self {
			fields: Vec::new(),
			name: Name(String::new()),
			internal_fields: HashMap::new(),
			anonymous_id: None,
			has_been_compile_time_evaluated: false,
		}
	}

	/// Creates a new object with the given name. If it is Text, the internal fields are added accordingly to "uninitialized" placeholder values. This should really only be used when generating
	/// return address variables, which have no need for an initial value.
	///
	/// # Parameters
	/// - `name` - The name of the object type to create
	///
	/// # Returns
	/// The newly created object.
	pub fn named(name: Name) -> Self {
		Self {
			fields: Vec::new(),
			internal_fields: if name == Name("Text".to_owned()) {
				HashMap::from([("internal_value".to_owned(), InternalValue::String("uninitialized".to_owned()))])
			} else {
				HashMap::new()
			},
			name,
			anonymous_id: None,
			has_been_compile_time_evaluated: false,
		}
	}

	/// Gives the table a unique ID. This is used for anonymous tables.
	pub fn make_anonymous(&mut self) {
		let id = TABLE_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
		self.anonymous_id = Some(id);
		self.name = Name("<anonymous>".to_owned());
	}

	/// Returns whether this is an anonymous object, i.e., an object created with `new Object`.
	///
	/// # Returns
	/// `true` iff this is an anonymous object.
	pub const fn is_anonymous(&self) -> bool {
		self.anonymous_id.is_some()
	}

	/// Adds an internal field to this object. If a field with the given name already exists, the new value provided
	/// will override the existing internal field.
	///
	/// # Parameters
	/// - `name` - The name of the field to add
	/// - `value` - The value of the field to add
	pub fn add_internal_field(&mut self, name: String, value: InternalValue) {
		self.internal_fields.insert(name, value);
	}

	/// Adds a field to this table.
	///
	/// # Parameters
	/// - `name`: The name of the field to add.
	/// - `value`: The value of the field to add.
	/// - `tags`: The tags of the field to add.
	pub fn add_field(&mut self, field: DeclarationData) {
		self.fields.push(field);
	}

	/// Gets the value of the field with the given name from this table.
	///
	/// # Parameters
	/// - `name`: the name of the field to get.
	///
	/// # Returns
	/// The value of the field with the given name from this table, or `None` if no such field exists.
	pub fn get_field(&self, name: &Name) -> Option<&Expression> {
		self.fields.iter().find_map(|field| (&field.name == name).then_some(field.value.as_ref().unwrap()))
	}

	/// Gets the value of the internal field with the given name from this table.
	///
	/// # Parameters
	/// - `name`: the name of the internal field to get.
	///
	/// # Returns
	/// The value of the internal field with the given name from this table, or `None` if no such field exists.
	pub fn get_internal_field(&self, name: &str) -> Option<&InternalValue> {
		self.internal_fields.get(name)
	}

	/// Gets the value of the internal field with the given name from this table.
	///
	/// # Parameters
	/// - `name`: the name of the internal field to get.
	///
	/// # Returns
	/// The value of the internal field with the given name from this table, or `None` if no such field exists.
	pub fn get_internal_field_mut(&mut self, name: &str) -> Option<&mut InternalValue> {
		self.internal_fields.get_mut(name)
	}

	/// Returns the C name for the type of this object. If this object is not anonymous, this returns the C name of the type of this object. If this object is anonymous,
	/// this returns `table_` followed by the object's unique anonymous ID.
	pub fn c_name(&self) -> String {
		self.anonymous_id.map_or_else(|| self.name.c_name(), |anonymous_id| format!("table_{anonymous_id}"))
	}
}

impl Parse for Object {
	type Output = Self;

	fn parse(tokens: &mut std::collections::VecDeque<Token>, context: &mut Context) -> anyhow::Result<Self::Output> {
		tokens.pop(TokenType::KeywordNew, context)?;
		let type_name = tokens.pop(TokenType::Identifier, context)?;
		tokens.pop(TokenType::LeftBrace, context)?;

		let mut object = Self::new();

		// Name
		object.name = Name(type_name);
		if object.name == Name("Object".to_owned()) {
			object.make_anonymous();
		}

		if !tokens.next_is(TokenType::RightBrace) {
			parse_list!(tokens, context, {
				let tags = TagList::parse(tokens, context)?;
				let field_name = Name(tokens.pop(TokenType::Identifier, context)?);
				tokens.pop(TokenType::Equal, context).map_err(|error| {
					anyhow::anyhow!(
						"{error}\n\twhile parsing the equal sign before parsing the field \"{}\"'s value on an object literal",
						field_name.cabin_name().bold().cyan()
					)
				})?;
				let mut value = Expression::parse(tokens, context).map_err(|error| anyhow::anyhow!("Error parsing value of field \"{}\": {error}", field_name.cabin_name()))?;
				if let Expression::Literal(Literal(LiteralValue::FunctionDeclaration(function_declaration), ..)) = &mut value {
					function_declaration.tags = tags.clone();
					function_declaration.name = Some(field_name.cabin_name());
				}
				object.add_field(DeclarationData {
					name: field_name,
					value: Some(value),
					tags,
					type_annotation: None,
				});
			});
		}

		tokens.pop(TokenType::RightBrace, context)?;

		if object.name == Name("List".to_owned()) {
			object.add_internal_field("data".to_owned(), InternalValue::List(Vec::new()));
		}

		Ok(object)
	}
}

impl CompileTime for Object {
	fn compile_time_evaluate(&self, context: &mut Context, with_side_effects: bool) -> anyhow::Result<Expression> {
		let mut new_object = Self::new();
		for field in &self.fields {
			// Value
			let mut new_value = field.value.as_ref().unwrap().compile_time_evaluate(context, with_side_effects).map_err(|error| {
				anyhow::anyhow!(
					"{error}\n\t{}",
					format!(
						"while performing compile-time evaluation on the {} \"{name}\" on an object",
						"value of the field".bold().white(),
						name = field.name.cabin_name().bold().cyan()
					)
					.dimmed()
				)
			})?;

			if let Expression::Literal(Literal(LiteralValue::VariableReference(variable_reference), ..)) = &new_value {
				let value_of_new_value = if with_side_effects {
					context
						.scope_data
						.get_variable_from_id(variable_reference.name(), context.scope_data.unique_id())
						.ok_or_else(|| {
							context.encountered_compiler_bug = true;
							anyhow::anyhow!(
								"The variable {} was referenced in scope ID {} but that scope doesn't have the variable: {:?}",
								variable_reference.name().cabin_name(),
								variable_reference.scope_id(),
								context.scope_data
							)
						})?
						.value
						.clone()
						.unwrap()
				} else {
					variable_reference.value(context)?
				};

				if let Expression::Literal(new_value_literal) = &value_of_new_value {
					if !new_value_literal.is(&context.unknown_at_compile_time().clone(), context)? {
						new_value = value_of_new_value;
					}
				} else {
					new_value = value_of_new_value;
				};
			}

			// Tags
			let compile_time_annotations = if let Expression::Literal(Literal(LiteralValue::FunctionDeclaration(function_declaration), ..)) = &new_value {
				function_declaration.tags.clone()
			} else {
				field.tags.clone()
			};

			// Add the field
			new_object.add_field(DeclarationData {
				name: field.name.clone(),
				value: Some(new_value),
				tags: compile_time_annotations,
				type_annotation: None,
			});
		}

		// Extra handling for non-anonymous objects
		if !self.is_anonymous() {
			// Get the group that the object's type is
			let Expression::Literal(Literal(LiteralValue::Group(group_declaration), ..)) = context
				.scope_data
				.get_variable(&self.name)
				.ok_or_else(|| {
					anyhow::anyhow!(
						"Attempted to create an object literal of type \"{}\", but no variable with that name was found in this scope\n\n\t{}",
						self.name.cabin_name().bold().cyan(),
						"while evaluating an object literal at compile-time".dimmed(),
					)
				})?
				.value
				.clone()
				.unwrap()
			else {
				anyhow::bail!(
					"Attempted to create an object literal of type {}, and that variable exists in this scope, but it's not a group",
					self.name.cabin_name()
				);
			};

			// Add the default fields from the group
			for field in &group_declaration.fields {
				if let Some(field_value) = &field.value {
					new_object.add_field(DeclarationData {
						name: field.name.clone(),
						type_annotation: None,
						value: Some(field_value.clone()),
						tags: field.tags.clone(),
					});
				}
			}
		}

		new_object.name = self.name.clone();
		new_object.internal_fields = self.internal_fields.clone();
		new_object.anonymous_id = self.anonymous_id;
		new_object.has_been_compile_time_evaluated = true;

		if new_object.is_anonymous()
			&& !context
				.groups
				.iter()
				.any(|group_name| group_name.0 == format!("table_{}", new_object.anonymous_id.as_ref().unwrap()))
		{
			context.groups.push((format!("table_{}", new_object.anonymous_id.as_ref().unwrap()), GroupType::Group));
		}

		Ok(Expression::Literal(Literal::new(LiteralValue::Object(new_object))))
	}
}

impl ParentExpression for Object {
	fn evaluate_children_at_compile_time(&self, context: &mut Context) -> anyhow::Result<Expression> {
		self.compile_time_evaluate(context, true)
	}
}

impl Typed for Object {
	fn get_type(&self, _context: &mut Context) -> anyhow::Result<Literal> {
		Ok(if self.is_anonymous() {
			var_literal!("AnonymousTable", 0)
		} else {
			var_literal!(self.name.cabin_name(), 0)
		})
	}
}

impl TranspileToC for Object {
	fn c_prelude(&self, context: &mut Context) -> anyhow::Result<String> {
		let mut prelude = String::new();

		// Anonymous Tables
		if self.is_anonymous() {
			write!(prelude, "struct {} {{", self.c_name())?;
			for field in &self.fields {
				match field.value.as_ref().unwrap() {
					Expression::Literal(Literal(LiteralValue::FunctionDeclaration(function_declaration), ..)) => prelude.push_str(
						&("\n".to_owned()
							+ &format!(
								"{return_type} (*{name})({parameters});",
								name = field.name.c_name(),
								return_type = {
									let mut raw = function_declaration.return_type.to_c(context)?;
									if &raw != "void" {
										raw += "*";
									}
									raw
								},
								parameters = function_declaration
									.parameters
									.iter()
									.map(|(name, type_annotation)| Ok(format!("{}* {name}", type_annotation.to_c(context)?, name = name.c_name())))
									.collect::<anyhow::Result<Vec<_>>>()?
									.join(", ")
							)
							.lines()
							.map(|line| format!("\t{line}"))
							.collect::<Vec<_>>()
							.join("\n")),
					),

					Expression::Literal(Literal(LiteralValue::Group(group), ..)) => write!(prelude, "\n\tGroup_{}* {};", group.id, field.name.c_name())?,
					Expression::Literal(Literal(LiteralValue::Object(object), ..)) => write!(prelude, "\n\ttable_{}* {};", object.anonymous_id.unwrap(), field.name.c_name())?,

					_ => {
						// let Type::Group(field_type, field_id) = field.value.as_ref().unwrap().get_type(context)?;
						// let group_type = context.scope_data.get_variable_from_id(&field_type, field_id).unwrap().value.as_ref().unwrap().clone();

						// write!(prelude, "\n\t{}* {};", group_type.to_c(context)?, field.name.c_name())?;
						todo!()
					},
				};
			}

			if self.fields.is_empty() {
				prelude.push_str("\n\tchar empty;");
			}

			writeln!(prelude, "\n}};\n")?;
		}

		for field in &self.fields {
			prelude.push_str(&field.value.as_ref().unwrap().c_prelude(context)?);
		}

		Ok(prelude)
	}

	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		let mut fields = Vec::new();

		// Explicit fields
		for field in &self.fields {
			fields.push(
				format!(".{name} = &{value}\n", name = field.name.c_name(), value = field.value.as_ref().unwrap().to_c(context)?)
					.lines()
					.map(|line| format!("\t{line}"))
					.collect::<Vec<_>>()
					.join("\n"),
			);
		}

		if self.name == Name("Number".to_owned()) {
			let Some(InternalValue::Number(internal_value)) = self.get_internal_field("internal_value") else {
				unreachable!();
			};

			fields.push(format!("\t.internal_value = {internal_value}"));
		}

		if self.name == Name("Text".to_owned()) {
			let Some(InternalValue::String(internal_value)) = self.get_internal_field("internal_value") else {
				unreachable!();
			};

			fields.push(format!("\t.internal_value = \"{internal_value}\""));
		}

		if fields.is_empty() {
			fields.push("\t.empty = '0'".to_owned());
		}

		Ok(format!("({}) {{\n{}\n}}", self.c_name(), fields.join(",\n")))
	}
}

impl ToCabin for Object {
	fn to_cabin(&self) -> String {
		let mut table_cabin = self.name.cabin_name() + " {";
		for field in &self.fields {
			if !field.tags.is_empty() {
				write!(table_cabin, "#[{}]", field.tags.iter().map(|tag| tag.to_cabin()).collect::<Vec<_>>().join(", ")).unwrap();
			}
			write!(table_cabin, "\n\t{} = {}", field.name.cabin_name(), field.value.as_ref().unwrap().to_cabin()).unwrap();
		}
		if !self.fields.is_empty() {
			table_cabin.push('\n');
		}
		table_cabin.push('}');
		table_cabin
	}
}

impl ColoredCabin for Object {
	fn to_colored_cabin(&self, context: &mut Context) -> String {
		let mut table_cabin = format!("{} {} {{", "new".style(context.theme().keyword()), self.name.to_colored_cabin(context));
		for field in &self.fields {
			if !field.tags.is_empty() {
				write!(
					table_cabin,
					"    #[{}]",
					field.tags.iter().map(|tag| tag.to_colored_cabin(context)).collect::<Vec<_>>().join(", ")
				)
				.unwrap();
			}
			write!(
				table_cabin,
				"\n    {} = {}",
				field.name.to_colored_cabin(context),
				field.value.as_ref().unwrap().to_colored_cabin(context)
			)
			.unwrap();
		}
		table_cabin.push_str("\n}");
		table_cabin
	}
}
