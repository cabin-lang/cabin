use colored::Colorize;
use std::collections::VecDeque;

use crate::{
	compile_time::{CompileTime, CompileTimeStatement, TranspileToC},
	context::Context,
	formatter::{ColoredCabin, ToCabin},
	lexer::{Token, TokenType},
	parser::{
		expressions::{
			literals::{
				group::GroupType, Literal, LiteralValue
			}, run::{ParentExpression, ParentStatement}, util::{tags::TagList, name::Name, types::Typed}, Expression
		},
		statements::Statement,
		Parse, TokenQueue,
	},
};

// Brings the `write!()` and `writeln!()` macros into scope, which allows appending to a string. This is more efficient than using
// `string = format!("{string}...")`, because it avoids an extra allocation. We have a clippy warning turned on for this very
// purpose. We assign this to `_` to indicate clearly that it's just a trait and not used explicitly anywhere outside of bringing its
// methods into scope.
use std::fmt::Write as _;

/// A variable declaration
#[derive(Clone, Debug)]
pub struct Declaration {
	/// The name of the variable being declared
	pub name: Name,
	/// The scope ID of the scope in which the variable was declared
	pub declared_scope_id: usize,
	/// The tags on the variable
	pub tags: TagList,
	/// The explicit type tag of the variable, if one was provided, otherwise `None` if the type is to be inferred.
	pub type_annotation: Option<Expression>,

	/// The value of this declaration as it's *initially declared*. This is intentionally obscurely named because it **should not be used to get the
	/// value of this declaration**. This is only saved so that the formatter has the original value without needing a `Context` when formatting
	/// the declaration.
	pub initial_value: Expression,

	/// The line number that the declaration starts on in the source code. This is used to pretty-print errors that
	/// point to the exact line of the declaration.
	pub line_start: usize,
}

impl Parse for Declaration {
	type Output = Self;

	fn parse(tokens: &mut VecDeque<Token>, context: &mut Context) -> anyhow::Result<Self::Output> {
		// Tags
		let tags = tokens
			.next_is(TokenType::TagOpening)
			.then(|| TagList::parse(tokens, context))
			.transpose()?
			.unwrap_or_default();

		let line_start = tokens.current_line();

		// Name
		tokens.pop(TokenType::KeywordLet, context)?;
		let name = Name(tokens.pop(TokenType::Identifier, context)?);

		// Type
		context.is_parsing_type = true;
		let type_annotation = if tokens.next_is(TokenType::Colon) {
			tokens.pop(TokenType::Colon, context)?;
			Some(Expression::parse(tokens, context)?)
		} else {
			None
		};
		context.is_parsing_type = false;

		// Value
		tokens.pop(TokenType::Equal, context)?;
		let mut value = Expression::parse(tokens, context).map_err(|error| {
			anyhow::anyhow!(
				"{error}\n\twhile parsing the initial declared value of the variable \"{}\"",
				name.cabin_name().bold().cyan()
			)
		})?;

		// Infer type tag, plus other cleanup actions
		match &mut value {
			// Add names to the function declarations on a group
			Expression::Literal(Literal(LiteralValue::Group(group), ..)) => {
				for field in &mut group.fields {
					if let Some(Expression::Literal(Literal(LiteralValue::FunctionDeclaration(function_declaration), ..))) = &mut field.value {
						function_declaration.name = Some(format!("{}_{}", name.cabin_name(), function_declaration.name.as_ref().unwrap()));
					}
				}
			},

			// Add names to function declaration
			Expression::Literal(Literal(LiteralValue::FunctionDeclaration(function_declaration), ..)) => {
				function_declaration.name = Some(name.cabin_name());
			},
			_ => (),
		};

		let cabin_value_node = value.clone();

		if let Expression::Literal(Literal(LiteralValue::FunctionDeclaration(function_declaration), ..)) = &mut value {
			function_declaration.tags = tags.clone();
		}

		// Add the variable into the scope
		context
			.scope_data
			.declare_new_variable(name.clone(), type_annotation.clone(), value, tags.clone())
			.map_err(|error| anyhow::anyhow!("{error}\n\twhile attempting to declare a new variable called \"{}\"", name.cabin_name()))?;

		Ok(Self {
			name,
			declared_scope_id: context.scope_data.unique_id(),
			type_annotation,
			tags,
			initial_value: cabin_value_node,
			line_start,
		})
	}
}

