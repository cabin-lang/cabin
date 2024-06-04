/// Creates a cabin object with the given fields, and returns the table wrapped in a literal wrapped in an expression.
///
/// Groups which have generic parameters originally had their parameter values just set to `UnknownAtCompileTime` like normal functions, but this
/// makes it difficult for `represent` statements to resolve which parameters are which. So, instead, we make them an object, which has an `index`
/// field, indicating the index of the parameter. This allows us to reverse-engineer which generic parameters are which.
#[macro_export]
macro_rules! object {
	(
		$name: tt {
			$(fields = {$($field_name: tt = $field_value: expr),*})?
			$(internal_fields = {$($internal_name: tt = $internal_value: expr),*})?
		}
	) => {{
		let mut table = $crate::parser::expressions::literals::object::Object::new();
		table.name = $crate::parser::expressions::util::name::Name(stringify!($name).to_owned());
		$($(
			table.add_field(DeclarationData {
				name: Name(stringify!($field_name).to_owned()),
				value: $field_value.into(),
				tags: TagList::default(),
				type_annotation: None,
			});
		)*)?

		$($(
			table.add_internal_field(stringify!($internal_name).to_owned(), $internal_value);
		)*)?

		$crate::parser::expressions::Expression::Literal($crate::parser::expressions::literals::Literal::new($crate::parser::expressions::literals::LiteralValue::Object(table)))
	}};
}

/// Creates a new number as an expression in the language with the given number value.
#[macro_export]
macro_rules! number {
	($number: expr) => {
		$crate::object! {
			Number {
				internal_fields = {
					internal_value = $crate::parser::expressions::literals::object::InternalValue::Number($number as f64)
				}
			}
		}
	};
}

/// Creates a new string as an expression in the language with the given string value.
#[macro_export]
macro_rules! string {
	($string: expr) => {
		$crate::object! {
			Text {
				internal_fields = {
					internal_value = $crate::parser::expressions::literals::object::InternalValue::String($string.to_owned())
				}
			}
		}
	};
}

/// Creates a boolean literal as an expression based on the given boolean value. This is used by built-in functions
/// and other parts of the compiler to easily create boolean literals.
#[macro_export]
macro_rules! boolean {
	($value: expr) => {
		if $value {
			$crate::global_var!("true")
		} else {
			$crate::global_var!("false")
		}
	};
}

/// Creates a cabin object with the given fields, and returns the table wrapped in a literal wrapped in an expression.
///
/// Groups which have generic parameters originally had their parameter values just set to `UnknownAtCompileTime` like normal functions, but this
/// makes it difficult for `represent` statements to resolve which parameters are which. So, instead, we make them an object, which has an `index`
/// field, indicating the index of the parameter. This allows us to reverse-engineer which generic parameters are which.
#[macro_export]
macro_rules! object_literal {
	(
		$name: tt {
			$(fields = {$($field_name: tt = $field_value: expr),*})?
			$(internal_fields = {$($internal_name: tt = $internal_value: expr),*})?
		}
	) => {{
		let mut table = $crate::parser::expressions::literals::object::Object::new();
		table.name = Name(stringify!($name).to_owned());
		$($(
			table.add_field(DeclarationData {
				name: Name(stringify!($field_name).to_owned()),
				value: $field_value.into(),
				tags: TagList::default(),
				type_annotation: None,
			});
		)*)?

		$($(
			table.add_internal_field(stringify!($internal_name).to_owned(), $internal_value);
		)*)?

		$crate::parser::expressions::literals::LiteralValue::Object(table)
	}};
}

/// Creates a new variable reference with the given name and referenced scope ID, and returns the result as an expression.
#[macro_export]
macro_rules! var {
	($name: expr, $scope_id: expr) => {
		$crate::parser::expressions::Expression::Literal($crate::parser::expressions::literals::Literal::new(
			$crate::parser::expressions::literals::LiteralValue::VariableReference($crate::parser::expressions::literals::variable_reference::VariableReference::new(
				$crate::parser::expressions::util::name::Name($name.to_owned()),
				$scope_id,
			)),
		))
	};
}

/// Creates a new variable reference with the given name and referenced scope ID, and returns the result as a literal
#[macro_export]
macro_rules! var_literal {
	($name: expr, $scope_id: expr) => {
		$crate::parser::expressions::literals::Literal::new($crate::parser::expressions::literals::LiteralValue::VariableReference(
			$crate::parser::expressions::literals::variable_reference::VariableReference::new($crate::parser::expressions::util::name::Name($name.to_owned()), $scope_id),
		))
	};
}

/// Creates a new global variable reference with the given name and returns the result as an expression.
#[macro_export]
macro_rules! global_var {
	($name: expr) => {
		$crate::parser::expressions::Expression::Literal($crate::parser::expressions::literals::Literal::new(
			$crate::parser::expressions::literals::LiteralValue::VariableReference($crate::parser::expressions::literals::variable_reference::VariableReference::new(
				$crate::parser::expressions::util::name::Name($name.to_owned()),
				0,
			)),
		))
	};
}

/// Returns an identifier referencing the `Void` constant as an expression. This is used by many built-in functions and other places in the code to return void.
#[macro_export]
macro_rules! void {
	() => {
		$crate::global_var!("Void")
	};
}

/// Returns an identifier referencing the `Void` constant as a literal. This is used by many built-in functions and other places in the code to return void.
#[macro_export]
macro_rules! void_literal {
	() => {
		$crate::var_literal!("Void", 0)
	};
}

#[macro_export]
macro_rules! block {
	({ $($statement: expr;)* }, $context: expr) => {
		{
			$context.scope_data.enter_new_scope($crate::scopes::ScopeType::Block);
			let inner_scope_id = $context.scope_data.unique_id();
			$context.scope_data.exit_scope()?;
			$crate::parser::statements::Statement::Expression($crate::parser::expressions::Expression::Block($crate::parser::expressions::block::Block { statements: vec![$($statement,)*], inner_scope_id }))
		}
	};
}
