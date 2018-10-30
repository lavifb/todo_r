// Binary for finding TODOs in specified files
extern crate todo_r;

#[macro_use(clap_app)]
extern crate clap;

use std::process::Command;
use todo_r::{TodoRConfig, todo_r, print_error};


/// Processor for parsing command line arguments
fn main() {
	// TODO: add config file option
	let matches = clap_app!(todo_r =>
		(version: env!("CARGO_PKG_VERSION"))
		(author: "Lavi Blumberg <lavifb@gmail.com>")
		(about: "Lists TODO comments in code")
		(@arg FILE: ... "File to search for TODO items.")
		(@arg NOSTYLE: -s --("no-style") "Prints output with no ansi colors or styles.")
		(@arg TAG: -t --("tag") +takes_value +multiple "Todo tags to search for.")
		(@arg VERBOSE: -v --("verbose") "Provide verbose output.")
	).get_matches();


	let todo_words = match matches.values_of("TAG") {
		Some(words_iter) => words_iter.collect(),
		None => vec!["todo", "fixme"],
	};

	let verbose: bool = matches.is_present("VERBOSE");
	if verbose { println!("TODO keywords: {}", todo_words.join(", ").to_uppercase()); }

	let mut config: TodoRConfig = TodoRConfig::new(&todo_words);
	config.no_style = matches.is_present("NOSTYLE");
	config.verbose = verbose;


	match matches.values_of("FILE") { 
		Some(files) => {
			iter_todo_r(files, &config);
		},
		None => {
			// try to use git using `git ls-files $(git rev-parse --show-toplevel)`
			let rev_parse = Command::new("git")
			                        .arg("rev-parse")
			                        .arg("--show-toplevel")
			                        .output()
			                        .unwrap();

			let top_level: String = String::from_utf8_lossy(&rev_parse.stdout).trim().to_string();
			if verbose { println!("Searching git repo at {}", top_level); }

			let output = Command::new("git")
			                        .arg("ls-files")
			                        .arg(top_level)
			                        .output()
			                        .unwrap();

			let files = String::from_utf8_lossy(&output.stdout);
			iter_todo_r(files.lines(), &config);
		},
	}
}

fn iter_todo_r<'a, I>(files: I, config: &'a TodoRConfig) 
where
	I: Iterator<Item = &'a str>,
{
	for file in files {
		todo_r(file, config).unwrap_or_else(|err| print_error(&err));
	}
}
