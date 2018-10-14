extern crate regex;

use std::env;
use std::fs;
use regex::Regex;

fn main() {

	let mut files = Vec::new();

    // For now we will just open files given in args
    // TODO: get list of tracked files
    for arg in env::args().skip(1) {
	    println!("{}\n", arg);
	    files.push(arg)
	}

    // open each file and look for TODO comments
	for file in files.iter() {
		// TODO: look at file extension to figure out how to parse
		let file_contents = fs::read_to_string(file).unwrap();

		// TODO: add custom TODO keywords
		// TODO: record line numbers
		// TODO: use a better regex to find TODOs
		// TODO: add more regexs for other languages/patterns
		let re = Regex::new(r"(?m)^(\s*)// TODO:(.*)$").unwrap();

		// TODO: store TODOs in buffer before printing for other uses
		for todo in re.captures_iter(&file_contents) {
			println!("{}", &todo[0].trim());
		}
	}
}
