extern crate regex;
extern crate ansi_term;

mod parser;
mod display;
mod custom_tags;

use std::fs;

use parser::{Todo, find_todos};
use display::{print_file_todos};

/// Searches file for TODOs
pub fn todo_r(filename: &str) -> Result<(), String> {
	// TODO: look at file extension to figure out how to parse
	// TODO: handle error when filename is not found
	let file_contents = fs::read_to_string(filename);

	match file_contents {
		Ok(file_contents) => {
			// TODO: store TODOs for other uses
			let todos: Vec<Todo> = find_todos(&file_contents);
			print_file_todos(filename, todos);
			Ok(())
		},
		Err(error) => {
			println!("{} {:?} {}", filename, error.kind(), error);
			Err(error.to_string())
		},
	}
}
