// Module for finding TODOs in files

use regex::Regex;

/// A struct holding the TODO and all the needed meta-information for it.
pub struct Todo {
	line: usize,
	todo_type: String,
	content: String,
}

/// Creates a list of TODOs found in content
// TODO: return list of TODOs
pub fn find_todos(content: &str) {
	
	// TODO: add custom TODO keywords
	// TODO: record line numbers
	// TODO: use a better regex to find TODOs
	// TODO: add more regexs for other languages/patterns
	let re = Regex::new(r"(?m)^\s*// (TODO:.*)$").unwrap();

	// TODO: store TODOs in Todo struct
	for todo in re.captures_iter(content) {
		println!("{}", &todo[1].trim());
	}
}