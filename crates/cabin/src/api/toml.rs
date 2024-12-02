use std::ops::Deref;

use semver::Version;

/// Defines a TOML schema with default values that creates static types for the schema and provides
/// validation for property setting. See `crate::api::options` for example usage.
#[macro_export]
macro_rules! toml {
	(
		$document_name: ident: $path: literal:
		$(
			[$heading: ident]
			$(
				$(#[$attrs: meta])?
				$name: ident: $type: ty = $value: expr;
			)*
		)*
	) => {
		use $crate::api::toml::IntoTomlItem as _;
		use $crate::api::toml::SetOption as _;

		paste::paste! {
			#[derive(Default, Debug)]
			pub struct [<$document_name:camel Toml>] {
				$(
					$heading: [<$document_name:camel $heading:camel>],
				)*
			}

			impl [<$document_name:camel Toml>] {
				$(
					pub const fn $heading(&self) -> &[<$document_name:camel $heading:camel>] {
						&self.$heading
					}

					pub fn [<$heading _mut>](&mut self) -> &mut [<$document_name:camel $heading:camel>] {
						&mut self.$heading
					}
				)*
			}

			impl From<&mut [<$document_name:camel Toml>]> for toml_edit::DocumentMut {
				fn from(value: &mut [<$document_name:camel Toml>]) -> toml_edit::DocumentMut {
					let mut document = toml_edit::DocumentMut::new();
					$(
						let _ = document.as_table_mut().insert(stringify!($heading), (&mut value.$heading).into());
					)*
					document
				}
			}

			pub struct [<$document_name:camel TomlWriteOnDrop>]<'toml> {
				options: &'toml mut CabinToml,
				root_directory: PathBuf,
			}

			impl<'toml> [<$document_name:camel TomlWriteOnDrop>]<'toml> {
				pub fn new(options: &'toml mut CabinToml, root_directory: PathBuf) -> Self {
					Self { options, root_directory }
				}
			}

			impl Deref for [<$document_name:camel TomlWriteOnDrop>]<'_> {
				type Target = CabinToml;

				fn deref(&self) -> &Self::Target {
					self.options
				}
			}

			impl DerefMut for [<$document_name:camel TomlWriteOnDrop>]<'_> {
				fn deref_mut(&mut self) -> &mut Self::Target {
					self.options
				}
			}

			impl Drop for [<$document_name:camel TomlWriteOnDrop>]<'_> {
				fn drop(&mut self) {
					let path = self.root_directory.join($path);
					let config: toml_edit::DocumentMut = self.options.into();
					std::fs::write(path, config.to_string()).unwrap();
				}
			}
		}

		$(
			paste::paste! {
				#[derive(Debug)]
				pub struct [<$document_name:camel $heading:camel>] {
					$(
						$(#[$attrs])?
						$name: $crate::api::toml::TomlValue<$type>
					),*
				}

				impl Default for [<$document_name:camel $heading:camel>] {
					fn default() -> [<$document_name:camel $heading:camel>] {
						[<$document_name:camel $heading:camel>] {
							$(
								$name: $value.into()
							),*
						}
					}
				}

				impl [<$document_name:camel $heading:camel>] {
					pub fn try_set(&mut self, name: &str, value: &str) -> anyhow::Result<()> {
						match name.to_case(Case::Snake).as_str() {
							$(
								stringify!($name) => self.$name.try_set_value(value)?,
							)*
							_ => anyhow::bail!("Unknown option: {}", name.to_case(Case::Kebab))
						}

						Ok(())
					}

					$(
						pub fn $name(&self) -> $type {
							self.$name.value.clone().unwrap()
						}
					)*
				}

				impl From<&mut [<$document_name:camel $heading:camel>]> for toml_edit::Item {
					fn from(value: &mut [<$document_name:camel $heading:camel>]) -> toml_edit::Item {
						let mut table = toml_edit::Table::new();
						$(
							let _ = table.insert(stringify!($name), value.$name().into_toml_item());
						)*
						toml_edit::Item::Table(table)
					}
				}
			}
		)*
	};
}

