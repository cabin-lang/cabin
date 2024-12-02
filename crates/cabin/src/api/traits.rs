use try_as::traits::{TryAsMut, TryAsRef};

pub trait TryAs {
	/// Attempts to convert this enum variant into the given type. This is a generic wrapper
	/// around `try_as_ref`.
	///
	/// # Errors
	///
	/// If this enum is of the wrong variant.
	fn try_as<T>(&self) -> anyhow::Result<&T>
	where
		Self: TryAsRef<T>,
	{
		self.try_as_ref().ok_or_else(|| anyhow::anyhow!("Incorrect variant"))
	}
}
pub trait TryAsRefMut {
	/// Attempts to convert this enum variant into the given type. This is a generic wrapper
	/// around `try_as_mut`.
	///
	/// # Errors
	///
	/// If this enum is of the wrong variant.
	fn try_as_ref_mut<T>(&mut self) -> anyhow::Result<&mut T>
	where
		Self: TryAsMut<T>,
	{
		self.try_as_mut().ok_or_else(|| anyhow::anyhow!("Incorrect variant"))
	}
}

impl<T> TryAs for T {}
impl<T> TryAsRefMut for T {}
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