impl CompileTimeStatement for Declaration {
	fn compile_time_evaluate_statement(&self, context: &mut Context, with_side_effects: bool) -> anyhow::Result<Statement> {

		// Evaluate the value of the declared variable
		let mut value = context
			.scope_data
			.get_scope_from_id(self.declared_scope_id)
			.ok_or_else(|| anyhow::anyhow!("Attempted to get the scope that the variable \"{name}\" was declared in at compile-time, but no scope was found with the stored ID.\n\twhile evaluating the initial declared value of the variable \"{name}\"", name = self.name.cabin_name()))?
			.get_variable_direct(&self.name)
			.cloned()
			.ok_or_else(|| {
				anyhow::anyhow!(
					"Attempted to evaluate the initially declared value of the variable \"{name}\" at compile-time, but the no declaration for the variable was found in its declared scope\n\twhile evaluating the declaration for the variable \"{name}\" at compile-time",
					name = self.name.cabin_name().bold().cyan()
				)
			})?
			.value
			.as_ref()
			.unwrap()
			.compile_time_evaluate(context, with_side_effects)
			.map_err(|error| {
				anyhow::anyhow!(
					"{error}\n\t{}", format!("while evaluating the initial declared {} \"{}\" at compile-time",
					"value of the variable".bold().white(),
					self.name.cabin_name().cyan().bold()).dimmed()
				)
			})?;

		// Evaluate tags
		let tags = {
			let tags = if let Expression::Literal(Literal(LiteralValue::FunctionDeclaration(function_declaration), ..)) = &value {
				function_declaration.tags.clone()
			} else {
				self.tags.clone()
			};

			// Groups need special tag handling
			if let Expression::Literal(Literal(LiteralValue::Group(group), ..)) = &mut value {
				group.tags = tags.clone();
			}

			tags
		};

		// Save the initial value
		let cabin_value_node = value.clone();

		// Add to context structs
		if let Expression::Literal(Literal(LiteralValue::Group(_), ..)) = &value {
			context.structs.push((self.name.clone(), context.scope_data.unique_id()));
		}

		// Explicit type tag
		let type_annotation = if let Some(type_annotation) = &self.type_annotation {
			context.is_evaluating_type = true;

			let Expression::Literal(Literal(LiteralValue::VariableReference(variable_reference), ..)) = 
				type_annotation
				.compile_time_evaluate(context, with_side_effects)
				.map_err(|error| anyhow::anyhow!("{error}\n\t{}", format!("while evaluating the explicit type tag of the variable \"{}\" at compile-time", self.name.cabin_name())))? 

			else {
				let name = self.name.to_colored_cabin(context);
				let initial_value = self.initial_value.to_colored_cabin(context);
				context.add_error_details(format!(
					"Error occurred while evaluating the type for this declaration at compile-time:\n\n{}\n\n{}\n{}\n\nThis variable \"{}\" is declared with the expression \"{}\" as its type, but that expression can't be fully evaluated at compile-time.\nWhile Cabin allows arbitrary expressions as types, it is still a statically typed languages, meaning all types must be known at compile-time.",
					format!("{}:", context.file_name).bold().white(),

					// Print an arrow and message where the error is 
					format!("{}{}  the error is with this type", 
						" ".repeat(format!("    {}    let {}: ", self.line_start, self.name.cabin_name()).len()), 
						"v".repeat(type_annotation.to_cabin().len())
					).truecolor(100, 100, 100),

					// Print the code
					format!("{} {}: {} = {};", 
						"let".purple(), 
						name,
						type_annotation.to_cabin().red().bold().underline(),
						initial_value
					)
						.lines()
						.enumerate()
						.map(|(index, line)| format!("    {line_number}    {line}", line_number = if index == 0 {
							(self.line_start + index).to_string().red().bold()
						} else { 
							(self.line_start + index).to_string().truecolor(100, 100, 100) 
						}))
						.collect::<Vec<_>>()
						.join("\n"),

					self.name.cabin_name().bold().cyan(),
					self.type_annotation.as_ref().unwrap().to_cabin().bold().cyan()
				));

				anyhow::bail!(
					"The explicit type tag provided to the variable \"{name}\" at the time of declaration cannot be resolved at compile-time\n\n{}", 
					format!("\twhile evaluating the type of the variable \"{name}\" at compile-time\n\twhile evaluating the declaration for the variable \"{name}\" at compile-time", name = self.name.cabin_name().bold().cyan()).dimmed(),
					name = self.name.cabin_name().bold().cyan()
				);
			};
			context.is_evaluating_type = false;

			context.scope_data.get_variable_from_id(variable_reference.name(), variable_reference.scope_id()).ok_or_else(|| anyhow::anyhow!("Cannot find variable"))?.value.clone().unwrap()

		}

		// Infer type tag
		else {
			Expression::Literal(value
				.get_type(context)
				.map_err(|error| {
					anyhow::anyhow!(
						"{error}\n\t{}", format!("while inferring the type of the variable \"{}\" at its declaration based on its initial declared value", self.name.cabin_name().cyan().bold()).dimmed()
					)
				})?)
		};

		// Reassign the variable
		context.scope_data.reassign_variable_from_id(&self.name, value, self.declared_scope_id).map_err(|error| {
			anyhow::anyhow!(
				"{error}\n\twhile attempting to reassign the value of the variable \"{}\" after evaluating it at compile-time",
				self.name.cabin_name().bold().cyan()
			)
		})?;

		// Add groups to the context as structs
		if let Expression::Literal(Literal(LiteralValue::Group(group), ..)) = &cabin_value_node {
			if !context.groups.iter().any(|group_name| group_name.0 == self.name.c_name()) {
				context.groups.push((self.name.c_name(), group.group_type.clone()));
			}
		}

		// Add either's to the context as enums
		if let Expression::Literal(Literal(LiteralValue::Either(_either), ..)) = &cabin_value_node {
			if !context.groups.iter().any(|group_name| group_name.0 == self.name.c_name()) {
				context.groups.push((self.name.c_name(), GroupType::Either));
			}
		}

		if self.name == Name("main".to_owned()) {
			let Expression::Literal(Literal(LiteralValue::FunctionDeclaration(function_declaration), ..)) = &cabin_value_node else {
				anyhow::bail!("Main variable is not a function");
			};
			context.main_function_name = Some(format!("{}_{}", function_declaration.name.as_ref().unwrap(), function_declaration.id));
		}

		// Return the evaluated declaration
		Ok(Statement::Declaration(Self {
			name: self.name.clone(),
			declared_scope_id: self.declared_scope_id,
			tags,
			type_annotation: Some(type_annotation),
			initial_value: cabin_value_node,
			line_start: self.line_start,
		}))
	}
}

