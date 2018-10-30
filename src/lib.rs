#[macro_use]
extern crate error_chain;

extern crate regex;
extern crate ansi_term;

mod parser;
mod display;
mod custom_tags;

use std::fs::File;
use std::io::Read;

use ansi_term::Colour::Red;

use parser::{Todo, find_todos};
use display::{StyleConfig, print_file_todos};


mod errors {
	error_chain! {
		foreign_links {
			Error(::std::io::Error);
		}
	}
}

use errors::*;

pub fn print_error(err: &Error) {
	match err {
		_ => eprintln!("{}: {}", Red.paint("[todo_r error]"), err.to_string()),
	};
}


pub struct TodoRConfig {
	pub verbose: bool,
	pub todo_words: Vec<String>,
	styles: StyleConfig,
}

impl TodoRConfig {
	pub fn new<T: AsRef<str>>(todo_words: &[T]) -> TodoRConfig {
		let todo_word_strings: Vec<String> = todo_words.iter().map(|s| s.as_ref().to_string()).collect();

		TodoRConfig {
			verbose: false,
			todo_words: todo_word_strings,
			styles: StyleConfig::default(),
		}
	}

	pub fn set_no_style(&mut self) {
		self.styles = StyleConfig::no_style();
	}
}

struct TodoFile {
	filename: String,
	todos: Vec<Todo>,
}

impl TodoFile {
	fn new(filename: &str) -> TodoFile {
		TodoFile {
			filename: filename.to_string(),
			todos: Vec::with_capacity(0), // do not allocate because it will be replaced
		}
	}
}

/// Parser for finding TODOs in comments and storing them on a per-file basis.
pub struct TodoR {
	pub config: TodoRConfig,
	todo_files: Vec<TodoFile>,
}

impl TodoR {
	/// Creates new TodoR that looks for provided keywords.
	pub fn new<T: AsRef<str>>(todo_words: &[T]) -> TodoR {
		TodoR {
			config: TodoRConfig::new(todo_words),
			todo_files: Vec::new(),
		}
	}

	/// Opens file at given filename and process it by finding all its TODOs.
	pub fn open_todos(&mut self, filename: &str) -> Result<()> {

		let mut todo_file = TodoFile::new(filename);
		let file_ext = filename.rsplitn(2, '.').next().unwrap();
		let mut file = File::open(filename)?;

		// check the file is not a directory
		if file.metadata()?.is_dir() {
			return Err(format!("'{}' is a directory.", filename).into());
		}

		let mut file_contents = String::new();
		// TODO: Maybe use buffer in case file is very large
		file.read_to_string(&mut file_contents)?;
		todo_file.todos = find_todos(&file_contents, file_ext, &self.config.todo_words);

		self.todo_files.push(todo_file);
		Ok(())
	}

	/// Prints TODOs to stdout.
	pub fn print_todos(&self) {
		// TODO: implement
		unimplemented!();
	}
}

/// Searches file for TODOs
pub fn todo_r(filename: &str, config: &TodoRConfig) -> Result<()> {
	let file_ext: &str = filename.rsplitn(2, '.').next().unwrap();
	let mut file = File::open(filename)?;

	// check the file is not a directory
	if file.metadata()?.is_dir() {
		return Err(format!("'{}' is a directory.", filename).into());
	}

	let mut file_contents = String::new();
	// TODO: Maybe use buffer in case file is very large
	file.read_to_string(&mut file_contents)?;

	// TODO: store TODOs for other uses
	let todos: Vec<Todo> = find_todos(&file_contents, file_ext, &config.todo_words);

	print_file_todos(filename, &todos, &config.styles, config.verbose);
	Ok(())
}
