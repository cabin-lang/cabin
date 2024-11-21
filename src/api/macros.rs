use crate::{
	api::context::Context,
	comptime::CompileTime,
	lexer::Position,
	parser::expressions::{object::ObjectConstructor, Expression},
};

#[macro_export]
macro_rules! err {
	(
        base = $base: expr,
        while = $process: expr,
        context = $context: expr,
        $($field_name: ident = $field_value: expr),* $(,)?
    ) => {{
        use colored::Colorize as _;

		#[allow(clippy::needless_update)]
		let error = $crate::api::macros::CabinError {
            base: Some(anyhow::anyhow!($base)),
            process: Some("while ".to_owned() + &$process),
            $(
                $field_name: Some($field_value),
            )*
            .. Default::default()
        };

        if let Some(position) = error.position {
            $context.set_error_position(&position);
        }

        if let Some(details) = error.details {
            $context.set_error_details(&details);
        }

        anyhow::anyhow!("{}\n\t{}", error.base.unwrap(), error.process.unwrap().dimmed())
	}};
}

#[macro_export]
macro_rules! mapped_err {
	(
		$($tokens: tt)*
    ) => {
		|error| {
			$crate::err! {
				base = error,
				$($tokens)*
			}
		}
	};
}

#[macro_export]
macro_rules! compiler_message {
    (
        $($tokens: tt)*
    ) => {{
		// Max line length
		let max_line_length = 100;

		// Format the input tokens, unindent it, and remove all newlines
		let formatted = format!($($tokens)*).replace('\n', " ").trim().to_owned();
        let unindented = regex_macro::regex!("[ \t]+").replace_all(&formatted, " ");

		// Create the result string
		let mut result = String::new();
		let mut current_line_length = 0;

		// Add the result character-by-character
		for character in unindented.chars() {

			// Space when our line is at max length - start a new line
			if character == ' ' && current_line_length >= max_line_length {
				result.push('\n');
				current_line_length = 0;
				continue;
			}

			// Space at beginning of line - get the fudge out we don't need u
			if current_line_length == 0 && character == ' ' {
				continue;
			}

			// Non-space character
			result.push(character);
			current_line_length += 1;
		}

		// Return the result
		result
    }}
}

#[derive(Default)]
pub struct CabinError {
	pub base: Option<anyhow::Error>,
	pub details: Option<String>,
	pub position: Option<Position>,
	pub process: Option<String>,
}

#[macro_export]
macro_rules! bail_err {
    (
        $($tokens: tt)*
    ) => {
        return Err($crate::err!($($tokens)*))
    };
}

#[macro_export]
macro_rules! list {
	(
		$context: expr, $scope_id: expr, $elements: expr
	) => {{
		// Literal
		if $elements.iter().all(|element| element.can_be_literal()) {
			let constructor = ObjectConstructor {
				type_name: $crate::parser::expressions::name::Name::from("List"),
				fields: Vec::new(),
				internal_fields: std::collections::HashMap::from([("elements".to_owned(), $crate::parser::expressions::object::InternalFieldValue::List($elements))]),
				scope_id: $scope_id,
				object_type: $crate::parser::expressions::object::ObjectType::Normal,
			};

			Expression::Pointer(
				$crate::parser::expressions::object::LiteralObject::try_from_object_constructor(constructor, $context)
					.unwrap()
					.store_in_memory($context),
			)
		}
		// Not literal
		else {
			let constructor = ObjectConstructor {
				type_name: $crate::parser::expressions::name::Name::from("List"),
				fields: Vec::new(),
				internal_fields: std::collections::HashMap::from([("elements".to_owned(), $crate::parser::expressions::object::InternalFieldValue::List($elements))]),
				scope_id: $scope_id,
				object_type: $crate::parser::expressions::object::ObjectType::Normal,
			};
			Expression::ObjectConstructor(constructor)
		}
	}};
}

#[macro_export]
macro_rules! literal_list {
	(
		$context: expr, $scope_id: expr, $elements: expr
	) => {{
		let constructor = ObjectConstructor {
			type_name: Name::from("List"),
			fields: Vec::new(),
			internal_fields: HashMap::from([("elements".to_owned(), $crate::parser::expressions::object::InternalFieldValue::List($elements))]),
			scope_id: $scope_id,
			object_type: ObjectType::Normal,
		};

		let literal = LiteralObject::try_from_object_constructor(constructor, $context).unwrap();
		Expression::Pointer($context.virtual_memory.store(literal))
	}};
}

#[macro_export]
macro_rules! new_object {
	(
		$type_name: ident {
			$($field_name: ident = $field_value: expr),* $(,)?
		}, $scope_id: expr
	) => {
		ObjectConstructor {
			type_name: stringify!($type_name).into(),
			fields: vec![$(
				$crate::parser::expressions::object::Field {
					name: stringify!($field_name).into(),
					value: Some($field_value),
					field_type: None
				}
			),*],
			internal_fields: std::collections::HashMap::new(),
			scope_id: $scope_id,
			object_type: $crate::parser::expressions::object::ObjectType::Normal
		}
	};
}

#[macro_export]
macro_rules! literal {
	(
		$context: expr, $($tokens: tt)*
	) => {{
		let constructor = $crate::new_object!($($tokens)*);
		let literal = LiteralObject::try_from_object_constructor(constructor, $context).unwrap();
		let address = $context.virtual_memory.store(literal);
		Expression::Pointer(address)
	}};
}

#[macro_export]
macro_rules! object {
	($($tokens: tt)*) => {
		Expression::ObjectConstructor($crate::new_object!($($tokens)*))
	};
}

#[macro_export]
macro_rules! string_literal {
	(
		$value: expr, $context: expr
	) => {
		Expression::Pointer(ObjectConstructor::from_string($value, $context))
	};
}

pub fn cabin_true(context: &Context) -> Expression {
	context.scope_data.expect_global_variable("true").expect_clone_pointer()
}

pub fn number(number: f64, context: &mut Context) -> Expression {
	ObjectConstructor::from_number(number).evaluate_at_compile_time(context).unwrap()
}
