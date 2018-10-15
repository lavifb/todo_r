extern crate regex;

mod cli_parser;
mod parser;

use std::env;
use std::fs;

use cli_parser::parse_args;
use parser::find_todos;

fn main() {
    // TODO: get list of tracked files from git
    // For now we will just open files given in args
	let mut files = parse_args(env::args());

    // open each file and look for TODO comments
	for file in files.iter() {
		// TODO: look at file extension to figure out how to parse
		let file_contents = fs::read_to_string(file).unwrap();

		// TODO: store TODOs for other uses
		find_todos(&file_contents);
	}
}
