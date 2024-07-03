use crate::{
	cli::theme::{Theme, ONE_MIDNIGHT},
	formatter::ColoredCabin,
	parser::{
		expressions::{
			literals::{function_declaration::FunctionDeclaration, group::GroupType, Literal},
			util::name::Name,
			Expression,
		},
		Program,
	},
	scopes::ScopeData,
};

/// Data about the current state of the compiler. This is a single-instance context variable that is passed to all
/// parts of the compiler. This allows different, far apart parts of the program to communicate with one another.
pub struct Context {
	/// The name of the file that the compiler is currently compiling.
	pub file_name: String,
	/// The current scope data. This is used to manage the scope of variables and functions.
	pub scope_data: ScopeData,

	/// Whether or not the parser is currently parsing a type. This is used for `Declarations` and `BinaryExpressions`. When parsing the type of a
	/// declaration, the parser will attempt to parse the `=` as a binary expression token, which will mess up the parsing. `BinaryExpression` won't
	/// check for an `=` when this is true, making the parser function correctly. This is set in `Declaration's` `parse()` method.
	pub is_parsing_type: bool,

	/// The name of the function that is currently being parsed as a type.
	///
	/// TODO: document this more
	pub function_type_name: Option<Name>,

	/// A list of all structs in the program.
	pub structs: Vec<(Name, usize)>,

	/// The current generics being evaluated.
	///
	/// TODO: This is unfinished and needs more work/documentation.
	pub generics_stack: Vec<Vec<Name>>,

	/// Whether or not we are currently evaluating a type at compile-time. This changes what operators are available when parsing binary operations. To be specific, the assignment
	/// (`=`) binary operator is not available when parsing a type, because it causes ambiguity when parsing declarations, i.e.:
	///
	/// ```cabin
	/// let x: y = 4;
	/// ```
	///
	/// Is this a variable of type `y` and value `4`, or is it a variable of type `y = 4`?; The compiler can't tell, so we disallow assignment in types.
	///
	/// TODO: to avoid confusion, I would also like to disallow assignment in non-statement expressions in general.
	pub is_evaluating_type: bool,

	/// The current program that is being compiled. This is used to get the colored version of the program, which is used to pretty-print code snippets
	/// to the console when errors occur. This is initially set to `None`, but is guaranteed to be present after parse-time, i.e., during compile-time
	/// evaluation, transpilation, etc.
	pub program: Option<Program>,

	/// The current "bad" identifier. When the developer references a variable that can't be found during compile-time
	/// analysis, the literal stores the name of that variable here. Then, when printing a colored snippet of the
	/// program, `Name` checks if it is that variable, and if so, displays it in bold underlined red. This is used to
	/// allow communication between `Literal` during `evaluate_at_compile_time`, and `Name`'s method `to_colored_cabin()`.
	pub current_bad_identifier: Option<Name>,

	/// A list of "error notes" to display. These are details about an error that is displayed below the error and stacktrace. Generally, these have code
	/// snippets showing the location of the error and explaining more detail about the error, as well as suggestions when possible.
	pub error_details: Vec<String>,

	/// The theme that the user is using. This is used by various parts of the compiler to pretty-print code snippets that show where errors and warnings are.
	/// This should never really change at any point during compilation, so this is private and only accessible via an immutable reference returned from
	/// `theme()`.
	theme: Theme,

	/// The functions declared in the program. This is used to forward declare the functions in the compiled C code.
	pub function_declarations: Vec<FunctionDeclaration>,

	/// The groups declared in the program. This is used to forward declare the structs in the compiled C code.
	pub groups: Vec<(String, GroupType)>,

	/// The name of the main function. Each function in Cabin gets a unique identifier, so when we find the main function, we need to save what its name is, which includes its
	/// ID. This is `None` until a main function is found.
	pub main_function_name: Option<String>,

	/// Whether the error encountered is an error with the compiler. Whenever "unreachable" code is reached in the compiler, this is set to true before returning
	/// an error from the enclosing function (generally with `anyhow::bail!`). When the compiler exits, a special message will be printed to the user indicating
	/// that the error is a compiler bug and not an issue with their code, and that they should report it to the GitHub issues.
	pub encountered_compiler_bug: bool,

