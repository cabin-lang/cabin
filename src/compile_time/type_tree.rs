use std::collections::HashMap;

use colored::Colorize as _;

use crate::parser::expressions::util::name::Name;

/// A set of trees representing variable dependencies. Each tree is composed of `Name` nodes that represent variable names. The root of each tree is the name of the variable,
/// and each child is a dependency of that variable. This is used to detect dependency cycles in the global scope. For example, in the program:
///
/// ```cabin
/// let x = y;
/// let y = x;
/// ```
///
/// the variables x and y depend on each other. This would cause a stack overflow in the compiler, so we need to detect these cycles, which is the responsibility of this struct.
pub struct VariableDependencyTreeSet {
	/// The trees stored in this dependency set. This is stored as a `HashMap` where each key is the name of the root of the tree, and the value is the set of variables that it directly depends on.
	/// To recursively check if one variable depends on another, use `depends_on()`.
	nodes: HashMap<Name, Vec<Name>>,

	/// The current variable that is being dependency-tracked. This is a stack so that when multiple declarations occur simultaneously (such as a function where there are variables being declared in the
	/// function body before the declaration for the function itself ends), we can pop back into the previous dependency tree.
	current_stack: Vec<Name>,
}

impl VariableDependencyTreeSet {
	/// Creates a new empty `VariableDependencyTreeSet`.
	#[must_use]
	pub fn new() -> Self {
		Self {
			nodes: HashMap::new(),
			current_stack: Vec::new(),
		}
	}

	/// Adds a variable as a dependency of the current variable. If the dependency depends on the current variable, this is a circular dependency, and an error is returned. If not,
	/// `Ok(())` is returned.
	///
	/// # Parameters
	/// - `dependency` - The variable to add as a dependency of the current variable.
	///
	/// # Returns
	/// an error if the dependency also depends on the current variable.
	pub fn add_dependency(&mut self, dependency: Name) -> anyhow::Result<()> {
		let current = self
			.current_stack
			.last()
			.ok_or_else(|| anyhow::anyhow!("Attempted to add a dependency to the current variable dependency subgraph, but there is no current variable dependency subgraph"))?;

		if &dependency != current && self.depends_on(&dependency, current) {
			anyhow::bail!(
  				"Variable dependency cycle detected: The variable \"{current}\" depends on the variable \"{dependency}\", but the variable \"{dependency}\" depends on the variable \"{current}\"",
  				current = current.cabin_name().bold().cyan(),
  				dependency = dependency.cabin_name().bold().cyan()
  			);
		}

		let dependencies = self.nodes.get_mut(current).unwrap_or_else(|| unreachable!());
		if !dependencies.contains(&dependency) {
			dependencies.push(dependency);
		}

		Ok(())
	}

	/// Creates a new dependency tree and sets it as the current dependency tree. The current dependency tree can be restored to the previous one with `close_tree()`, which should always be called
	/// at some point after this.
	///
	/// # Parameters
	/// - `name` - The name of the root node for the new dependency tree.
	pub fn create_new_tree_and_set_current(&mut self, name: Name) {
		self.nodes.insert(name.clone(), Vec::new());
		self.current_stack.push(name);
	}

	/// Sets the current dependency tree to the previous one, or `None` if we are currently in the first dependency tree. If there is currently no dependency tree, an error is returned.
	/// This should only and always be called after a corresponding call to `create_new_tree_and_set_current`.
	///
	/// # Returns
	/// an error if there is no current dependency tree.
	pub fn close_tree(&mut self) -> anyhow::Result<()> {
		let Some(_) = self.current_stack.pop() else {
			anyhow::bail!("Attempted to close a variable dependency subgraph but none currently is being built")
		};

		Ok(())
	}

	/// Recursively checks if one variable depends on another. This is used to detect cycles; When adding dependencies, this checks if the reverse dependency exists, i.e., when adding `y`
	/// as a dependency of `x`, we check if `x` is already a dependency of `y`.
	///
	/// # Parameters
	/// - `parent` - The name of the variable to check if depends on another
	/// - `possible_child` - The name of the possibly dependency to check if it's actually a dependency.
	///
	/// # Returns
	/// `true` iff `parent` depends on `possible_child`. If there is no dependency tree stored for `parent`, `false` is returned. If there is no dependency graph stored for `possible_child`,
	/// `false` is returned. If `parent` and `possible_child` are the same, `true` is returned.
	fn depends_on(&self, parent: &Name, possible_child: &Name) -> bool {
		if !self.nodes.contains_key(parent) {
			return false;
		}

		if parent == possible_child {
			return true;
		}

		for child in self.nodes.get(parent).unwrap() {
			if self.depends_on(child, possible_child) {
				return true;
			}
		}

		false
	}

	/// Returns all dependencies, including sub-dependencies, for the variable with the given name.
	///
	/// # Parameters
	/// - `parent` - The name of the variable to get the dependencies of.
	#[must_use]
	pub fn dependencies(&self, parent: &Name) -> Vec<Name> {
		let Some(parent_tree) = self.nodes.get(parent) else {
			return Vec::new();
		};

		let mut dependencies = parent_tree.to_owned();
		for dependency in parent_tree {
			dependencies.push(dependency.clone());
			for sub_dependency in self.dependencies(dependency) {
				dependencies.push(sub_dependency);
			}
		}

		dependencies
	}

	/// Returns whether or not a dependency tree is currently being built.
	#[must_use]
	pub fn is_creating_dependency_tree(&self) -> bool {
		!self.current_stack.is_empty()
	}
}
