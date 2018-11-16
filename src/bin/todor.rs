// Binary for finding TODOs in specified files
extern crate todo_r;

#[macro_use(clap_app)]
extern crate clap;

use std::path::Path;
use std::process::Command;
use todo_r::TodoR;
use todo_r::errors::eprint_error;


/// Parses command line arguments and use TodoR to find TODO comments.
fn main() {
	// TODO: add config file option
	// TODO: add subcommand for just content so it can be piped
	let matches = clap_app!(todo_r =>
		(version: env!("CARGO_PKG_VERSION"))
		(author: "Lavi Blumberg <lavifb@gmail.com>")
		(about: "Lists TODO comments in code")
		(@arg FILE: ... "File to search for TODO items.")
		(@arg NOSTYLE: -s --("no-style") "Prints output with no ansi colors or styles.")
		(@arg TAG: -t --("tag") +takes_value +multiple "Todo tags to search for.")
		(@arg VERBOSE: -v --("verbose") "Provide verbose output.")
		(@subcommand remove =>
			(version: "0.0.1")
			(about: "Removes TODO comments from the code")
			(author: "Lavi Blumberg <lavifb@gmail.com>")
			(@arg FILE: +required ... "File to remove TODO items from.")
			(@arg LINE: -l +takes_value +required "Index of TODO to remove.")
		)
	).get_matches();


	let todo_words = match matches.values_of("TAG") {
		Some(words_iter) => words_iter.collect(),
		None => vec!["todo", "fixme"],
	};

	let verbose: bool = matches.is_present("VERBOSE");
	if verbose { println!("TODO keywords: {}", todo_words.join(", ").to_uppercase()); }

	let mut todor = TodoR::new(&todo_words);
	if matches.is_present("NOSTYLE") {
		todor.config.set_no_style();
	}
	todor.config.verbose = verbose;

	match matches.values_of("FILE") { 
		Some(files) => {
			for file in files {
				todor.open_todos(Path::new(file)).unwrap_or_else(|err| eprint_error(&err));
			}
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

			let files_in_lines = String::from_utf8_lossy(&output.stdout);
			for file in files_in_lines.lines() {
				todor.open_todos(Path::new(file)).unwrap_or_else(|err| eprint_error(&err));
			}
		},
	}

	todor.print_todos();

	// handle remove subcommand
	if let Some(matches) = matches.subcommand_matches("remove") {
		let line: usize = matches.value_of("LINE").unwrap().parse().unwrap();
		let file = matches.value_of("FILE").unwrap();

		println!("\n Removing TODO comment {} in {}\n", line, file);

		todor.remove_todo_line(Path::new(file), line).unwrap_or_else(|err| eprint_error(&err));

		todor.print_todos();
	}
}