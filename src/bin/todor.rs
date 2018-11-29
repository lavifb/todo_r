// Binary for finding TODOs in specified files
extern crate todo_r;

#[macro_use(clap_app)] extern crate clap;
extern crate dialoguer;
extern crate ansi_term;

use std::path::Path;
use std::process::Command;
use dialoguer::Select;
use ansi_term::Color::Red;

use todo_r::{TodoR, TodoRBuilder};
use todo_r::errors::eprint_error;


/// Parses command line arguments and use TodoR to find TODO comments.
fn main() {
	// TODO: add subcommand for just content so it can be piped
	let matches = clap_app!(todo_r =>
		(version: env!("CARGO_PKG_VERSION"))
		(author: "Lavi Blumberg <lavifb@gmail.com>")
		(about: "Lists TODO comments in code")
		(@arg FILE: ... "File to search for TODO items.")
		// TODO: option to make config file
		(@arg CONFIG: -c --("config") +takes_value "Takes configuration from file.")
		(@arg NOSTYLE: -s --("no-style") "Prints output with no ansi colors or styles.")
		(@arg TAGS: -t --("tag") +takes_value +multiple "TODO tags to search for.")
		(@arg IGNORE: -i --("ignore") +takes_value +multiple "Paths to be ignored.")
		(@arg OVERRIDE_TAGS: -T --("override-tags") +takes_value +multiple "Overrides default TODO tags to only search custom ones.")
		(@arg VERBOSE: -v --("verbose") "Provide verbose output.")
		(@arg DELETE_MODE: -d --("delete") "Interactive delete mode.")
		(@subcommand remove =>
			(version: "0.1")
			(about: "Removes TODO comments from the code")
			(author: "Lavi Blumberg <lavifb@gmail.com>")
			(@arg FILE: +required ... "File to remove TODO items from.")
			(@arg LINE: -l +takes_value +required "Index of TODO to remove.")
		)
	).get_matches();


	let mut builder = TodoRBuilder::new();

	if let Some(config_path) = matches.value_of("CONFIG") {
		// TODO: handle error
		builder.add_config_file(Path::new(config_path)).unwrap();
	};

	if let Some(tags_iter) = matches.values_of("TAGS") {
		builder.add_tags(tags_iter);
	}

	if let Some(tags_iter) = matches.values_of("OVERRIDE_TAGS") {
		builder.add_override_tags(tags_iter);
	}

	let verbose: bool = matches.is_present("VERBOSE");
	if verbose { 
		builder.set_verbose(verbose);
	}

	if matches.is_present("NOSTYLE") {
		builder.set_no_style();
	}

	if let Some(ignore_paths_iter) = matches.values_of("IGNORE") {
		// TODO: handle error
		builder.add_override_ignore_paths(ignore_paths_iter).unwrap();
	}

	let mut todor = builder.build().unwrap();
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
			                        // TODO: handle error
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

	if matches.is_present("DELETE_MODE") {
		loop {
			let file_selection = match select_file(&todor) {
				Some(file_selection) => file_selection,
				None => return,
			};
			
			let filepath = Path::new(&file_selection);
			let selected_todo = select_todo(&todor, filepath);

			let todo_ind = match selected_todo {
				Some(todo_ind) => todo_ind,
				None => continue,
			};

			todor.remove_todo(filepath, todo_ind).unwrap_or_else(|err| eprint_error(&err));
			println!("Comment removed");
		}
	} else {
		todor.print_todos();
	}

	// handle remove subcommand
	if let Some(matches) = matches.subcommand_matches("remove") {
		let line: usize = matches.value_of("LINE").unwrap().parse().unwrap();
		let file = matches.value_of("FILE").unwrap();

		println!("\n Removing TODO comment {} in {}\n", line, file);

		todor.remove_todo_line(Path::new(file), line).unwrap_or_else(|err| eprint_error(&err));

		todor.print_todos();
	}
}

fn select_file(todor: &TodoR) -> Option<String> {
	let option_quit = format!("{}", Red.paint("QUIT"));
	let mut tracked_files = todor.get_tracked_files();
	tracked_files.push(&option_quit);
	// IMPR: Cache tracked_files for when you go back

	let mut file_selector = Select::new();
	file_selector.with_prompt("Pick a file to delete comment")
	             .items(&tracked_files)
	             .default(0);

	let file_ind = file_selector.interact().unwrap();
	if file_ind + 1 == tracked_files.len() {
		return None;
	}

	Some(tracked_files[file_ind].to_string())
}

fn select_todo(todor: &TodoR, filepath : &Path) -> Option<usize> {
	let mut todos_buf: Vec<u8> = Vec::new();
	todor.write_todos_from_file(filepath, &mut todos_buf);

	let todos_string = String::from_utf8_lossy(&todos_buf);
	let mut todos_lines = todos_string.lines();
	let styled_filename = todos_lines.next().unwrap();

	let option_back = format!("{}", Red.paint("BACK"));
	let mut todos_items: Vec<&str> = todos_lines.collect();
	todos_items.push(&option_back);

	let mut todo_selector = Select::new();
	todo_selector.with_prompt(styled_filename)
	             .items(&todos_items)
	             .default(0);

	let todo_ind = todo_selector.interact().unwrap();
	if todo_ind + 1 == todos_items.len() {
		return None;
	}

	Some(todo_ind)
}