	/// A list of warnings emitted by the compiler. These are all printed at once when compilation finishes. The warnings should already be pretty-printed and colored when added
	/// to this list; There is no special formatting done on these when they're printed.
	pub warnings: Vec<String>,

	/// The names of the current function parameters. Function parameters get lifted into a scope outside of function calls, and they are tracked here as the function is being evaluated
	/// to tell `VariableReference` not to panic if one of them is encountered and not found in the current scope.
	pub parameter_names: Vec<(Name, Literal)>,

	pub transpiling_group_name: Option<Name>,
	pub transpiling_either_name: Option<Name>,

	pub show_c_errors: bool,
}

impl Context {
	/// Creates a new `Context` instance with the given file name and prelude line count.
	///
	/// # Parameters
	/// - `file_name` - The name of the file that the compiler is currently compiling.
	/// - `prelude_line_count` - The number of lines in the prelude.
	///
	/// # Returns
	/// A new `Context` instance.
	#[must_use]
	pub fn new(file_name: String) -> Self {
		Self {
			file_name,
			scope_data: ScopeData::global(),
			is_parsing_type: false,
			function_type_name: None,
			is_evaluating_type: false,
			program: None,
			current_bad_identifier: None,
			theme: ONE_MIDNIGHT,
			main_function_name: None,
			encountered_compiler_bug: false,
			structs: Vec::new(),
			generics_stack: Vec::new(),
			error_details: Vec::new(),
			function_declarations: Vec::new(),
			groups: Vec::new(),
			warnings: Vec::new(),
			parameter_names: Vec::new(),
			transpiling_group_name: None,
			transpiling_either_name: None,
			show_c_errors: false,
		}
	}

	/// Returns the theme that the user is using. This is used by various parts of the compiler to pretty-print code snippets that show where errors and warnings are.
	/// This should never really change at any point during compilation, so the theme field is private and only accessible via an immutable reference returned from
	/// this function.
	///
	/// # Returns
	/// the currently active theme as specified from the user, or the default theme if none was explicitly configured.
	#[must_use]
	pub const fn theme(&self) -> &Theme {
		&self.theme
	}

	/// Returns the unique value of the `UnknownAtCompileTime` object from the Cabin source. This object is the default assigned to function and group
	/// parameters before they are called. Functions and groups use this when evaluating themselves at compile-time to inspect what they can't evaluate.
	///
	/// # Returns
	/// A reference to the unique value representing `UnknownAtCompileTime`.
	#[must_use]
	pub fn unknown_at_compile_time(&self) -> &Literal {
		let Some(Expression::Literal(literal)) = self
			.scope_data
			.get_variable(&Name("Parameter".to_owned()))
			.map(|declaration| declaration.value.as_ref().unwrap())
		else {
			unreachable!("Internal Error: The variable \"Parameter\" is not found in the current scope or is not a literal");
		};

		literal
	}

	/// Adds a "note" to the context. When the program errors, all notes will be printed at the bottom. This is used by
	/// AST nodes to print the part of the program where the error occurred and provide more detail on the error.
	///
	/// # Parameters
	/// - `error_note` - The note to add to the error type.
	pub fn add_error_details(&mut self, error_note: String) {
		self.error_details.push(error_note);
	}

	/// Returns the program as a colored string. This is used by parts of the compiler to pretty-print the part of the program where the error occurred.
	///
	/// # Parameters
	/// The program as a colored string.
	#[must_use]
	pub fn colored_program(&mut self) -> String {
		self.program.clone().unwrap().to_colored_cabin(self)
	}
}

/// An error severity level.
#[derive(Debug)]
pub enum Severity {
	/// An error message. This should be used as a hard error that terminates compilation.
	Error,
}

impl std::fmt::Display for Severity {
	#[allow(clippy::use_debug)]
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{self:?}")
	}
}

#[derive(Debug)]
/// An error that is associated with a specific token. All user errors encountered in source code should be thrown using this error type.
pub struct TokenError {
	/// The token that the error occurred on.
	pub line: usize,
	/// The severity of the error.
	pub severity: Severity,
	/// The error message.
	pub message: String,
	/// The name of the file that the error occurred in.
	pub filename: String,
}

impl std::fmt::Display for TokenError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.message)
	}
}

impl std::error::Error for TokenError {}
