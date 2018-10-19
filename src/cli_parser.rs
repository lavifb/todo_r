// Module for processing command line arguments

// TODO: store configuration in Config struct
// TODO: change crate to lib with bins

/// Processor for parsing command line arguments
pub fn parse_args() -> Vec<String> {
	// TODO: add more cli options
	let matches = clap_app!(todo_r =>
        (version: "1.0")
        (author: "Lavi Blumberg <lavifb@gmail.com>")
        (about: "Lists TODO comments in code")
        (@arg FILE: ... +required "File to search for TODO items.")
    ).get_matches();

	let files: Vec<&str> = matches.values_of("FILE").unwrap().collect();

	let file_strings = files.iter().map(|s| s.to_string()).collect();

	file_strings
}
