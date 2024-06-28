use crate::{cli::theme::Styled, context::Context, formatter::ColoredCabin};

use colored::Colorize as _;

/// An identifier name in the language defined by the user. This is used when the user names a variable or
/// function, etc. The incentive to use this over a regular String is that these get converted into different
/// identifiers when transpiled to C to avoid name clashing. For example, when writing anonymous tables, Cabin
/// automatically inserts typedef structs called `table_0`, `table_1`. etc., so If a user were to name their variable
/// `table_0` it would cause a name clash. Thus, all identifiers parsed should be represented with this form.
///
/// The string field stored in this is private, but it represents the original name as specified by the Cabin
/// developer before transforming it into the C version. To avoid confusion, this is private, and you can use the
/// `c_name` and `cabin_name` functions to specifically get the version you want. Generally, you'd want the original
/// name for reporting error messages, such as "Variable {name} not found", and you'd want the C name when transpiling
/// the source code into C.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Name(pub String);

impl Name {
	/// Returns the name of this `Name` as string **after converting it into a unique C identifier**. This should
	/// be used when transpiling Cabin code into C. **Do not use this when reporting information to the user about a
	/// variable; use `cabin_name()` instead**.
	///
	/// This is exactly equivalent to calling `to_c(&mut context)` on the name, except that `to_c` will wrap it in an
	/// `Ok()`.
	pub fn c_name(&self) -> String {
		match self.0.as_str() {
			"Void" => "void".to_owned(),
			"unique" => "int".to_owned(),
			_ => format!("{}_u", self.0),
		}
	}

	/// Returns the name of this `Name` as a string **as originally specified in the Cabin source code.** This should be
	/// used for things like communicating to the user, such as error messages that need to display information about a
	/// variable. **Do not use this when transpiling to C; Use `c_name()` or `to_c(context)` instead**.
	pub fn cabin_name(&self) -> String {
		self.0.clone()
	}

	/// Creates a name from its C representation. The default name constructor creates a name from it's Cabin name, so this allows going the other direction and converting
	/// a C name into a Cabin name.
	///
	/// This function guarantees that:
	///
	/// ```rust
	/// let name = Name::from_c(/* any name */);
	/// assert_eq!(name, Name(name.cabin_name()));
	/// ```
	///
	/// # Parameters
	/// - `c` - The string representation of the C version of the name.
	pub fn from_c(c: &str) -> Self {
		Self(c.get(0..c.len() - 2).unwrap().to_owned())
	}
}

impl ColoredCabin for Name {
	fn to_colored_cabin(&self, context: &mut Context) -> String {
		if let Some(bad_identifier) = &context.current_bad_identifier {
			if bad_identifier == self {
				context.current_bad_identifier = None;
				return format!("{}", self.cabin_name().red().bold().underline());
			}
		}

		if self.cabin_name().starts_with(|character: char| character.is_uppercase()) {
			format!("{}", self.cabin_name().style(context.theme().type_name()))
		} else {
			format!("{}", self.cabin_name().style(context.theme().variable_name()))
		}
	}
}
