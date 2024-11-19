use std::collections::VecDeque;

// Bring the `Casing` trait into scope, which allows us to use `.to_case()` on strings. This is used to convert `TokenType` enum variant names into Title Case,
// which is useful for outputting human-readable errors. We assign this to `as _` to indicate clearly that The trait is not referenced directly and only used
// to bring its methods into scope to be called.
use convert_case::Casing as _;
// Bring the `IntoEnumIterator` trait into scope, which allows us to use `::iter()` on enums that `#[derive(strum_macros::EnumIter)]`. Specifically here, this is used
// on `TokenType` to iterate over the values of the enum to test which one matches a specific string of Cabin code. We assign this to `as _` to indicate clearly that
// The trait is not referenced directly and only used to bring its methods into scope to be called.
use strum::IntoEnumIterator as _;

/// A type of token in Cabin source code. The first step in Cabin compilation is tokenization, which is the process of splitting a raw String of source code into
/// "tokens" which each have a "type" representing the kind of token it is, and a "value" representing the string of source code that is associated with it. This
/// enum defines the different "types" of values. Tokens themselves are stored in a separate `Token` struct, which has a `token_type: TokenType` field. This *is*
/// against general Rust convention, which recommends instead making `value: String` a subtype of the `TokenType` enum. However, for this specific implementation and
/// use case, we want to be able to easily iterate over all types of tokens, which means we want each type to be a "concrete object" instead of an instantiable
/// function or type. Thus, we instead make `TokenType` a field on the `Token` struct.
///
/// # Conventions
///
/// These token types are named by what the token itself appears as, not the usage in the language. For example, the "`.`" token is not called "access" or something,
/// it is just called `Dot`. The names of the tokens should be written parser-agnostic, meaning they should have no "knowledge" of the actual use cases of the
/// token in the language. This helps make parser changes easier, as we can repurpose token types without having to rename them and without causing confusion
/// or ambiguity in what they refer to.
#[derive(strum_macros::EnumIter, PartialEq, Eq, Debug, Clone)]
pub enum TokenType {
	/// The "tag opening" token type. This marks the start of a list of tags on a variable declaration. Please note that this only notes the *start* of such
	/// a list, not the entire list. To be specific, this *only* matches the character sequence `#[`. All tokens after that sequenced are tokenized as normal, including
	/// the ending right bracket that tag lists conclude with. A returned token with this type will always have the value `#[`.
	TagOpening,

	/// The "asterisk" token type. This is used for parsing arithmetic multiplication expressions. Any token tokenized of this type will always have a single-character value,
	/// which is the "asterisk" character (*).
	Asterisk,

	/// The "caret" token type. This is used for parsing arithmetic exponential expressions. Any token tokenized of this type will always have a single-character value,
	/// which is the "caret" character (^).
	Caret,

	/// The "colon" token type. This is used for parsing explicit type tags on variable declarations. While Cabin does perform type inference and often allows type
	/// tags to be omitted, there are occasionally times where it's preferred or even necessary to insert a specific tag on a variable. Any token tokenized
	/// of this type will always have a single-character value, which is the "colon" character (:).
	Colon,

	/// The "comma" token type. This is used for things like function parameter separation, group field separation, etc. Any token tokenized of this type will always
	/// have a single-character value, which is the "comma" character (,).
	Comma,

	/// The double equals token type. This is used for comparisons in if statements, similar to other languages. Any token tokenized of this type will always
	/// have the value "=="
	DoubleEquals,

	/// The "dot" token type. This is used for things like function parameter separation, group field separation, etc. Any token tokenized of this type will always
	/// have a single-character value, which is the "comma" character (,).
	Dot,

	/// The "equal" token type. This is used for variable and field assignment. Any token tokenized of this type will always have a single-character value, which is
	/// the "equal" character (=).
	Equal,

