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


pub struct TodoRConfig<'a> {
	no_style: bool,
	verbose: bool,
	todo_words: Vec<&'a str>,
}

impl<'a> TodoRConfig<'a> {
	pub fn new(todo_words: &[&'a str]) -> TodoRConfig<'a> {
		TodoRConfig {
			no_style: false,
			verbose: false,
			todo_words: todo_words.to_vec(),
		}
	}

	pub fn set_verbose(&mut self) {
		self.verbose = true;
	}

	pub fn set_no_style(&mut self) {
		self.no_style = true;
	}
}

/// Searches file for TODOs
pub fn todo_r(filename: &str, config: &TodoRConfig) -> Result<()> {
	let file_ext = filename.rsplitn(2, '.').next().unwrap();
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
