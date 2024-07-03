use crate::{
	compile_time::{CompileTime, TranspileToC},
	context::Context,
	formatter::{ColoredCabin, ToCabin},
	lexer::{Token, TokenType},
	parse_list,
	parser::{
		expressions::{
			literals::LiteralValue,
			run::ParentExpression,
			util::{name::Name, tags::TagList, types::Typed},
			Expression,
		},
		Parse, TokenQueue,
	},
	scopes::{DeclarationData, ScopeType},
	var_literal,
};

use colored::Colorize as _;

// Brings the `write!()` and `writeln!()` macros into scope, which allows appending to a string. This is more efficient than using
// `string = format!("{string}...")`, because it avoids an extra allocation. We have a clippy warning turned on for this very
// purpose. We assign this to `_` to indicate clearly that it's just a trait and not used explicitly anywhere outside of bringing its
// methods into scope.
use std::{
	fmt::Write as _,
	sync::atomic::{AtomicUsize, Ordering},
};

use super::Literal;

/// A type declaration. This is equivalent to a struct or interface declaration in other languages.
#[derive(Clone, Debug)]
pub struct GroupDeclaration {
	/// The fields declared on this group as a `Vec` of declarations.
	pub fields: Vec<DeclarationData>,

	/// The compile-time parameters on this group. This is similar to generics in other languages.
	pub compile_time_parameters: Option<Vec<Name>>,

	/// The id of the scope of the *inside* of this group declaration. This is the scope where compile-time parameters passed to this group are stored.
	pub inner_scope_id: usize,

	/// The type of group. Groups, unique lists, and objects are all essentially the same data construct: They're a list of fields, each of which has a name,
	/// possibly a type, and possibly a value. The implementations of these all vary slightly, so we tag `GroupDeclaration` with this `GroupType`, which we can then
	/// match on later to determine specific implementation details.
	pub group_type: GroupType,

	/// The tags on this group. This is stored to check if the group is "creatable" at the time of instantiation.
	pub tags: TagList,

	pub id: usize,
}

static NEXT_UNUSED_GROUP_ID: AtomicUsize = AtomicUsize::new(0);
fn next_unused_group_id() -> usize {
	NEXT_UNUSED_GROUP_ID.fetch_add(1, Ordering::Relaxed)
}

/// The type of a group object. Groups, unique lists, and objects are all essentially the same data construct: They're a list of fields, each of which has a name,
/// possibly a type, and possibly a value. The implementations of these all vary slightly, so we tag `GroupDeclaration` with this `GroupType`, which we can then
/// match on later to determine specific implementation details.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GroupType {
	/// The group type, used for instantiable group declarations.
	Group,
	/// The unique list type, used for unique lists.
	Either,
}

impl Parse for GroupDeclaration {
	type Output = Self;

	fn parse(tokens: &mut std::collections::VecDeque<Token>, context: &mut Context) -> anyhow::Result<Self::Output> {
		tokens.pop(TokenType::KeywordGroup, context)?;

		// Compile-time Parameters
		let compile_time_parameters = tokens
			.next_is(TokenType::LeftAngleBracket)
			.then(|| {
				let mut compile_time_parameters = Vec::new();
				tokens.pop(TokenType::LeftAngleBracket, context)?;
				if !tokens.next_is(TokenType::RightAngleBracket) {
					parse_list!(tokens, context, {
						let name = Name(tokens.pop(TokenType::Identifier, context)?);
						compile_time_parameters.push(name);
					});
				}
				tokens.pop(TokenType::RightAngleBracket, context)?;
				Ok::<Vec<Name>, anyhow::Error>(compile_time_parameters)
			})
			.transpose()?;

		tokens
			.pop(TokenType::LeftBrace, context)
			.map_err(|error| anyhow::anyhow!("{error}\n\twhile attempting to parse the opening brace on a group declaration"))?;
		context.scope_data.enter_new_scope(ScopeType::Group);
		let inner_scope_id = context.scope_data.unique_id();

		// Add compile-time parameters to the scope
		if let Some(generic_parameters) = &compile_time_parameters {
			for parameter in generic_parameters {
				context
					.scope_data
					.declare_new_variable(parameter.clone(), None, Expression::Literal(context.unknown_at_compile_time().clone()), TagList::default())?;
			}
		}

		// Fields
		let mut fields = Vec::new();
		if !tokens.next_is(TokenType::RightBrace) {
			parse_list!(tokens, context, {
				// Tags
				let tags = tokens
					.next_is(TokenType::TagOpening)
					.then(|| TagList::parse(tokens, context).map_err(|error| anyhow::anyhow!("{error}\n\twhile parsing the tags of this field")))
					.transpose()?
					.unwrap_or_default();

				// Field name
				let name = Name(tokens.pop(TokenType::Identifier, context)?);

				// Explicit type tag
				let mut type_annotation = tokens
					.next_is(TokenType::Colon)
					.then(|| {
						tokens.pop(TokenType::Colon, context)?;
						Expression::parse(tokens, context).map_err(|error| anyhow::anyhow!("{error}\n\twhile parsing the type of the field \"{}\"", name.cabin_name()))
					})
					.transpose()?;

				// Value
				let value = tokens
					.next_is(TokenType::Equal)
					.then(|| {
						tokens.pop(TokenType::Equal, context)?;
						let mut value = Expression::parse(tokens, context).map_err(|error| anyhow::anyhow!("{error}\n\twhile parsing value of field \"{}\"", name.cabin_name()))?;

						if let Expression::Literal(Literal(LiteralValue::FunctionDeclaration(function_declaration), ..)) = &mut value {
							function_declaration.name = Some(name.cabin_name());
						}

						// Infer type tag
						if type_annotation.is_none() {
							type_annotation = match &value {
								Expression::Literal(Literal(LiteralValue::FunctionDeclaration(function_declaration), ..)) => {
									Some(Expression::Literal(Literal::new(LiteralValue::FunctionDeclaration(function_declaration.clone()))))
								},
								_ => None,
							};
						}

						Ok::<Expression, anyhow::Error>(value)
					})
					.transpose()?;

				// Type tag couldn't be inferred
				let Some(field_type_annotation) = type_annotation else {
					anyhow::bail!(
						"Unable to infer the type of the field \"{name}\"\n\twhile parsing the field \"{name}\" of a group",
						name = name.cabin_name().bold().cyan()
					);
				};

				// Add the field
				fields.push(DeclarationData {
					name,
					type_annotation: Some(field_type_annotation),
					value,
					tags,
				});
			});
		}

		tokens.pop(TokenType::RightBrace, context)?;
		context.scope_data.exit_scope()?;

		let group = Self {
			fields,
			compile_time_parameters,
			inner_scope_id,
			tags: TagList::default(),
			group_type: GroupType::Group,
			id: next_unused_group_id(),
		};

		if !context.groups.iter().any(|group_name| group_name.0 == format!("Group_{}", group.id)) {
			context.groups.push((format!("Group_{}", group.id), group.group_type.clone()));
		}

		if !context.groups.iter().any(|group_name| group_name.0 == format!("anonymous_group_{}", group.id)) {
			context.groups.push((format!("anonymous_group_{}", group.id), group.group_type.clone()));
		}

		Ok(group)
	}
}