	/// The "line comment" token type. This is used for comments the programmer wants to make about the code that doesn't actually affect the code at runtime.
	/// This represents the type of comment that continues until the end of the line. Currently, this is a double slash (//), but in the future will likely be
	/// changed to something more intuitive. When tokenizes, tokens of this type will return the entire comment, including the leading two slashes, but not the
	/// trailing newline or carriage return.
	///
	/// NOTE: This *must* be checked ***before*** the `ForwardSlash` token type. Failure to do so will start parsing a comment as two separate forward slash
	/// tokens, and then attempt to parse the comment as code. These token types are iterated with `strum::IntoEnumIterator`, which iterates over this enum
	/// in order. This means that this enum variant declaration *must* be placed *before* the `ForwardSlash` token type. Please be careful moving this variant
	/// or that one around!
	///
	/// TODO: Change line comments! Double slash (//) isn't the most intuitive. What can we do here? Another common one is the octothorpe (#), but this is already
	/// used for tags. Any other ideas? One is A left arrow "<-" or "<=", indicating that it points to the thing it comments on. However, this can cause ambiguity
	/// when parsing comparative expressions like `if x<-4`, which would become whitespace-sensitive (a similar issue arises with the fat arrow, i.e., `if x<=4`). What
	/// should we use here? This is a design discussion that should happen sooner rather than later.
	LineComment,

	/// The forward slash token. This is used for arithmetic division expressions. Any token tokenized of this type will always have a single-character value, which is
	/// the "forward slash" character (/).
	ForwardSlash,

	/// The `otherwise` keyword token type. This is used similar to the "else" keyword in other languages: it runs a block of code if an `if` condition is false.
	///
	/// The design decision behind making this `otherwise` instead of `else` is that it reads more like natural English. It is less intuitive for experienced
	/// programmers, which may expect an `else` keyword to exist, but after learning it once it shouldn't be an issue.
	///
	/// Like all keywords, this enum variant declaration *must* come before `Identifier`. If it doesn't, then `otherwise` will be tokenized incorrectly as an
	/// identifier, which will cause issues when parsing. Please be careful when moving around this keyword or the `Identifier` token type!
	KeywordOtherwise,

	/// The `if` keyword token type. This is used similar to how it is in other languages: It runs a block of code if some condition is true.
	///
	/// Unlike many other keywords in Cabin, this is the same as it is in almost all other languages. Generally we swap out common keywords with things
	/// we find more intuitive or "common English" sounding, but `if` already is about as colloquial as it gets.
	///
	/// Like all keywords, this enum variant declaration *must* come before `Identifier`. If it doesn't, then `if` will be tokenized incorrectly as an
	/// identifier, which will cause issues when parsing. Please be careful when moving around this keyword or the `Identifier` token type!
	KeywordIf,

	/// The `action` keyword token type. This is used to declare functions.
	///
	/// A token created with this type will always have the value "function".
	///
	/// Like all keywords, this enum variant declaration *must* come before `Identifier`. If it doesn't, then `action` will be tokenized incorrectly as
	/// identifiers, which will cause issues when parsing. Please be careful when moving around this keyword or the `Identifier` token type!
	KeywordAction,

	/// The new keyword. This is used to instantiate a table.
	///
	/// A token created with this type will always have the value "new".
	///
	/// Like all keywords, this enum variant declaration *must* come before `Identifier`. If it doesn't, then `new` will be tokenized incorrectly as
	/// identifiers, which will cause issues when parsing. Please be careful when moving around this keyword or the `Identifier` token type!
	KeywordNew,

	/// The `group` keyword token type, which is used to declare a type of group of variables in the language, analogous to a `struct` in other languages.
	///
	/// This was named `group` instead of the far more common `struct` to be consistent with Cabin's readability and "common English" aesthetics. It's named
	/// exactly what it creates: A group of values.
	///
	/// A token created with this type will always have the value "group".
	///
	/// Like all keywords, this enum variant declaration *must* come before `Identifier`. If it doesn't, then `group` will be tokenized incorrectly as
	/// identifiers, which will cause issues when parsing. Please be careful when moving around this keyword or the `Identifier` token type!
	///
	/// TODO: We are considering renaming this to something like `grouptype`, because it doesn't actually represent a group of values, but instead a *type*
	/// of group of values. I don't really like multi-word keywords and I think this is unnecessary added length, so for now I more prefer `group`, but we
	/// should consider `grouptype` as well as other possibilities. One other option is just `type`, which would act similar to how it does in TypeScript,
	/// but the word "type" has a very specific meaning in Cabin and `group`s are only one specific subset of those, and we don't want to raise confusion.
	KeywordGroup,