impl TranspileToC for Declaration {
	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		if self.name.cabin_name() == "Void" {
			return Ok(String::new());
		}

		let value = if self.name.cabin_name().starts_with("parameter_") {
			self.initial_value.clone()
		} else {
			context
				.scope_data
				.get_scope_from_id(self.declared_scope_id)
				.ok_or_else(|| anyhow::anyhow!("Expected scope to exist for declaration"))?
				.get_variable_direct(&self.name)
				.cloned()
				.ok_or_else(|| anyhow::anyhow!("Variable {} not found", self.name.cabin_name()))?
				.value
				.unwrap()
		};

		Ok(match &value {
			Expression::Literal(Literal(LiteralValue::Object(object), ..)) => format!("{} {} = {};\n\n", object.c_name(), self.name.c_name(), value.to_c(context)?),
			Expression::Literal(Literal(LiteralValue::Either(either), ..)) => {
				let c = format!("enum {} {}", self.name.c_name(), either.to_c(context)?);
				format!("{c};\n\n")
			},
			Expression::Literal(Literal(LiteralValue::FunctionDeclaration(function_declaration), ..)) => {
				format!(
					"{return_type} (*{name})({parameters}) = {value};",
					return_type = {
						let mut raw = function_declaration.return_type.to_c(context)?;
						if &raw != "void" {
							raw += "*";
						};
						raw
					},
					name = self.name.c_name(),
					parameters = function_declaration
						.parameters
						.iter()
						.map(|parameter| Ok(format!("{}*", parameter.1.to_c(context)?)))
						.collect::<anyhow::Result<Vec<_>>>()?
						.join(", "),
					value = value.to_c(context)?
				)
			},
			_ => format!(
				"{} {} = {};",
				self.type_annotation
					.as_ref()
					.ok_or_else(|| {
						context.encountered_compiler_bug = true;
						anyhow::anyhow!(
							"Error: The variable \"{}\" has no type tag, even after type inference.\n\n\t{}", 
							self.name.cabin_name().bold().cyan(), 
							format!("while generating the C prelude for the variable \"{}\"", self.name.cabin_name().bold().cyan()).dimmed()
						)
					})?
					.to_c(context)?,
				self.name.c_name(),
				value.to_c(context).map_err(|error| anyhow::anyhow!(
					"{error}\n\twhile converting the {} of the variable \"{}\" to C code",
					"initial declared value".bold().white(),
					self.name.cabin_name().cyan().bold()
				))?
			),
		})
	}

	fn c_prelude(&self, context: &mut Context) -> anyhow::Result<String> {
		let value = context
			.scope_data
			.get_scope_from_id(self.declared_scope_id)
			.ok_or_else(|| anyhow::anyhow!("Expected scope to exist for declaration"))?
			.get_variable_direct(&self.name)
			.cloned()
			.ok_or_else(|| anyhow::anyhow!("Variable \"{}\" not found\n\t{}", self.name.cabin_name().bold().cyan(), format!("while generating the C prelude code for the declaration of the variable \"{}\"", self.name.cabin_name().bold().cyan()).dimmed()))?
			.value
			.unwrap();

		if let Expression::Literal(Literal(LiteralValue::Group(_), ..))  = &value {
			context.transpiling_group_name = Some(self.name.clone());
		}

		value.c_prelude(context).map_err(|error| {
			anyhow::anyhow!(
				"{error}\n\t{}", format!("while generating the C prelude of the initial declared value of the variable \"{}\"",
				self.name.cabin_name().bold().cyan()).dimmed()
			)
		})
	}
}

