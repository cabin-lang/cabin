use try_as::traits::TryAsRef;

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

impl<T> TryAs for T {}

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
