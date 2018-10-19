extern crate regex;
extern crate ansi_term;

mod parser;
mod display;
mod custom_tags;

use std::fs;

use parser::{Todo, find_todos};
use display::{print_file_todos};

/// Searches file for TODOs
pub fn todo_r(filename: &str) {
	// TODO: look at file extension to figure out how to parse
	// TODO: handle error when filename is not found
	let file_contents = fs::read_to_string(filename).unwrap();

	// TODO: store TODOs for other uses
	let todos: Vec<Todo> = find_todos(&file_contents);
	print_file_todos(filename, todos);
}