impl ParentStatement for Declaration {
	fn evaluate_statement_children_at_compile_time(&self,context: &mut Context) -> anyhow::Result<Statement> {
		// Evaluate the value of the declared variable
		let value = context
			.scope_data
			.get_scope_from_id(self.declared_scope_id)
			.ok_or_else(|| anyhow::anyhow!("Attempted to get the scope that the variable \"{name}\" was declared in at compile-time, but no scope was found with the stored ID.\n\twhile evaluating the initial declared value of the variable \"{name}\"", name = self.name.cabin_name()))?
			.get_variable_direct(&self.name)
			.cloned()
			.ok_or_else(|| {
				anyhow::anyhow!(
					"Attempted to evaluate the initially declared value of the variable \"{name}\" at compile-time, but the no declaration for the variable was found in its declared scope\n\twhile evaluating the declaration for the variable \"{name}\" at compile-time",
					name = self.name.cabin_name().bold().cyan()
				)
			})?
			.value
			.as_ref()
			.unwrap()
			.evaluate_children_at_compile_time(context)
			.map_err(|error| {
				anyhow::anyhow!(
					"{error}\n\t{}", format!("while evaluating the initial declared {} \"{}\" at compile-time",
					"value of the variable".bold().white(),
					self.name.cabin_name().cyan().bold()).dimmed()
				)
			})?;

		// Save the initial value
		let cabin_value_node = value.clone();

		// Evaluate tags
		let tags = if let Expression::Literal(Literal(LiteralValue::FunctionDeclaration(function_declaration), ..)) = &value {
			function_declaration.tags.clone()
		} else {
			self.tags.clone()
		};

		// Add to context structs
		if let Expression::Literal(Literal(LiteralValue::Group(_), ..)) = &value {
			context.structs.push((self.name.clone(), context.scope_data.unique_id()));
		}

		// Explicit type annotation
		let type_annotation = if let Some(type_annotation) = &self.type_annotation {
			context.is_evaluating_type = true;

			let Expression::Literal(Literal(LiteralValue::VariableReference(variable_reference), ..)) = 
				type_annotation
				.compile_time_evaluate(context, true)
				.map_err(|error| anyhow::anyhow!("{error}\n\t{}", format!("while evaluating the explicit type tag of the variable \"{}\" at compile-time", self.name.cabin_name())))? 

			else {
				let name = self.name.to_colored_cabin(context);
				let initial_value = self.initial_value.to_colored_cabin(context);
				context.add_error_details(format!(
					"Error occurred while evaluating the type for this declaration at compile-time:\n\n{}\n\n{}\n{}\n\nThis variable \"{}\" is declared with the expression \"{}\" as its type, but that expression can't be fully evaluated at compile-time.\nWhile Cabin allows arbitrary expressions as types, it is still a statically typed languages, meaning all types must be known at compile-time.",
					format!("{}:", context.file_name).bold().white(),

					// Print an arrow and message where the error is 
					format!("{}{}  the error is with this type", 
						" ".repeat(format!("    {}    let {}: ", self.line_start, self.name.cabin_name()).len()), 
						"v".repeat(type_annotation.to_cabin().len())
					).truecolor(100, 100, 100),

					// Print the code
					format!("{} {}: {} = {};", 
						"let".purple(), 
						name,
						type_annotation.to_cabin().red().bold().underline(),
						initial_value
					)
						.lines()
						.enumerate()
						.map(|(index, line)| format!("    {line_number}    {line}", line_number = if index == 0 {
							(self.line_start + index).to_string().red().bold()
						} else { 
							(self.line_start + index).to_string().truecolor(100, 100, 100) 
						}))
						.collect::<Vec<_>>()
						.join("\n"),

					self.name.cabin_name().bold().cyan(),
					self.type_annotation.as_ref().unwrap().to_cabin().bold().cyan()
				));

				anyhow::bail!(
					"The explicit type tag provided to the variable \"{name}\" at the time of declaration cannot be resolved at compile-time\n\n{}", 
					format!("\twhile evaluating the type of the variable \"{name}\" at compile-time\n\twhile evaluating the declaration for the variable \"{name}\" at compile-time", name = self.name.cabin_name().bold().cyan()).dimmed(),
					name = self.name.cabin_name().bold().cyan()
				);
			};
			context.is_evaluating_type = false;

			context.scope_data.get_variable_from_id(variable_reference.name(), variable_reference.scope_id()).ok_or_else(|| anyhow::anyhow!("Error getting variable value in declaration"))?.value.clone().unwrap()

		}

		// Infer type tag
		else {
			Expression::Literal(value
				.get_type(context)
				.map_err(|error| {
					anyhow::anyhow!(
						"{error}\n\twhile inferring the type of the variable \"{}\" at its declaration based on its initial declared value",
						self.name.cabin_name().cyan().bold()
					)
				})?)
		};

		// Reassign the variable
		context.scope_data.reassign_variable_from_id(&self.name, value, self.declared_scope_id).map_err(|error| {
			anyhow::anyhow!(
				"{error}\n\twhile attempting to reassign the value of the variable \"{}\" after evaluating it at compile-time",
				self.name.cabin_name().bold().cyan()
			)
		})?;

		if let Expression::Literal(Literal(LiteralValue::Group(group), ..)) = &cabin_value_node {
			if !context.groups.iter().any(|group_name| group_name.0 == self.name.c_name()) {
				context.groups.push((self.name.c_name(), group.group_type.clone()));
			}
		}

		if self.name == Name("main".to_owned()) {
			let Expression::Literal(Literal(LiteralValue::FunctionDeclaration(function_declaration), ..)) = &cabin_value_node else {
				anyhow::bail!("Main variable is not a function");
			};
			context.main_function_name = Some(format!("{}_{}", function_declaration.name.as_ref().unwrap(), function_declaration.id));
		}

		// Return the evaluated declaration
		Ok(Statement::Declaration(Self {
			name: self.name.clone(),
			declared_scope_id: self.declared_scope_id,
			tags,
			type_annotation: Some(type_annotation),
			initial_value: cabin_value_node,
			line_start: self.line_start,
		}))
	}
}

impl ToCabin for Declaration {
	fn to_cabin(&self) -> String {
		let mut cabin_code = format!("let {}", self.name.cabin_name());
		if let Some(type_annotation) = &self.type_annotation {
			write!(cabin_code, ": {}", type_annotation.to_cabin()).unwrap();
		}
		write!(cabin_code, " = {};", self.initial_value.to_cabin()).unwrap();
		cabin_code
	}
}

impl ColoredCabin for Declaration {
	fn to_colored_cabin(&self, context: &mut Context) -> String {
		let mut cabin_code = format!("{} {}", "let".purple(), self.name.to_colored_cabin(context));
		if let Some(type_annotation) = &self.type_annotation {
			write!(cabin_code, ": {}", type_annotation.to_colored_cabin(context)).unwrap();
		}
		write!(cabin_code, " = {};", self.initial_value.to_colored_cabin(context)).unwrap();
		cabin_code
	}
}
