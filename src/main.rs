#[macro_use]
extern crate clap;
extern crate regex;
extern crate ansi_term;

mod cli_parser;
mod parser;
mod display;
mod custom_tags;

use std::fs;

use cli_parser::parse_args;
use parser::{Todo, find_todos};
use display::{print_file_todos};

fn main() {
    // TODO: get list of tracked files from git
    // For now we will just open files given in args
	let files = parse_args();

    // open each file and look for TODO comments
	for file in files.iter() {
		// TODO: look at file extension to figure out how to parse
		// TODO: handle error when filename is not found
		let file_contents = fs::read_to_string(file).unwrap();

		// TODO: store TODOs for other uses
		let todos: Vec<Todo> = find_todos(&file_contents);
		print_file_todos(file, todos);
	}
}
