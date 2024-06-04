use std::fmt::Display;

/// A trait that allows adding English suffixes (like "st", "nd", "rd", and "th") to numbers.
pub trait IntegerSuffix: Display {
	/// Returns the "suffix" for the given number. For numbers that end in 1, this is "st"; For numbers that end in 2, this is "nd";
	/// For numbers that end in 3, this is "nd"; For all other numbers, this is "th".
	///
	/// # Returns
	/// The English suffix of the number
	fn suffix(&self) -> &'static str;

	/// Returns this number with its suffix, as specified by `suffix()`.
	///
	/// # Returns
	/// The number followed immediately by its suffix.
	fn suffixed(&self) -> String {
		format!("{self}{}", self.suffix())
	}
}

impl IntegerSuffix for usize {
	fn suffix(&self) -> &'static str {
		match self % 10 {
			1 => "st",
			2 => "nd",
			3 => "rd",
			_ => "th",
		}
	}
}
