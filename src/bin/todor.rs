// Binary for finding TODOs in specified files
extern crate todo_r;

#[macro_use(clap_app)]
extern crate clap;

use todo_r::todo_r;

/// Processor for parsing command line arguments
fn main() {
	// TODO: add more cli options
	// TODO: get list of tracked files from git
	let matches = clap_app!(todo_r =>
        (version: "1.0")
        (author: "Lavi Blumberg <lavifb@gmail.com>")
        (about: "Lists TODO comments in code")
        (@arg FILE: ... +required "File to search for TODO items.")
    ).get_matches();

	let files = matches.values_of("FILE").unwrap();

	for file in files {
		todo_r(file);
	}
}