	/// The `run` keyword token type, which is used to run an expression at compile-time. In Cabin, all expressions are run at compile-time by default,
	/// but sometimes you want something specifically to be run at compile-time. This keyword forces an expression to be run at runtime. Note, however,
	/// that all sub-expressions should still be evaluated at compile-time. For example, the expression:
	///
	/// ```cabin
	/// run ((1 + 2) + (3 + 4))
	/// ```
	///
	/// at compile-time will convert to:
	///
	/// ```cabin
	/// run 3 + 7
	/// ```
	///
	/// To run it deeply nested, you'd have to do:
	///
	/// ```cabin
	/// run (run (1 + 2) + run (3 + 4))
	/// ```
	///
	/// A token created with this type will always have the value "run".
	///
	/// Like all keywords, this enum variant declaration *must* come before `Identifier`. If it doesn't, then `run` will be tokenized incorrectly as
	/// identifiers, which will cause issues when parsing. Please be careful when moving around this keyword or the `Identifier` token type!
	KeywordRuntime,

	KeywordIs,

	/// The `return` keyword token type. This is used to exit from a function and give a value back to the caller.
	///
	/// A token created with this type will always have the value "return".
	///
	/// TODO: We would like to rename this to something different. Cabin should support tail expressions, and we have to consider what keyword to use for that.
	/// At the same time, it's handy to be able to return out from an entire function as well, so if `return` becomes the keyword for tail expressions,
	/// we should consider what the keyword for returning from the current function should be.
	///
	/// Either way, I don't really like `return` as a keyword for *either* to be honest, so we should consider some other possibilities. One silly idea I had is
	/// using "its" for a tail expression keyword, so things would look like this:
	///
	/// ```cabin
	/// constant image_type = if extension == "png" {
	/// 	it is "Portable networks graphics";
	/// } else {
	/// 	it is "not a PNG!";
	/// };
	/// ```
	///
	/// This reads well for a small example like this, but may be confusing when the bodies are much longer. Regardless it's one possibility to consider.
	///
	/// Like all keywords, this enum variant declaration *must* come before `Identifier`. If it doesn't, then `return` will be tokenized incorrectly as
	/// identifiers, which will cause issues when parsing. Please be careful when moving around this keyword or the `Identifier` token type!
	KeywordReturn,

	/// The `either` keyword token type. This is used to create enumerations.
	///
	/// A token created with this type will always have the value "either".
	///
	/// Like all keywords, this enum variant declaration *must* come before `Identifier`. If it doesn't, then `either` will be tokenized incorrectly as
	/// identifiers, which will cause issues when parsing. Please be careful when moving around this keyword or the `Identifier` token type!
	KeywordOneOf,

	/// The `let` keyword token type. This is used to declare a variable.
	///
	/// A token created with this type will always have the value "let".
	///
	/// Like all keywords, this enum variant declaration *must* come before `Identifier`. If it doesn't, then `let` will be tokenized incorrectly as
	/// identifiers, which will cause issues when parsing. Please be careful when moving around this keyword or the `Identifier` token type!
	KeywordLet,

	/// The `foreach` keyword token type. This is used to iterate over a list.
	///
	/// A token created with this type will always have the value "foreach".
	///
	/// Like all keywords, this enum variant declaration *must* come before `Identifier`. If it doesn't, then `foreach` will be tokenized incorrectly as
	/// identifiers, which will cause issues when parsing. Please be careful when moving around this keyword or the `Identifier` token type!
	KeywordForEach,

	/// The `in` keyword token type. This is used to iterate over a list.
	///
	/// A token created with this type will always have the value "in".
	///
	/// Like all keywords, this enum variant declaration *must* come before `Identifier`. If it doesn't, then `in` will be tokenized incorrectly as
	/// identifiers, which will cause issues when parsing. Please be careful when moving around this keyword or the `Identifier` token type!
	KeywordIn,

	/// The `while` keyword token type. This is used to loop while some condition is true
	///
	/// A token created with this type will always have the value "while".
	///
	/// Like all keywords, this enum variant declaration *must* come before `Identifier`. If it doesn't, then `while` will be tokenized incorrectly as
	/// identifiers, which will cause issues when parsing. Please be careful when moving around this keyword or the `Identifier` token type!
	KeywordWhile,

	KeywordEither,

	/// An identifier in the language. This is essentially a "name" of a variable. Whenever the user creates a new variable with a name, it is represented with
	/// this token type.
	///
	/// Note that the token types are all checked in the order they are declared. The `Identifier` pattern **does not** take special care to not include keywords.
	/// This means that when asked, the `Identifier` token type will match keywords. This means that this variant declaration must occur *after* all keywords
	/// in the language. Please be mindful of this when moving this token type declaration around or when moving keyword token type declarations around.
	///
	/// Currently, although this may change, valid Cabin identifiers follow the following pattern:
	///
	/// ```js
	/// /[A-Za-Z_]\w*/
	/// ```
	/// without the leading and trailing slash of course. This means all identifiers must start with an alphabetical character or a underscore, and then optionally
	/// can have more characters which can be alphabetical, underscore, or a number.
	///
	/// TODO: We should consider what other symbols to allow here. Should we allow dollar signs? Why should we? Why shouldn't we? What about other unused symbols in
	/// the language like @? There should be a general design discussion about what identifiers should be valid.
	Identifier,

