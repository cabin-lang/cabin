use std::{collections::HashMap, path::PathBuf};

use crate::{
	api::context::Context,
	cli::{
		commands::{start, CabinCommand},
		RunningContext,
	},
	comptime::{memory::VirtualPointer, CompileTime as _},
	lexer::{tokenize, tokenize_without_prelude, Span},
	parser::{
		expressions::{literal::LiteralObject, name::Name, object::ObjectType, Expression},
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
		let mut context = Context::new(&path)?;

		// Standard Library
		let mut stdlib_tokens = tokenize_without_prelude(STDLIB, &mut context)?;
		let stdlib_ast = parse(&mut stdlib_tokens, &mut context)?;
		let evaluated_stdlib = stdlib_ast.evaluate_at_compile_time(&mut context)?;
		let stdlib_module = evaluated_stdlib.into_literal(&mut context)?.store_in_memory(&mut context);
		context.scope_data.declare_new_variable("cabin", Expression::Pointer(stdlib_module))?;

		// User code
		start("Running", &context);

		// Project
		if let RunningContext::Project(project) = &context.running_context {
			let root = step!(get_source_code_directory(&project.root_directory().join("src")), context, "Reading", "source files");
			let tokenized = step!(tokenize_directory(root, &mut context), context, "Tokenizing", "source code");
			let module_ast = step!(parse_directory(tokenized, &mut context), context, "Parsing", "token streams");
			let compile_time_evaluated_module = step!(evaluate_directory(module_ast, &mut context), context, "Running", "compile-time code");
			let module_literal = literalize_directory(compile_time_evaluated_module, &mut context)?;
			let _global_module: VirtualPointer = composite_directory_into_module(module_literal, &mut context);
		}

		// let c_code = step!(transpile(&comptime_ast, &mut context), &context, "Transpiling", "evaluated AST to C");
		// std::fs::write("../output.c", &c_code)?;
		// let binary_location = step!(compile(&c_code), &context, "Compiling", "generated C code");
		// step!(run_native_executable(binary_location), &context, "Running", "compiled executable");

		Ok(())
	}
}

struct CabinDirectory<T> {
	source_files: HashMap<String, T>,
	sub_directories: HashMap<String, CabinDirectory<T>>,
}

fn get_source_code_directory(root_dir: &PathBuf) -> anyhow::Result<CabinDirectory<String>> {
	let mut source_files = HashMap::new();
	let mut sub_directories = HashMap::new();
	for entry in std::fs::read_dir(root_dir).unwrap().filter_map(Result::ok) {
		if entry.path().is_file() && entry.path().extension().unwrap() == "cabin" {
			source_files.insert(
				entry.path().file_name().unwrap().to_str().unwrap().to_owned().strip_suffix(".cabin").unwrap().to_owned(),
				std::fs::read_to_string(entry.path())?,
			);
		} else if entry.path().is_dir() {
			sub_directories.insert(entry.path().file_name().unwrap().to_str().unwrap().to_owned(), get_source_code_directory(&entry.path())?);
		}
	}

	Ok(CabinDirectory { source_files, sub_directories })
}

fn composite_directory_into_module(directory: CabinDirectory<LiteralObject>, context: &mut Context) -> VirtualPointer {
	let mut fields = HashMap::new();

	let mut submodules = HashMap::new();

	for (sub_directory_name, sub_directory_module) in directory.sub_directories {
		let composite = composite_directory_into_module(sub_directory_module, context);
		submodules.insert(Name::from(sub_directory_name.clone()), composite);
		fields.insert(Name::from(sub_directory_name), composite);
	}

	for (file_name, mut file_module) in directory.source_files {
		for (sub_module_name, sub_module) in &submodules {
			file_module.fields.insert(sub_module_name.clone(), *sub_module);
		}
		fields.insert(Name::from(file_name), file_module.store_in_memory(context));
	}

	LiteralObject {
		type_name: "Object".into(),
		fields,
		internal_fields: HashMap::new(),
		object_type: ObjectType::Normal,
		outer_scope_id: context.scope_data.unique_id(),
		inner_scope_id: None,
		name: "anonymous_directory_module".into(),
		address: None,
		span: Span::unknown(),
		tags: TagList::default(),
	}
	.store_in_memory(context)
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
			fn $name(directory: CabinDirectory<$input_type>, context: &mut Context) -> anyhow::Result<CabinDirectory<$output_type>> {
				Ok(CabinDirectory {
					source_files: directory
						.source_files
						.into_iter()
						.map(|(file_name, contents)| Ok((file_name.clone(), $mapping_function(file_name, contents, context)?)))
						.collect::<anyhow::Result<HashMap<_, _>>>()?,
					sub_directories: directory
						.sub_directories
						.into_iter()
						.map(|(name, sub_directory)| Ok((name, $name(sub_directory, context)?)))
						.collect::<anyhow::Result<HashMap<_, _>>>()?,
				})
			}
		)*
	};
}

directory_actions! {
	tokenize_directory(|_file_name, source_code: String, context| tokenize(&source_code, context)): String => TokenQueue;
	parse_directory(|_file_name, mut tokens, context| parse(&mut tokens, context)): TokenQueue => Module;
	evaluate_directory(|_file_name, ast: Module, context| ast.evaluate_at_compile_time(context)): Module => Module;
	literalize_directory(|_file_name, ast: Module, context| ast.into_literal(context)): Module => LiteralObject;
}
