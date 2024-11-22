use std::collections::HashMap;

use try_as::traits::TryAsRef;

use crate::{
	api::{context::Context, macros::TerminalOutput, traits::TryAs as _},
	bail_err,
	comptime::memory::VirtualPointer,
	lexer::Position,
	parser::expressions::{
		group::GroupDeclaration,
		name::Name,
		object::{InternalFieldValue, ObjectConstructor, ObjectType},
		Expression, Typed,
	},
	transpiler::TranspileToC,
};

/// A "literal object". Literal objects can be thought of as simple associative arrays, similar to a JSON object or similar.
/// Specifically, a literal object is a collection of fields where each field's value is another literal object.
///
/// You may notice that there's no `LiteralObject` variant of `Expression`. This is because literal objects live in "virtual memory",
/// and instead we refer to them with "virtual pointers" via the `Pointer` struct. You can read more about this in the documentation
/// for `VirtualMemory` and `context.virtual_memory`.
///
/// `LiteralObjects` are equivalent to types in Cabin. Cabin allows arbitrary expressions to be used as types, as long as "the entire
/// expression can be evaluated at compile-time" which just means that it can be evaluated down to a `LiteralObject`. If you want to
/// check or ensure that an expression is a type, check if it's a pointer to a `LiteralObject`.
///
/// Many constructs in Cabin are stored as `LiteralObject` that you might not expect. For example, all group declarations, either declarations,
/// function declarations, and one-of declarations are stored as literal objects. That's because at their core, all information about them is
/// known at compile-time. Any such object should be stored as a `LiteralObject`. Read the documentation on the `LiteralConvertible` trait for
/// more information about how these types of syntaxes are stored as and retrieved from `LiteralObjects`.
#[derive(Debug, Clone)]
pub struct LiteralObject {
	/// The type name of this `LiteralObject`. This is the name that the object would be constructed with in an object constructor, such as `Text`,
	/// `Number`, `Object`, etc.
	type_name: Name,

	/// The fields on this `LiteralObject`, as a map between field names and pointers to `LiteralObjects` as field values. This should be immutable
	/// after the object's creation; The whole point of being a literal is that it's known entirely at compile-time and won't change.
	fields: HashMap<Name, VirtualPointer>,

	/// The "internal" fields of this `LiteralObject`. These are special values that special types or objects need to store. These aren't accessible
	/// from within Cabin. For example, the `Text` group stores a `String` internally here, representing it's actual string value; `Number` behaves
	/// similarly.
	internal_fields: HashMap<String, InternalFieldValue>,

	object_type: ObjectType,

	scope_id: usize,

	pub name: Name,

	/// The address of this `LiteralObject` in memory. In theory, all `LiteralObjects` are stored in `VirtualMemory`, and thus have a unique
	/// address that points to them. This is an `Option`, however, because in theory a literal object could be constructed without being stored
	/// in memory for some reason, such as if a temporary value is needed. This is \*generally\* safe to `unwrap()`; It's only in rare exception
	/// cases that a `LiteralObject` will exist that doesn't live in `VirtualMemory`.
	///
	/// This is set to `Some` whenever the object is given to virtual memory, and `VirtualMemory` takes responsibility for updating it if it needs
	/// to be moved in memory or taken out of memory. See the `move_and_overwrite()` function on `VirtualMemory` for an example of this, which is
	/// called by `Declaration::evaluate_at_compile_time()`.
	pub address: Option<VirtualPointer>,
}

impl LiteralObject {
	/// Attempts to convert an `ObjectConstructor` expression into a literal value. This is possible if and only if all
	/// fields of the object constructor are themselves either literals or other `ObjectConstructors` that are capable of
	/// being converted into a literal value.
	pub fn try_from_object_constructor(object: ObjectConstructor, context: &mut Context) -> anyhow::Result<Self> {
		let mut fields = HashMap::new();
		for field in object.fields {
			let value = field.value.unwrap();
			if let Expression::Pointer(address) = value {
				fields.insert(field.name, address);
				continue;
			}

			let name = value.kind_name();
			let Expression::ObjectConstructor(field_object) = value else {
				bail_err! {
					base = "A value that's not fully known at compile-time was used as a type.",
					while = format!("checking the field \"{}\" of a value at compile-time", field.name.unmangled_name().bold().cyan()),
					context = context,
					position = field.name.position().unwrap_or_else(Position::zero),
					details = expression_formatter::format!(
						r#"
                        Although Cabin allows arbitrary expressions to be used as types, the expression needs to be able to 
						be fully evaluated at compile-time. The expression that this error refers to must be a literal object, 
						but instead it's a {name}. {if &name.to_lowercase() == "name" {
							"
							This means that you put a variable name where a type is required, but the value of that variable
							is some kind of expression that can't be fully evaluated at compile-time.
							"
						} else {
							""
						}}"#
					).as_terminal_output()
				};
			};