	LessThan,
	GreaterThan,

	/// The angle bracket parenthesis token type. This is used for function and group compile-time parameters. This will *always* come some amount of
	/// tokens before a `RightAngleBracket` token; There is no syntax in Cabin that constitutes unmatched angle brackets. This token type, when parsed, will always return
	/// a token with a single character value, which is just a left angle bracket character "<".
	LeftAngleBracket,

	/// The left brace token type. This is used for things like table creation, new scopes, etc. This should *always* come some amount of
	/// tokens before a `RightBrace` token; There is no syntax in Cabin that constitutes unmatched braces. This token type, when parsed, will always return
	/// a token with a single character value, which is just a left brace character "{".
	LeftBrace,

	/// The left bracket token type. This is used for things like list creation, list indexing, etc. This should *always* come some amount of
	/// tokens before a `RightBracket` token; There is no syntax in Cabin that constitutes unmatched brackets. This token type, when parsed, will always return
	/// a token with a single character value, which is just a left bracket character "[".
	LeftBracket,

	/// The left parenthesis token type. This is used for things like parenthesized arithmetic expressions, function calls, etc. This will *always* come some amount of
	/// tokens before a `RightParenthesis` token; There is no syntax in Cabin that constitutes unmatched parenthesis. This token type, when parsed, will always return
	/// a token with a single character value, which is just a left parenthesis character "(".
	LeftParenthesis,

	/// The "minus" token type. This is used for parsing arithmetic subtraction expressions. Any token tokenized of this type will always have a single-character value,
	/// which is the "hyphen" or "minus" value (-).
	Minus,

	/// The number token type. Currently, Cabin only supports very clear decimal literals. It does not currently support binary literals, hex literals, scientific
	/// notation literals, octal literals, etc. Furthermore, unlike Rust, Cabin does not allow number literals with a decimal but no numbers after the decimal,
	/// nor does it allow decimal literals with no numbers proceeding the decimal. To be specific, all numbers in the value currently must match the given regular
	/// expression:
	///
	/// ```js
	/// /-?\d+(\.\d+)?/
	/// ```
	///
	/// without including the leading and trailing slash of course.
	Number,

	/// The "plus" token type. This is used for parsing arithmetic addition expressions. Any token tokenized of this type will always have a single-character value,
	/// which is the "plus" value (+).
	Plus,

	/// The right angle bracket token type. This is used for function and group compile-time parameters. This will *always* come some amount of
	/// tokens after a `LeftAngleBracket` token; There is no syntax in Cabin that constitutes unmatched angle brackets. This token type, when parsed, will always return
	/// a token with a single character value, which is just a right angle bracket character ">".
	RightAngleBracket,

	/// The right brace token type. This is used for things like table creation, new scopes, etc. This will *always* come some amount of
	/// tokens after a `LeftBrace` token; There is no syntax in Cabin that constitutes unmatched brackets. This token type, when parsed, will always return
	/// a token with a single character value, which is just a right brace character "}".
	RightBrace,

	/// The right bracket token type. This is used for things like list creation, list indexing, etc. This will *always* come some amount of
	/// tokens after a `LeftBracket` token; There is no syntax in Cabin that constitutes unmatched brackets. This token type, when parsed, will always return
	/// a token with a single character value, which is just a right bracket character "]".
	RightBracket,

	/// The right parenthesis token type. This is used for things like parenthesized arithmetic expressions, function calls, etc. This will *always* come some amount of
	/// tokens after a `LeftParenthesis` token; There is no syntax in Cabin that constitutes unmatched parenthesis. This token type, when parsed, will always return
	/// a token with a single character value, which is just a right parenthesis character ")".
	RightParenthesis,

	/// The semicolon token type. These are exclusively used in the language to end statements. Cabin is not a whitespace-sensitive language, so semicolons are used to
	/// indicate the end of a statement. This token type, when parsed, will always return a token with a single character value, which is just a semicolon.
	Semicolon,

