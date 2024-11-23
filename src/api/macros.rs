use crate::{
	api::context::Context,
	comptime::CompileTime,
	lexer::Span,
	parser::expressions::{object::ObjectConstructor, Expression},
};

#[macro_export]
macro_rules! bail_err {
	(
		$($tokens: tt)*
    ) => {
		return Err($crate::err!($($tokens)*))
	};
}

#[macro_export]
macro_rules! err {
	(
        base = $base: expr,
        $(while = $process: expr,)?
        context = $context: expr,
        $($field_name: ident = $field_value: expr),* $(,)?
    ) => {{
        use colored::Colorize as _;

		#[allow(clippy::needless_update)]
		let error = $crate::api::macros::CabinError {
            base: Some(anyhow::anyhow!($base)),
            $(process: Some("while ".to_owned() + &$process),)?
            $($field_name: Some($field_value),)*
            .. Default::default()
        };

        if let Some(position) = error.at {
            $context.set_error_position(&position);
        }

        if let Some(details) = error.details {
            $context.set_error_details(&details);
        }

		$context.set_compiler_error_position($crate::here!());

        anyhow::anyhow!("{}{}", error.base.unwrap(), if let Some(process) = error.process { format!("\n\t{}", process).dimmed() } else { String::new().bold() })
	}}
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

/// Returns the fully qualified path to the current function, similar to how `file!()` from `std` works, but for function names.
///
/// This is used by the compiler to log stack traces for printing developer information upon errors.
///
/// modified from https://stackoverflow.com/a/40234666
#[macro_export]
macro_rules! function {
	() => {{
		fn f() {}
		fn type_name_of<T>(_: T) -> &'static str {
			std::any::type_name::<T>()
		}
		let name = type_name_of(f);
		let stripped = name.strip_suffix("::f").unwrap();
		let simplified = regex_macro::regex!("^<([^>< ]+) as ([^>< ]+)>(.*)$").replace(stripped, "${1}${3}").to_string();
		simplified.strip_suffix("::{{closure}}").unwrap_or(&simplified).to_owned()
	}};
}

pub fn string(value: &str, span: Span, context: &mut Context) -> Expression {
	let number = ObjectConstructor::from_string(value, span).evaluate_at_compile_time(context).unwrap();
	if !number.is_pointer() {
		panic!("Internal error: Number literal isn't a pointer");
	}
	number
}

pub fn cabin_true(context: &Context) -> anyhow::Result<Expression> {
	context.scope_data.expect_global_variable("true").expect_clone_pointer(context)
}

pub fn number(number: f64, span: Span, context: &mut Context) -> Expression {
	let number = ObjectConstructor::from_number(number, span).evaluate_at_compile_time(context).unwrap();
	if !number.is_pointer() {
		panic!("Internal error: Number literal isn't a pointer");
	}
	number
}

/// Returns the second value provided wrapped in `Some()` if the first value is true; Otherwise, returns `None`.
///
/// This is equivalent to `boolean::then`, but doesn't create a closure, meaning return statements and the question mark operator
/// can be used in reference to the surrounding function.
#[macro_export]
macro_rules! if_then_some {
	(
		$value: expr, $body: expr
	) => {
		if $value {
			Some($body)
		} else {
			None
		}
	};
}

/// Returns the second value provided if the first provided value is `true`, otherwise, returns `Default::default()`.
#[macro_export]
macro_rules! if_then_else_default {
	(
		$value: expr, $body: expr
	) => {
		if $value {
			$body
		} else {
			Default::default()
		}
	};
}

#[derive(Default)]
pub struct CabinError {
	pub base: Option<anyhow::Error>,
	pub details: Option<String>,
	pub at: Option<Span>,
	pub process: Option<String>,
}