impl Typed for GroupDeclaration {
	fn get_type(&self, _context: &mut Context) -> anyhow::Result<Literal> {
		Ok(var_literal!("Group", 0))
	}
}

impl CompileTime for GroupDeclaration {
	fn compile_time_evaluate(&self, context: &mut Context, with_side_effects: bool) -> anyhow::Result<Expression> {
		if self.group_type == GroupType::Either {
			return Ok(Expression::Literal(Literal::new(LiteralValue::Group(self.clone()))));
		};

		if let Some(generics) = &self.compile_time_parameters {
			context.generics_stack.push(generics.clone());
		}

		let mut fields = self.fields.clone();

		for field in &mut fields {
			// Compile-time evaluate the field's tags
			field.tags = TagList::new(
				field
					.tags
					.iter()
					.map(|tag| tag.compile_time_evaluate(context, with_side_effects))
					.collect::<anyhow::Result<Vec<_>>>()
					.map_err(|error| {
						anyhow::anyhow!(
							"{error}\n\t{}",
							format!(
								"while evaluating the {} \"{}\" at compile-time",
								"tags of the field".bold().white(),
								field.name.cabin_name().cyan().bold()
							)
							.dimmed()
						)
					})?,
			);

			// Compile-time evaluate the fields type tag
			field.type_annotation = Some(field.type_annotation.as_ref().unwrap().compile_time_evaluate(context, with_side_effects).map_err(|error| {
				anyhow::anyhow!(
					"{error}\n\t{}",
					format!(
						"while evaluating the {} \"{}\" at compile-time",
						"type of the field".white().bold(),
						field.name.cabin_name().cyan().bold()
					)
					.dimmed()
				)
			})?);

			// Compile-time evaluate the field's value
			field.value = field
				.value
				.as_ref()
				.map(|value| {
					let mut evaluated_value = value.compile_time_evaluate(context, with_side_effects).map_err(|error| {
						anyhow::anyhow!(
							"{error}\n\twhile evaluating the {} \"{}\" at compile-time",
							"value of the field".bold().white(),
							field.name.cabin_name().bold().cyan()
						)
					});

					// if the field is a function, give it the tags
					if let Ok(Expression::Literal(Literal(LiteralValue::FunctionDeclaration(function_declaration), ..))) = &mut evaluated_value {
						function_declaration.tags = field.tags.clone();
					}

					// Return the evaluated field value
					evaluated_value
				})
				.transpose()
				.map_err(|error| {
					anyhow::anyhow!(
						"{error}\n\twhile evaluating the {} \"{}\" at compile-time",
						"value of the field".bold().white(),
						field.name.cabin_name().cyan().bold()
					)
				})?;
		}

		if self.compile_time_parameters.is_some() {
			context.generics_stack.pop().unwrap();
		}

		Ok(Expression::Literal(Literal::new(LiteralValue::Group(Self {
			fields,
			compile_time_parameters: self.compile_time_parameters.clone(),
			inner_scope_id: self.inner_scope_id,
			group_type: GroupType::Group,
			tags: self.tags.clone(),
			id: self.id,
		}))))
	}
}