	/// The string token type. This is a double quoted string. In Cabin, all strings are formatted and multiline by default; However, the parsing of inlined formatted
	/// expressions is done at a later step, so an entire formatted string is still just returned from the lexer as a simple string. The double quotes of the string
	/// are both included in the returned token.
	String,

	/// The whitespace token type. This is a special token type because it is detected by the lexer, but tokens of this type are not added to the token list;
	/// The parser never sees them. This constitutes characters of all standard ASCII whitespace including spaces, tabs, newlines, and carriage returns (which are
	/// outside of strings of course). Cabin is not a whitespace-sensitive language, so these are intentionally ignored when tokenizing.
	Whitespace,
}

impl TokenType {
	// TODO: This could pretty easily be refactored into a non-regex solution that would almost certainly be more performant;
	// It would certainly be less clean and less concise, but it would be more performant, so we should consider this at some point.

	/// Returns a regular expression pattern that matches the token type. This specifically checks if the given string *starts* with the token type.
	/// The returned value is a lazily-evaluated static, so there is no performance loss to calling this repeatedly.
	///
	/// # Returns
	/// A regular expression pattern that matches the token type.
	fn pattern(&self) -> &'static regex_macro::Regex {
		match self {
			// Keywords
			Self::KeywordOneOf => regex_macro::regex!(r"^oneof\b"),
			Self::KeywordEither => regex_macro::regex!(r"^either\b"),
			Self::KeywordIf => regex_macro::regex!(r"^if\b"),
			Self::KeywordAction => regex_macro::regex!(r"^action\b"),
			Self::KeywordNew => regex_macro::regex!(r"^new\b"),
			Self::KeywordOtherwise => regex_macro::regex!(r"^otherwise\b"),
			Self::KeywordLet => regex_macro::regex!(r"^let\b"),
			Self::KeywordGroup => regex_macro::regex!(r"^group\b"),
			Self::KeywordReturn => regex_macro::regex!(r"^return\b"),
			Self::KeywordRuntime => regex_macro::regex!(r"^run\b"),
			Self::KeywordIs => regex_macro::regex!(r"^is\b"),
			Self::KeywordForEach => regex_macro::regex!(r"^foreach\b"),
			Self::KeywordIn => regex_macro::regex!(r"^in\b"),
			Self::KeywordWhile => regex_macro::regex!(r"^while\b"),

			// Left opening groupings
			Self::LeftAngleBracket => regex_macro::regex!("^<"),
			Self::LeftBrace => regex_macro::regex!(r"^\{"),
			Self::LeftBracket => regex_macro::regex!(r"^\["),
			Self::LeftParenthesis => regex_macro::regex!(r"^\("),

			// Right closing groupings
			Self::RightAngleBracket => regex_macro::regex!("^>"),
			Self::RightBrace => regex_macro::regex!(r"^\}"),
			Self::RightBracket => regex_macro::regex!(r"^\]"),
			Self::RightParenthesis => regex_macro::regex!(r"^\)"),

			// Literals
			Self::String => regex_macro::regex!(r#"(?s)^"[^"]*""#),
			Self::Number => regex_macro::regex!(r"^-?\d+(\.\d+)?"),
			Self::Identifier => regex_macro::regex!(r"^[a-zA-Z_]\w*"),

			// Operators
			Self::Plus => regex_macro::regex!(r"^\+"),
			Self::Minus => regex_macro::regex!("^-"),
			Self::Asterisk => regex_macro::regex!(r"^\*"),
			Self::Caret => regex_macro::regex!(r"^\^"),
			Self::Dot => regex_macro::regex!(r"^\."),
			Self::DoubleEquals => regex_macro::regex!("^=="),
			Self::ForwardSlash => regex_macro::regex!("^/"),
			Self::Equal => regex_macro::regex!("^="),
			Self::LessThan => regex_macro::regex!(r"^\s+<"),
			Self::GreaterThan => regex_macro::regex!(r"^\s+>"),

			// Punctuations / Misc
			Self::TagOpening => regex_macro::regex!(r"^\#\["),
			Self::Colon => regex_macro::regex!("^:"),
			Self::Comma => regex_macro::regex!("^,"),
			Self::Semicolon => regex_macro::regex!("^;"),

			// Ignored tokens
			Self::Whitespace => regex_macro::regex!(r"^\s"),
			Self::LineComment => regex_macro::regex!(r"^//[^\n\r]*"),
		}
	}

