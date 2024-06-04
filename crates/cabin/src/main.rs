//! # Cabin
//!
//! A dead simple, highly performant, and extremely safe programming language.
//!
//! ## Installation
//!
//! Cabin can be installed cross-platform with Cargo:
//!
//! ```bash
//! cargo install cabin-language
//! ```

/// Bring the `Parser` trait into scope from `clap`, which allows parsing argument structs from the command line. We assign it to underscore to indicate
/// clearly that it's not used outside of bringing its trait methods into scope.
use clap::Parser as _;

use crate::cli::{commands::CabinCommand, CabinCompilerArguments};

/// The main entry point for the Cabin compiler. This parses the arguments passed at the command-line, and runs the
/// given subcommand (`run`, `new`, `build`, etc.)
fn main() -> anyhow::Result<()> {
	CabinCompilerArguments::parse().command.execute()
}
