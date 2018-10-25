// Binary for finding TODOs in specified files
extern crate todo_r;

#[macro_use(clap_app)]
extern crate clap;

use std::process::Command;
use todo_r::{TodoRConfig, todo_r, print_error};


/// Processor for parsing command line arguments
fn main() {
	// TODO: add more cli options
	let matches = clap_app!(todo_r =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: "Lavi Blumberg <lavifb@gmail.com>")
        (about: "Lists TODO comments in code")
        (@arg FILE: ... "File to search for TODO items.")
        (@arg NOSTYLE: -s --("no-style") "Prints output with no ansi colors or styles.")
        (@arg TAG: -t --("tag") +takes_value +multiple "Todo tags to search for.")
    ).get_matches();

	let no_style = matches.is_present("NOSTYLE");
	// TODO: check that tags dont have spaces or punctuation
	let todo_words = match matches.values_of("TAG") {
		Some(words_iter) => words_iter.collect(),
		None => Vec::new(),
	};

	let config:TodoRConfig = TodoRConfig::new(
		no_style,
		&todo_words,
	);

	// TODO: make this better somehow
	match matches.values_of("FILE") { 
		Some(files) => {
			for file in files {
				todo_r(file, &config).unwrap_or_else(|err| print_error(&err));
			}
		},
		None => {
			// try to use git using `git ls-files $(git rev-parse --show-toplevel)`
			let rev_parse = Command::new("git")
									.arg("rev-parse")
									.arg("--show-toplevel")
									.output()
									.unwrap();

			let top_level: String = String::from_utf8_lossy(&rev_parse.stdout).into_owned().trim().to_string();

			let output = Command::new("git")
									.arg("ls-files")
									.arg(top_level)
									.output()
									.unwrap();

			let files = String::from_utf8_lossy(&output.stdout).into_owned();
			for file in files.lines() {
				todo_r(file, &config).unwrap_or_else(|err| print_error(&err));
			}
		},
	}
}