			let value_address = LiteralObject::try_from_object_constructor(field_object, context)?.store_in_memory(context);
			fields.insert(field.name, value_address);
		}

		Ok(LiteralObject {
			type_name: object.type_name,
			fields,
			internal_fields: object.internal_fields,
			object_type: object.object_type,
			scope_id: object.scope_id,
			name: object.name,
			address: None,
		})
	}

	pub fn type_name(&self) -> &Name {
		&self.type_name
	}

	pub fn object_type(&self) -> &ObjectType {
		&self.object_type
	}

	pub fn name(&self) -> &Name {
		&self.name
	}

	pub fn get_field(&self, name: impl Into<Name>) -> Option<Expression> {
		self.fields.get(&name.into()).map(|address| Expression::Pointer(*address))
	}

	pub fn get_field_literal<'a>(&'a self, name: impl Into<Name>, context: &'a Context) -> Option<&'a LiteralObject> {
		self.fields.get(&name.into()).map(|address| context.virtual_memory.get(address))
	}

	pub fn expect_field_literal<'a>(&'a self, name: impl Into<Name>, context: &'a Context) -> &'a LiteralObject {
		self.get_field_literal(name, context).unwrap()
	}

	pub fn expect_field_literal_as<'a, T>(&'a self, name: impl Into<Name>, context: &'a Context) -> &T
	where
		LiteralObject: TryAsRef<T>,
	{
		self.get_field_literal(name, context).unwrap().expect_as().unwrap()
	}

	pub fn get_internal_field<T>(&self, name: &str) -> anyhow::Result<&T>
	where
		InternalFieldValue: TryAsRef<T>,
	{
		self.internal_fields
			.get(name)
			.ok_or_else(|| anyhow::anyhow!("Attempted to get an internal field that doesn't exist"))?
			.try_as::<T>()
	}

	/// Stores this value in virtual memory and returns a pointer to the location stored. Naturally, this consumes
	/// `self`, because virtual memory should own it's literal objects. To retrieve a reference of this object, use
	/// one of the methods on `VirtualMemory` with the returned pointer.
	///
	/// # Parameters
	/// - `context` - Global data about the current state of the compiler. In this case, it's used to access the compiler's
	/// virtual memory, which is stored on the context.
	///
	/// # Returns
	/// A pointer to the location of this literal object, which is now owned by the compiler's virtual memory.
	pub fn store_in_memory(self, context: &mut Context) -> VirtualPointer {
		context.virtual_memory.store(self)
	}

	pub fn declared_scope_id(&self) -> usize {
		self.scope_id
	}

	pub fn dependencies(&self) -> Vec<VirtualPointer> {
		self.fields.values().map(|pointer| pointer.to_owned()).collect()
	}

	pub fn fields(&self) -> impl Iterator<Item = (&Name, &VirtualPointer)> {
		self.fields.iter()
	}

	pub fn to_c_type(&self, context: &mut Context) -> anyhow::Result<String> {
		Ok(match self.type_name.unmangled_name().as_str() {
			"Object" => format!("type_{}_{}", self.name.to_c(context)?, self.address.unwrap()),
			_ => {
				format!("group_{}_{}", self.name.mangled_name(), self.address.unwrap())
			},
		})
	}

	pub fn is_type_assignable_to_type(&self, target_type: VirtualPointer, context: &mut Context) -> anyhow::Result<bool> {
		// Anything is assignable to Anything!
		if &target_type == context.scope_data.expect_global_variable("Anything").expect_as::<VirtualPointer>()? {
			return Ok(true);
		}

		Ok(self.address.unwrap() == target_type)
	}
}

impl TryAsRef<String> for LiteralObject {
	fn try_as_ref(&self) -> Option<&String> {
		self.get_internal_field("internal_value").ok()
	}
}

impl TryAsRef<f64> for LiteralObject {
	fn try_as_ref(&self) -> Option<&f64> {
		self.get_internal_field("internal_value").ok()
	}
}
impl TryAsRef<Vec<Expression>> for LiteralObject {
	fn try_as_ref(&self) -> Option<&Vec<Expression>> {
		self.get_internal_field("elements").ok()
	}
}

impl Typed for LiteralObject {
	fn get_type(&self, context: &mut Context) -> anyhow::Result<VirtualPointer> {
		let result = context
			.scope_data
			.get_variable_from_id(self.type_name.clone(), self.declared_scope_id())
			.unwrap()
			.expect_as::<VirtualPointer>()?
			.to_owned();

		Ok(result)
	}
}

