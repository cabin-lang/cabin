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