#[macro_export]
macro_rules! choose {
	(
		$($values: expr),* $(,)?
	) => {
		$crate::api::toml::choose_from(&[$($values),*])
	};
}

#[derive(Debug)]
pub struct TomlValue<T: Clone> {
	pub value: Option<T>,
	pub choices: Vec<T>,
}

impl<T: Clone> TomlValue<T> {
	pub const fn new(value: T) -> TomlValue<T> {
		TomlValue {
			value: Some(value),
			choices: Vec::new(),
		}
	}

	pub fn default(mut self, value: T) -> Self {
		self.value = Some(value);
		self
	}
}

pub trait SetOption {
	fn try_set_value(&mut self, value: &str) -> anyhow::Result<()>;
}

impl SetOption for TomlValue<bool> {
	fn try_set_value(&mut self, value: &str) -> anyhow::Result<()> {
		self.value = Some(value.parse()?);
		Ok(())
	}
}

impl SetOption for TomlValue<Version> {
	fn try_set_value(&mut self, value: &str) -> anyhow::Result<()> {
		self.value = Some(value.parse()?);
		Ok(())
	}
}

impl SetOption for TomlValue<String> {
	fn try_set_value(&mut self, value: &str) -> anyhow::Result<()> {
		if !self.choices.is_empty() && !self.choices.contains(&value.to_owned()) {
			anyhow::bail!("invalid value")
		}
		self.value = Some(value.to_owned());
		Ok(())
	}
}

impl SetOption for TomlValue<Option<String>> {
	fn try_set_value(&mut self, value: &str) -> anyhow::Result<()> {
		self.value = Some(Some(value.to_owned()));
		Ok(())
	}
}

impl SetOption for TomlValue<i64> {
	fn try_set_value(&mut self, value: &str) -> anyhow::Result<()> {
		if !self.choices.is_empty() && !self.choices.contains(&value.parse()?) {
			anyhow::bail!("invalid value")
		}
		self.value = Some(value.parse()?);
		Ok(())
	}
}

impl<T: Clone> Deref for TomlValue<T> {
	type Target = T;

	fn deref(&self) -> &Self::Target {
		self.value.as_ref().unwrap()
	}
}

pub fn choose_from<T: Clone>(choices: &[T]) -> TomlValue<T> {
	TomlValue {
		value: None,
		choices: choices.to_vec(),
	}
}

impl<T: Clone> From<T> for TomlValue<T> {
	fn from(value: T) -> Self {
		TomlValue::new(value)
	}
}

impl From<&str> for TomlValue<String> {
	fn from(value: &str) -> Self {
		TomlValue::new(value.to_owned())
	}
}

impl From<TomlValue<&str>> for TomlValue<String> {
	fn from(value: TomlValue<&str>) -> Self {
		TomlValue {
			value: value.value.map(str::to_owned),
			choices: value.choices.into_iter().map(&str::to_owned).collect(),
		}
	}
}

impl IntoTomlValue for Version {
	fn into_toml(self) -> toml_edit::Value {
		self.to_string().into()
	}
}

pub trait IntoTomlValue {
	fn into_toml(self) -> toml_edit::Value;
}

pub trait IntoTomlItem {
	fn into_toml_item(self) -> toml_edit::Item;
}

impl<T: IntoTomlValue> IntoTomlItem for T {
	fn into_toml_item(self) -> toml_edit::Item {
		self.into_toml().into()
	}
}

macro_rules! impl_into_toml {
	(
		$(
			$type: ty
		),*
	) => {
		$(
			impl IntoTomlValue for $type {
				fn into_toml(self) -> toml_edit::Value {
					self.into()
				}
			}
		)*
	};
}

impl_into_toml!(String, i64, bool);
