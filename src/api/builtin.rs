use std::collections::VecDeque;

use colored::Colorize as _;

use crate::{
	api::{
		context::context,
		macros::{number, string},
		traits::TryAs as _,
	},
	comptime::{memory::VirtualPointer, CompileTime},
	debug_start, err,
	lexer::Span,
	mapped_err,
	parser::expressions::{name::Name, object::ObjectConstructor, Expression, Spanned, Typed},
};

use super::scope::ScopeId;

pub struct BuiltinFunction {
	evaluate_at_compile_time: fn(ScopeId, Vec<Expression>, Span) -> anyhow::Result<Expression>,
	to_c: fn(&[String]) -> anyhow::Result<String>,
}

static BUILTINS: phf::Map<&str, BuiltinFunction> = phf::phf_map! {
	"terminal.print" => BuiltinFunction {
		evaluate_at_compile_time: |caller_scope_id, arguments, span| {
			let debug_section = debug_start!("{} built-in function {}.{}", "Calling".green().bold(), "terminal".red(), "print".blue());
			let pointer = VecDeque::from(arguments).pop_front().ok_or_else(|| anyhow::anyhow!("Missing argument to print"))?;
			let returned_object = call_builtin_at_compile_time("Anything.to_string", caller_scope_id, vec![pointer], span)?;
			let string_value = returned_object.try_as_literal()?.try_as::<String>()?.to_owned();

			if context().lines_printed == 0 && !context().config().options().quiet() {
				println!("\n");
				context().lines_printed += 1;
			}

			println!("{string_value}");
			context().lines_printed += string_value.chars().filter(|character| character == &'\n').count() + 1;

			debug_section.finish();
			Ok(Expression::Void(()))
		},
		to_c: |parameter_names| {
			let text_address = context().scope_data.get_variable("Text").unwrap().try_as::<VirtualPointer>().unwrap();
			let anything_address = context().scope_data.get_variable("Anything").unwrap().try_as::<VirtualPointer>().unwrap();
			let object = parameter_names.first().unwrap();
			Ok(unindent::unindent(&format!(
				"
				group_u_Text_{text_address}* return_address;
				(((void (*)(group_u_Anything_{anything_address}*, group_u_Text_{text_address}*))({object}->u_to_string->call))({object}, return_address));
				printf(\"%s\\n\", return_address->internal_value);
				"
			)))
		}
	},
	"terminal.input" => BuiltinFunction {
		evaluate_at_compile_time: |_caller_scope_id, _arguments, span| {
			let mut line = String::new();
			std::io::stdin().read_line(&mut line)?;
			line = line.get(0..line.len() - 1).unwrap().to_owned();
			Ok(Expression::Pointer(*ObjectConstructor::string(&line, span).evaluate_at_compile_time()?.try_as::<VirtualPointer>()?))
		},
		to_c: |parameter_names| {
			let return_address = parameter_names.first().unwrap();
			let text_address = context().scope_data.get_variable("Text").unwrap().try_as::<VirtualPointer>().unwrap();
			Ok(format!("char* buffer = malloc(sizeof(char) * 256);\nfgets(buffer, 256, stdin);\n*{return_address} = (group_u_Text_{text_address}) {{ .internal_value = buffer }};"))
		}
	},
	"Number.plus" => BuiltinFunction {
		evaluate_at_compile_time: |_caller_scope_id, arguments, _span| {
			let first = arguments.first().ok_or_else(|| anyhow::anyhow!("Missing argument to Number.plus"))?;
			let first_number = first.try_as_literal()?.try_as::<f64>()?.to_owned();
			let second = arguments.get(1).ok_or_else(|| anyhow::anyhow!("Missing argument to Number.plus"))?;
			let second_number = second.try_as_literal()?.try_as::<f64>()?;

			Ok(number(first_number + second_number, first.span().to(&second.span())))
		},
		to_c: |parameter_names| {
			let number_address = context().scope_data.get_variable("Number").unwrap().try_as::<VirtualPointer>().unwrap();
			let first = parameter_names.first().unwrap();
			let second = parameter_names.get(1).unwrap();
			let number_type = format!("group_u_Number_{number_address}");
			Ok(format!("*return_address = ({number_type}) {{ .internal_value = {first}->internal_value + {second}->internal_value }};"))
		}
	},
	"Number.minus" => BuiltinFunction {
		evaluate_at_compile_time: |_caller_scope_id, arguments, _span| {
			let first = arguments.first().ok_or_else(|| anyhow::anyhow!("Missing argument to Number.plus"))?;
			let first_number = first.try_as_literal()?.try_as::<f64>()?.to_owned();
			let second = arguments.get(1).ok_or_else(|| anyhow::anyhow!("Missing argument to Number.plus"))?;
			let second_number = second.try_as_literal()?.try_as::<f64>()?;

			Ok(number(first_number - second_number, first.span().to(&second.span())))
		},
		to_c: |_parameter_names| {
			Ok(String::new())
		}
	},
	"Anything.to_string" => BuiltinFunction {
		evaluate_at_compile_time: |_caller_scope_id, arguments, span| {
			let this = arguments
				.first()
				.ok_or_else(|| anyhow::anyhow!("Missing argument to {}", format!("{}.{}()", "Anything".yellow(), "to_string".blue()).bold()))?
				.try_as_literal().map_err(mapped_err! {
					while = format!("Interpreting the first argument to {} as a literal", format!("{}.{}()", "Anything".yellow(), "to_string".blue()).bold()),
				})?;

			let type_name = this.get_internal_field::<Name>("representing_type_name").unwrap_or(this.type_name());
			Ok(string(&match type_name.unmangled_name() {
				"Number" => this.try_as::<f64>()?.to_string(),
				"Text" => this.try_as::<String>()?.to_owned(),
				_ => {
					let mut builder = "{".to_owned();

					for (field_name, field_pointer) in this.fields() {
						builder += &format!("\n\t{} = {},", field_name.unmangled_name(), field_pointer);
					}

					if !this.has_any_fields() {
						builder += "\n";
					}

					builder += "}";

					builder
				}
			}, span))
		},
		to_c: |parameter_names| {
			let object = parameter_names.first().unwrap();
			let return_address = parameter_names.get(1).ok_or_else(|| err! {
				base = format!("Missing first argument to {}", format!("{}.{}()", "Anything".yellow(), "to_string".blue()).bold()),
				while = format!("getting the first argument to {}", format!("{}.{}()", "Anything".yellow(), "to_string".blue()).bold()),
			})?;

			let group_address = context().scope_data.get_variable("Group").unwrap().try_as::<VirtualPointer>().unwrap();
			let text_address = context().scope_data.get_variable("Text").unwrap().try_as::<VirtualPointer>().unwrap();
			let anything_address = context().scope_data.get_variable("Anything").unwrap().try_as::<VirtualPointer>().unwrap();

			Ok(unindent::unindent(&format!(
				r#"
				// Get the type metadata of the value
				group_u_Group_{group_address} type;
				(((void (*)(group_u_Anything_{anything_address}*, group_u_Group_{group_address}*))({object}->u_type->call))({object}, &type));

				// Build the string
				DynamicString result = (DynamicString) {{ .value = type.name, .capacity = 16 }};
				push_to_dynamic_string(&result, " {{");

				// Add fields
				for (int fieldNumber = 0; fieldNumber < type.u_fields->elements.size; fieldNumber++) {{										
					push_to_dynamic_string(&result, (char*) type.u_fields->elements.data[fieldNumber]);
				}}

				// Return the built string
				push_to_dynamic_string(&result, " }}");
				*{return_address} = (group_u_Text_{text_address}) {{ .internal_value = result.value }};
				"#
			)))
		}
	},
	"Anything.type" => BuiltinFunction {
		evaluate_at_compile_time: |_caller_scope_id, arguments, _span| {
			let this = arguments.first().unwrap();
			Ok(Expression::Pointer(this.get_type()?))
		},
		to_c: |_parameter_names| {
			Ok(String::new())
		}
	},
};

pub fn call_builtin_at_compile_time(name: &str, caller_scope_id: ScopeId, arguments: Vec<Expression>, span: Span) -> anyhow::Result<Expression> {
	(BUILTINS
		.get(name)
		.ok_or_else(|| {
			anyhow::anyhow!(
				"Attempted to call the built-in function \"{}\", but no built-in function with that name exists.",
				name.bold().cyan()
			)
		})?
		.evaluate_at_compile_time)(caller_scope_id, arguments, span)
	.map_err(mapped_err! {
		while = format!("calling the built-in function \"{}\" at compile-time", name.bold().cyan()).dimmed(),
	})
}

pub fn transpile_builtin_to_c(name: &str, parameters: &[String]) -> anyhow::Result<String> {
	(BUILTINS
		.get(name)
		.ok_or_else(|| {
			anyhow::anyhow!(
				"Attempted to transpile the built-in function \"{}\" to C, but no built-in function with that name exists.",
				name.bold().cyan()
			)
		})?
		.to_c)(parameters)
}
