use crate::{context::Context, parser::expressions::Expression};

#[macro_export]
macro_rules! list {
	(
		$context: expr, $scope_id: expr, $elements: expr
	) => {{
		let constructor = ObjectConstructor {
			type_name: Name::from("List"),
			fields: Vec::new(),
			internal_fields: HashMap::from([("elements".to_owned(), InternalFieldValue::List($elements))]),
			scope_id: $scope_id,
			object_type: ObjectType::Normal,
		};

		Expression::ObjectConstructor(constructor)
	}};
}

#[macro_export]
macro_rules! literal_list {
	(
		$context: expr, $scope_id: expr, $elements: expr
	) => {{
		let constructor = ObjectConstructor {
			type_name: Name::from("List"),
			fields: Vec::new(),
			internal_fields: HashMap::from([("elements".to_owned(), $crate::parser::expressions::object::InternalFieldValue::List($elements))]),
			scope_id: $scope_id,
			object_type: ObjectType::Normal,
		};

		let literal = LiteralObject::try_from_object_constructor(constructor, $context).unwrap();
		Expression::Pointer($context.virtual_memory.store(literal))
	}};
}

#[macro_export]
macro_rules! new_object {
	(
		$type_name: ident {
			$($field_name: ident = $field_value: expr),* $(,)?
		}, $scope_id: expr
	) => {
		ObjectConstructor {
			type_name: stringify!($type_name).into(),
			fields: vec![$(
				$crate::parser::expressions::object::Field {
					name: stringify!($field_name).into(),
					value: Some($field_value),
					field_type: None
				}
			),*],
			internal_fields: std::collections::HashMap::new(),
			scope_id: $scope_id,
			object_type: $crate::parser::expressions::object::ObjectType::Normal
		}
	};
}

#[macro_export]
macro_rules! literal {
	(
		$context: expr, $($tokens: tt)*
	) => {{
		let constructor = $crate::new_object!($($tokens)*);
		let literal = LiteralObject::try_from_object_constructor(constructor, $context).unwrap();
		let address = $context.virtual_memory.store(literal);
		Expression::Pointer(address)
	}};
}

#[macro_export]
macro_rules! object {
	($($tokens: tt)*) => {
		Expression::ObjectConstructor($crate::new_object!($($tokens)*))
	};
}

#[macro_export]
macro_rules! string_literal {
	(
		$value: expr, $context: expr
	) => {
		Expression::Pointer(ObjectConstructor::from_string($value, $context))
	};
}

pub fn cabin_true(context: &Context) -> Expression {
	context.scope_data.get_global_variable(&"true".into()).unwrap().value.to_owned_literal().unwrap()
}
