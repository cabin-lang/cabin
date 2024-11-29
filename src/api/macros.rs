use crate::{
	comptime::CompileTime,
	lexer::Span,
	parser::expressions::{object::ObjectConstructor, Expression},
};

use super::context::context;

/// Returns a `err!()` from the current function, wrapped in a `Result::Err()`.
#[macro_export]
macro_rules! bail_err {
	(
		$($tokens: tt)*
    ) => {
		return Err($crate::err!($($tokens)*))
	};
}

/// If someone could make it so that this doesn't require a trailing comma I will actually serve you for life
#[macro_export]
macro_rules! err {
	(
        base = $base: expr,
        $(while = $process: expr,)?
        $(position = $position: expr,)?
        $(details = $details: expr,)?
    ) => {{
        use colored::Colorize as _;

		#[allow(clippy::needless_update)]
		let error = $crate::api::macros::CabinError {
            base: Some(anyhow::anyhow!($base)),
            $(process: Some("while ".to_owned() + &$process),)?
            $(at: Some($position),)?
            $(details: Some($details),)?
            .. Default::default()
        };

        if let Some(position) = error.at {
            $crate::api::context::context().set_error_position(position);
        }

        if let Some(details) = error.details {
            $crate::api::context::context().set_error_details(&details);
        }

		$crate::api::context::context().set_compiler_error_position($crate::here!());

        anyhow::anyhow!("{}{}", error.base.unwrap(), if let Some(process) = error.process { format!("\n\t{}", process).dimmed() } else { String::new().bold() })
	}}
}

/// Equivalent to `err!`, but returns a closure that takes an error as a parameter and uses that error as the base for
/// the error stack. This is generally used in `map_err()`, i.e.:
///
/// ```rust
/// try_something().map_err(mapped_err! {
/// 	while = "trying to do something",
/// 	context = context,
/// })?;
/// ```
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

pub fn string(value: &str, span: Span) -> Expression {
	let number = ObjectConstructor::string(value, span).evaluate_at_compile_time().unwrap();
	if !number.is_pointer() {
		panic!("Internal error: Number literal isn't a pointer");
	}
	number
}

pub fn cabin_true() -> anyhow::Result<Expression> {
	context().scope_data.get_variable("true").unwrap().expect_clone_pointer()
}

pub fn number(number: f64, span: Span) -> Expression {
	let number = ObjectConstructor::number(number, span).evaluate_at_compile_time().unwrap();
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

/// Parses a comma-separated list of things. This takes a block of code as one of its parameters. The block is run once at the beginning,
/// and then while the next token is a comma, a comma is consumed and the block is run again. This is used for many comma-separated lists
/// in the language like function parameters, function arguments, group fields, group instantiation, etc.
///
/// This will return the last token that was parsed, so that expressions that end in a list can generate their spans.
#[macro_export]
macro_rules! parse_list {
	(
		$tokens: expr, $list_type: expr, $body: block
	) => {{
		use $crate::parser::TokenQueueFunctionality as _;

		$tokens.pop($list_type.opening())?;
		while !$tokens.next_is($list_type.closing()) {
			$body
			if $tokens.next_is($crate::lexer::TokenType::Comma) {
				$tokens.pop($crate::lexer::TokenType::Comma)?;
			} else {
				break;
			}
		}

		$tokens.pop($list_type.closing())?
	}};
}

#[derive(Default)]
pub struct CabinError {
	pub base: Option<anyhow::Error>,
	pub details: Option<String>,
	pub at: Option<Span>,
	pub process: Option<String>,
}

#[macro_export]
macro_rules! debug_log {
	(
		$($tokens: tt)*
	) => {{
		use colored::Colorize as _;
		if $crate::api::context::context().config().options().debug_info() == "some" {
			println!("{}{}", "│\t".repeat($crate::api::context::context().debug_indent()).dimmed(), format!($($tokens)*));
		}
	}};
}

#[macro_export]
macro_rules! debug_start {
	(
		$($tokens: tt)*
	) => {{
		let message = format!($($tokens)*);
		use colored::Colorize as _;
		if $crate::api::context::context().config().options().debug_info() == "some" {
			println!("{}{}", "│\t".repeat($crate::api::context::context().debug_indent()).dimmed(), message);
		}
		let dropper = $crate::api::context::context().start_debug_sequence(&message);
		dropper
	}};
}

#[macro_export]
macro_rules! here {
	() => {
		$crate::api::context::SourceFilePosition::new(std::line!(), std::column!(), std::file!(), $crate::function!())
	};
}

#[macro_export]
macro_rules! warn {
	() => {};
}
