use std::{collections::HashMap, path::PathBuf};

use crate::{
	api::{context::context, scope::ScopeType},
	cli::{
		commands::{start, CabinCommand},
		RunningContext,
	},
	comptime::CompileTime as _,
	debug_log, debug_start,
	lexer::{tokenize, tokenize_main, tokenize_without_prelude, Span},
	mapped_err,
	parser::{
		expressions::{
			field_access::FieldAccessType,
			function_call::FunctionCall,
			name::Name,
			object::{Field, ObjectConstructor},
			Expression,
		},
		parse,
		statements::tag::TagList,
		Module, TokenQueue,
	},
	step, STDLIB,
};

/// Run a cabin file or project without outputting any permanent files.
#[derive(clap::Parser)]
pub struct RunCommand {
	path: Option<String>,
}

impl CabinCommand for RunCommand {
	fn execute(self) -> anyhow::Result<()> {
		let path = self.path.map(PathBuf::from).unwrap_or_else(|| std::env::current_dir().unwrap());
		context().running_context = RunningContext::try_from(path)?;

		// Standard Library
		{
			let _dropper = debug_start!("{} stdlib module...", "Adding".bold().green());
			let mut stdlib_tokens = tokenize_without_prelude(STDLIB)?;
			let stdlib_ast = parse(&mut stdlib_tokens)?;
			let evaluated_stdlib = stdlib_ast.evaluate_at_compile_time()?;
			let stdlib_module = evaluated_stdlib.into_literal()?.store_in_memory();
			context().scope_data.declare_new_variable("cabin", Expression::Pointer(stdlib_module))?;
		}

		// User code
		start("Running");

		// Project
		if let RunningContext::Project(project) = &context().running_context {
			let _dropper = debug_start!("\n{} project...", "Running".bold().green());
			let root = step!(get_source_code_directory(&project.root_directory().join("src")), "Reading", "source files");
			let tokenized = step!(tokenize_directory(root), "Tokenizing", "source code");
			let module_ast = step!(parse_directory(tokenized), "Parsing", "token streams");
			let root_module = add_modules_to_scope(module_ast).map_err(mapped_err! {
				while = "adding the program's modules into scope",
			})?;
			let Expression::ObjectConstructor(compile_time_evaluated_root_module) = step!(
				root_module.evaluate_at_compile_time().map_err(mapped_err! {
					while = "evaluating the project's root module at compile-time",
				}),
				"Running",
				"compile-time code"
			) else {
				unreachable!();
			};
			let Expression::ObjectConstructor(main_module) = context()
				.scope_data
				.get_variable_from_id("main", compile_time_evaluated_root_module.inner_scope_id)
				.unwrap()
			else {
				unreachable!()
			};
			let main_function = main_module.get_field("main_function").unwrap().try_clone_pointer().unwrap();
			debug_log!("{} main function...", "Calling".bold().green());
			FunctionCall::call_main(main_function, compile_time_evaluated_root_module.inner_scope_id)?;
		}

		// let c_code = step!(transpile(&comptime_ast, &mut context), &context, "Transpiling", "evaluated AST to C");
		// std::fs::write("../output.c", &c_code)?;
		// let binary_location = step!(compile(&c_code), &context, "Compiling", "generated C code");
		// step!(run_native_executable(binary_location), &context, "Running", "compiled executable");

		Ok(())
	}
}

#[derive(Debug)]
struct CabinDirectory<T> {
	source_files: HashMap<String, T>,
	sub_directories: HashMap<String, CabinDirectory<T>>,
}

fn get_source_code_directory(root_dir: &PathBuf) -> anyhow::Result<CabinDirectory<String>> {
	let mut source_files = HashMap::new();
	let mut sub_directories = HashMap::new();
	for entry in std::fs::read_dir(root_dir).unwrap().filter_map(Result::ok) {
		if entry.path().is_file() && entry.path().extension().unwrap() == "cabin" {
			let name = entry.path().file_name().unwrap().to_str().unwrap().to_owned().strip_suffix(".cabin").unwrap().to_owned();
			source_files.insert(name, std::fs::read_to_string(entry.path())?);
		} else if entry.path().is_dir() {
			let name = entry.path().file_name().unwrap().to_str().unwrap().to_owned().strip_suffix(".cabin").unwrap().to_owned();
			sub_directories.insert(name.clone(), get_source_code_directory(&entry.path())?);
		}
	}

	Ok(CabinDirectory { source_files, sub_directories })
}

fn add_modules_to_scope(directory: CabinDirectory<Module>) -> anyhow::Result<ObjectConstructor> {
	let mut fields = Vec::new();

	context().scope_data.enter_new_unlabeled_scope(ScopeType::File);
	let inner_scope_id = context().scope_data.unique_id();
	for (file_name, file_module) in directory.source_files {
		let value = Expression::ObjectConstructor(file_module.into_object().unwrap());
		context().scope_data.declare_new_variable(file_name.clone(), value)?;
		fields.push(Field {
			name: file_name.clone().into(),
			field_type: None,
			value: Some(Expression::Name(Name::from(file_name))),
		})
	}
	context().scope_data.exit_scope()?;

	let constructor = ObjectConstructor {
		type_name: "Module".into(),
		internal_fields: HashMap::new(),
		fields,
		span: Span::unknown(),
		inner_scope_id,
		outer_scope_id: context().scope_data.unique_id(),
		field_access_type: FieldAccessType::Normal,
		name: "root_module".into(),
		tags: TagList::default(),
	};

	Ok(constructor)
}

macro_rules! directory_actions {
	(
		$(
			$(#[$annotations: meta])?
			$name: ident($mapping_function: expr): $input_type: ty => $output_type: ty;
		)*
	) => {
		$(
			$(#[$annotations])?
			fn $name(directory: CabinDirectory<$input_type>) -> anyhow::Result<CabinDirectory<$output_type>> {
				Ok(CabinDirectory {
					source_files: directory
						.source_files
						.into_iter()
						.map(|(file_name, contents)| Ok((file_name.clone(), $mapping_function(file_name, contents)?)))
						.collect::<anyhow::Result<HashMap<_, _>>>()?,
					sub_directories: directory
						.sub_directories
						.into_iter()
						.map(|(name, sub_directory)| Ok((name, $name(sub_directory)?)))
						.collect::<anyhow::Result<HashMap<_, _>>>()?,
				})
			}
		)*
	};
}

directory_actions! {
	parse_directory(|_file_name, mut tokens| parse(&mut tokens)): TokenQueue => Module;
}

fn tokenize_directory(directory: CabinDirectory<String>) -> anyhow::Result<CabinDirectory<TokenQueue>> {
	Ok(CabinDirectory {
		source_files: directory
			.source_files
			.into_iter()
			.map(|(file_name, contents)| Ok((file_name.clone(), if file_name == "main" { tokenize_main(&contents)? } else { tokenize(&contents)? })))
			.collect::<anyhow::Result<HashMap<_, _>>>()?,
		sub_directories: directory
			.sub_directories
			.into_iter()
			.map(|(name, sub_directory)| Ok((name, tokenize_directory(sub_directory)?)))
			.collect::<anyhow::Result<HashMap<_, _>>>()?,
	})
}
