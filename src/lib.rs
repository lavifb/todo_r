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
use display::{print_file_todos};

mod errors {
	error_chain! {
		foreign_links {
			Error(::std::io::Error);
		}
	}
}

use errors::*;

/// Searches file for TODOs
// TODO: add config struct for configurations like colors
pub fn todo_r(filename: &str) -> Result<()> {
	// TODO: look at file extension to figure out how to parse
	let mut file = File::open(filename)?;

	// check the file is not a directory
	if file.metadata()?.is_dir() {
        return Err(format!("'{}' is a directory.", filename).into());
    }

	let mut file_contents = String::new();
	// TODO: Maybe use buffer in case file is very large
	file.read_to_string(&mut file_contents)?;

	// TODO: store TODOs for other uses
	let todos: Vec<Todo> = find_todos(&file_contents);
	print_file_todos(filename, todos);
	Ok(())
}

pub fn print_error(err: &Error) {
	match err {
		_ => eprintln!("{}: {}", Red.paint("[todo_r error]"), err.to_string()),
	};
}