impl TranspileToC for GroupDeclaration {
	fn to_c(&self, _context: &mut Context) -> anyhow::Result<String> {
		Ok(format!("group_{}", self.id))
	}

	fn c_prelude(&self, context: &mut Context) -> anyhow::Result<String> {
		let name = context
			.transpiling_group_name
			.clone()
			.map_or_else(|| format!("anonymous_group_{}", self.id), |name| name.c_name());
		let mut prelude = vec![format!("// group {name}", name = Name::from_c(&name).cabin_name())];
		if let Some(compile_time_parameters) = &self.compile_time_parameters {
			context.generics_stack.push(compile_time_parameters.clone());
		}
		for field in &self.fields {
			if let Some(value) = &field.value {
				if let &Expression::Literal(Literal(LiteralValue::FunctionDeclaration(_), ..)) = &value {
					// nothing to see here...
				} else {
					prelude.push(field.value.as_ref().unwrap().c_prelude(context).map_err(|error| {
						anyhow::anyhow!(
							"{error}\n\t{}",
							format!("while generating the C prelude for the field \"{}\" of a group", field.name.cabin_name().bold().cyan()).dimmed()
						)
					})?);
				}
			}
		}
		if self.compile_time_parameters.is_some() {
			context.generics_stack.pop().unwrap();
		}

		if let Some(compile_time_parameters) = &self.compile_time_parameters {
			context.generics_stack.push(compile_time_parameters.clone());
		}

		prelude.push(format!("struct {name} {{"));

		prelude.push("\tBoolean_u* (*equals_u)(void*, void*);".to_owned());

		for field in &self.fields {
			if let Expression::Literal(Literal(LiteralValue::FunctionDeclaration(_), ..)) = field.type_annotation.as_ref().unwrap() {
				context.function_type_name = Some(field.name.clone());
				let function_type_c = field.type_annotation.as_ref().unwrap().to_c(context)?;
				prelude.push(format!("{};", function_type_c.get(0..function_type_c.len() - 1).unwrap()));
				context.function_type_name = None;
			} else {
				prelude.push(format!("\t{}* {};", field.type_annotation.as_ref().unwrap().to_c(context)?, field.name.c_name()));
			}
		}

		match name.as_str() {
			"Text_u" => prelude.push("\tchar* internal_value;".to_owned()),
			"Number_u" => prelude.push("\tfloat internal_value;".to_owned()),
			"List_u" => prelude.push("\tint size;\n\tint capacity;\n\tvoid** data;".to_owned()),

			// TODO: C doesn't allow empty structs. For now, the temporary fix is just to add this useless char field (char is the smallest data type). However,
			// this will cause empty structs to have more size than they otherwise would. What should we do here?
			// ===
			// You can give an array a size of 0, that would make the sizeof(Struct) == 0
			_ => {
				if self.fields.is_empty() {
					prelude.push("\tchar empty[0];".to_owned());
				}
			},
		}

		if self.compile_time_parameters.is_some() {
			context.generics_stack.pop().unwrap();
		}

		prelude.push("};".to_owned());
		context.transpiling_group_name = None;

		Ok(prelude.join("\n"))
	}
}

impl ToCabin for GroupDeclaration {
	fn to_cabin(&self) -> String {
		let mut cabin_code = "group {".to_owned();
		for field in &self.fields {
			write!(cabin_code, "\t{}", field.name.cabin_name()).unwrap();
			write!(cabin_code, ": {}", field.type_annotation.as_ref().unwrap().to_cabin()).unwrap();
			if let Some(value) = &field.value {
				write!(cabin_code, " = {}", value.to_cabin().lines().collect::<Vec<_>>().join("\n\t")).unwrap();
			}
		}
		cabin_code.push('}');
		cabin_code
	}
}

impl ColoredCabin for GroupDeclaration {
	fn to_colored_cabin(&self, context: &mut Context) -> String {
		let mut cabin_code = format!("{} {{", "group".purple());
		for field in &self.fields {
			write!(cabin_code, "\n    {}", field.name.to_colored_cabin(context)).unwrap();
			// write!(cabin_code, ": {}", field.type_annotation.to_colored_cabin()).unwrap();
			if let Some(value) = &field.value {
				write!(cabin_code, " = {},", value.to_colored_cabin(context).lines().collect::<Vec<_>>().join("\n    ")).unwrap();
			}
		}
		cabin_code.push_str("\n}");
		cabin_code
	}
}

impl ParentExpression for GroupDeclaration {
	fn evaluate_children_at_compile_time(&self, _context: &mut Context) -> anyhow::Result<Expression> {
		anyhow::bail!("Attempted to use a \"run\" expression on a group declaration: Run expressions have no meaning here");
	}
}
