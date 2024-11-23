use try_as::traits::{TryAsMut, TryAsRef};

pub trait TryAs {
	fn try_as<T>(&self) -> anyhow::Result<&T>
	where
		Self: TryAsRef<T>,
	{
		self.try_as_ref().ok_or_else(|| anyhow::anyhow!("Incorrect variant"))
	}

	fn expect_as<T>(&self) -> anyhow::Result<&T>
	where
		Self: TryAsRef<T>,
	{
		self.try_as()
	}
}
pub trait TryAsRefMut {
	fn try_as_ref_mut<T>(&mut self) -> anyhow::Result<&mut T>
	where
		Self: TryAsMut<T>,
	{
		self.try_as_mut().ok_or_else(|| anyhow::anyhow!("Incorrect variant"))
	}
}

impl<T> TryAs for T {}
impl<T> TryAsRefMut for T {}

pub trait TupleOption<T, U> {
	/// Converts an `Option<(T, U)>` into an `(Option<T>, Option<U>)`.
	fn deconstruct(self) -> (Option<T>, Option<U>);
}

impl<T, U> TupleOption<T, U> for Option<(T, U)> {
	fn deconstruct(self) -> (Option<T>, Option<U>) {
		if let Some((first, second)) = self {
			(Some(first), Some(second))
		} else {
			(None, None)
		}
	}
}

pub trait TerminalOutput {
	fn as_terminal_output(&self) -> String;
}

impl<T: AsRef<str>> TerminalOutput for T {
	fn as_terminal_output(&self) -> String {
		let string = self.as_ref();

		// Max line length
		let max_line_length = 100;

		// Format the input tokens, unindent it, and remove all newlines
		let formatted = string.replace('\n', " ").trim().to_owned();
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
	}
}
