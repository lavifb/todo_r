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
	no_style: bool,
	verbose: bool,
	todo_words: Vec<String>,
}

impl TodoRConfig {
	pub fn new<T: AsRef<str>>(todo_words: &[T]) -> TodoRConfig {
		let todo_word_strings: Vec<String> = todo_words.iter().map(|s| s.as_ref().to_string()).collect();

		TodoRConfig {
			no_style: false,
			verbose: false,
			todo_words: todo_word_strings,
		}
	}

	pub fn set_verbose(&mut self) {
		self.verbose = true;
	}

	pub fn set_no_style(&mut self) {
		self.no_style = true;
	}
}

struct TodoFile {
	filename: String,
	todos: Vec<Todo>,
}

/// TODO finder that stores all of the found TODOs on a per-file basis.
pub struct TodoR {
	pub config: TodoRConfig,
	todo_files: Vec<TodoFile>,
}

impl TodoR {
	/// Creates new TodoR struct with provided configuration.
	pub fn new<T: AsRef<str>>(todo_words: &[T]) -> TodoR {
		TodoR {
			config: TodoRConfig::new(todo_words),
			todo_files: Vec::new(),
		}
	}

	/// Opens file at given filename and process it by finding all its TODOs.
	pub fn open_todos(&mut self, filename: &str) -> Result<()> {
		// TODO: implement
		unimplemented!();


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

	let styles = match config.no_style {
		true => StyleConfig::no_style(),
		false => StyleConfig::default(),
	};

	print_file_todos(filename, &todos, &styles, config.verbose);
	Ok(())
}
