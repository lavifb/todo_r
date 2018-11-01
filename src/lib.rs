#[macro_use]
extern crate error_chain;

extern crate regex;
extern crate ansi_term;

mod parser;
mod display;
mod custom_tags;

pub mod errors {
	error_chain! {
		foreign_links {
			Error(::std::io::Error);
		}
	}

	use ansi_term::Colour::Red;

	pub fn eprint_error(err: &Error) {
		match err {
			_ => eprintln!("{}: {}", Red.paint("[todo_r error]"), err.to_string()),
		};
	}
}

use std::fs::File;
use std::io::Read;

use errors::*;
use parser::parse_content;
use display::{StyleConfig, print_file_todos, TodoFile};


/// Configuration for `TodoR`.
///
/// `verbose` holds whether to print extra content.
/// `todo_words` gives a list of the TODO terms to search for.
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
		todo_file.set_todos(parse_content(&file_contents, file_ext, &self.config.todo_words));

		self.todo_files.push(todo_file);
		Ok(())
	}

	/// Parses provided content by finding all its TODOs.
	pub fn parse_todos(content: &str, file_ext: &str) {
		// TODO: implement
		unimplemented!();
	}

	/// Finds TODO comments in the given content
	#[allow(unused)]
	pub fn find_todos(content: &str) {
		unimplemented!();
	}

	/// Prints TODOs to stdout.
	// TODO: make method that writes to anything instead of only printing
	pub fn print_todos(&self) {
		for todo_file in &self.todo_files {
			print_file_todos(&todo_file, &self.config.styles, self.config.verbose);
		}
	}
}