	/// Returns the matched text of the token type in the given code. This only returns `Some` if there is a match *at the start* of the string to this
	/// token type. Even if this token type exists in the given code, but occurs later than the start, this will return `None`.
	///
	/// # Parameters
	/// - `code`: The code to find a match in.
	///
	/// # Returns
	/// The matched text of the token type in the given code, or `None` if no match was found.
	#[must_use]
	pub fn get_match(&self, code: &str) -> Option<String> {
		self.pattern().find(code).map(|m| m.as_str().to_owned())
	}

	/// Finds the first token type that matches the given code.
	///
	/// # Parameters
	/// - `code`: The code to find a match for.
	///
	/// # Returns
	/// The first token type that matches the given code, along with the matched text.
	#[must_use]
	fn find_match(code: &str) -> Option<(Self, String)> {
		for token_type in Self::iter() {
			if let Some(matched) = token_type.get_match(code) {
				return Some((token_type, matched));
			}
		}
		None
	}
}

impl std::fmt::Display for TokenType {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", format!("{self:?}").to_case(convert_case::Case::Title))
	}
}

/// A token in source code.
#[derive(Debug)]
pub struct Token {
	/// The type of the token.
	pub token_type: TokenType,
	/// The value of the token. This value is how the token originally appears in the source code *exactly*. There are some nuances to what is considered
	/// part of the value; For example, all strings retain their quotes in this field. For information about what is considered part of the `value` for
	/// a specific token type, refer to the documentation for that specific token type.
	pub value: String,
	/// The line number of the token. This is the line number as it appears in the source code passed to `tokenize`. It is up to the caller to do
	/// arithmetic to find out the actual line number the token was written on, for example, with multiple files. Keep in mind that there is a global
	/// prelude code (see `/prelude.cbn`) that is added to all Cabin projects, so that must be taken into account as well when error reporting with
	/// this `line` field.
	pub line: usize,
	/// The column number of the token.
	pub column: usize,
}

/// Tokenizes a string of Cabin source code into a vector of tokens. This is the first step in compiling Cabin source code. The returned vector of tokens
/// should be passed into the Cabin parser, which will convert it into an abstract syntax tree.
///
/// # Parameters
/// - `code` - The Cabin source code. If the given code is not valid Cabin code, this function makes no guarantees to return an error, nor does it make
/// a guarantee to return an `Ok`. This includes semantic and syntactic errors. This function will only return an error if an unrecognized token is found;
/// Meaning a piece of code is encountered that doesn't match any known token types. This could be a non-ASCII character or just generally any unused character
/// in the language like `@`.
///
/// # Returns
/// A vector of tokens in the order they appeared in the given source code after tokenization, or an `Err` if an unrecognized token was found.
///
/// # Errors
/// If the given code string is not syntactically valid Cabin code. It needn't be semantically valid, but it must be comprised of the proper tokens.
#[allow(clippy::missing_panics_doc)]
pub fn tokenize(code: &str) -> anyhow::Result<VecDeque<Token>> {
	let mut code = code.to_owned();
	code = code.replace('\t', "    ");

	let mut tokens = Vec::new();
	let mut line = 1;
	let mut column = 1;

	// We only read tokens from the start of a string, so we repeatedly loop over the code and remove the tokenized text when we find tokens.
	// This means we can just iterate while code isn't empty.
	while !code.is_empty() {
		// We've got a match - we found a token that matches the start of the code
		if let Some((token_type, value)) = TokenType::find_match(&code) {
			let length = value.len(); // This must be done early so that we aren't trying to get the length of a moved value
			let newline_count = value.chars().filter(|char| *char == '\n').count();

			// Add the token - ignore whitespace and comments!
			if token_type != TokenType::Whitespace && token_type != TokenType::LineComment {
				let token = Token { token_type, value, line, column };
				tokens.push(token);
			}
			// If it is whitespace, Add to the newlines!
			else {
				line += newline_count;
			}

			// Update the column and the code so that the code cuts off the bit we just tokenized
			column = if newline_count > 0 { 1 } else { column + length };
			code = code.get(length..).unwrap().to_owned();
		}
		// Unrecognized token - return an error!
		else {
			anyhow::bail!(
				"{line}:{column}:{severity}:Unrecognized token: {code}",
				severity = "error",
				code = code.split('\n').next().unwrap()
			);
		}
	}

	// We'll only get here if we didn't get any errors, so we can just return the tokens wrapped in an `Ok`
	Ok(VecDeque::from(tokens))
}