pub trait LiteralConvertible: Sized {
	/// Attempts to serialize `self` into a literal object.
	///
	/// For example, consider a `FunctionDeclaration`. Function declarations are their own struct with their own
	/// type information (return type, parameters, etc.), but within the language, they're just objects like everything
	/// else. They interact just like objects, meaning you can access fields on them, pass them as values, etc. For this
	/// reason, it's often helpful to be able to convert a type-safe function declaration object into a generic Cabin
	/// object; For example, the compiler's virtual memory only stores `LiteralObjects`, so to store a function declaration
	/// in memory, it needs to be converted first.
	///
	/// The reverse of this method is `from_literal`, which exists for all types that implement this method, `to_literal`.
	/// This is used, for example, to retrieve a function declaration as a type-safe instance of `FunctionDeclaration` from
	/// the compiler's virtual memory. Together, these two functions allow storing and retrieving arbitrary types in virtual
	/// memory.
	///
	/// This function is generally called at the very end of compile-time evaluation in a type's implementation of
	/// `evaluate_at_compile_time` from the `CompileTime` trait. This is when literals should be stored in virtual memory,
	/// and such types should return a pointer to that location in virtual memory from their compile-time evaluation method.
	///
	/// # Parameters
	///
	/// - `context` - Global data about the current state of the compiler. For this function in particular, implementors may
	/// find use out of the context by being able to access the program's scopes, which is how `Name`s are resolved, among
	/// other things.
	///
	/// # Returns
	///
	/// The literal object that this was converted to, or an error if there was an error while attempting to convert this
	/// to a literal object. This could be, for example, that a value that should be a literal isn't; Such as the case of a
	/// user using an expression as a type when that expression can't be fully evaluated at compile-time.
	fn to_literal(self, context: &mut Context) -> anyhow::Result<LiteralObject>;

	/// Attempts to deserialize a literal object into `Self`.
	///
	/// For example, consider a `FunctionDeclaration`. Function declarations are their own struct with their own
	/// type information (return type, parameters, etc.), but they're serialized as literals with `to_literal` to be
	/// stored in virtual memory. Thus, when we want to retrieve information about the function declaration (such as
	/// when calling the function), we need to be able to deserialize the literal object back into a type-safe function
	/// declaration.
	///
	/// Note that this function takes a reference to a literal object, but returns an owned instance of `Self`. It may involve
	/// cloning. This is because literal objects are owned by virtual memory, and currently nothing can be moved out of virtual
	/// memory. Additionally, this function can't receive a mutable reference to a literal object, because any borrow of a literal
	/// object is indirectly a borrow of the compiler's `context`, and then `context` couldn't be passed to this function at all
	/// because only one mutable reference of it can exist at a time, which would be taken up by the borrow to the literal.
	///
	/// When this function is called depends on the specific type that's implementing it. For example, function declarations
	/// get deserialized during function calls, but group declarations get deserialized during object construction.
	///
	/// # Parameters
	///
	/// - `context` - Global data about the current state of the compiler. For this function in particular, implementors may
	/// find use out of the context by being able to access the program's scopes, which is how `Name`s are resolved, among
	/// other things.
	///
	/// # Returns
	///
	/// The instance of `Self` that the literal object was
	fn from_literal(literal: &LiteralObject, context: &Context) -> anyhow::Result<Self>;
}

impl TranspileToC for LiteralObject {
	fn to_c(&self, context: &mut Context) -> anyhow::Result<String> {
		Ok(match self.type_name.unmangled_name().as_str() {
			"Number" => {
				format!(
					"&({}) {{ .internal_value = {} }}",
					self.get_type(context)?.virtual_deref(context).clone().to_c_type(context)?,
					self.expect_as::<f64>()?.to_owned()
				)
			},
			"Text" => {
				format!(
					"&({}) {{ .internal_value = \"{}\" }}",
					self.get_type(context)?.virtual_deref(context).clone().to_c_type(context)?,
					self.expect_as::<String>()?.to_owned()
				)
			},
			_ => {
				// Type name
				let type_name = match self.type_name.unmangled_name().as_str() {
					"Object" => format!("type_{}_{}", self.name.to_c(context)?, self.address.unwrap()),
					_ => self.get_type(context)?.virtual_deref(context).clone().to_c_type(context)?,
				};

				// Create string builder
				let mut builder = format!("&({}) {{", type_name);

				// Anything fields
				if self.name != "Anything".into() {
					let anything = GroupDeclaration::from_literal(context.scope_data.expect_global_variable("Anything").expect_literal(context)?, context)?;
					for field in &anything.fields {
						builder += &format!("\n\t.{} = {},", field.name.to_c(context)?, field.value.as_ref().unwrap().to_c(context)?);
					}
				}

				// Add fields
				for (field_name, field_pointer) in &self.fields {
					builder += &format!("\n\t.{} = {},", field_name.to_c(context)?, field_pointer.to_c(context)?);
				}

				if self.type_name == "Function".into() {
					builder += &format!("\n\t.call = &call_anonymous_function_{}", self.address.unwrap());
				}

				// Finish building the string
				builder += "\n}";
				builder
			},
		})
	}
}
