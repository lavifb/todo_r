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
	let re = Regex::new(r"^\s*// TODO:(.*)$").unwrap();

	let mut line_num: usize = 0;

	for line in content.lines() {

		line_num += 1;

		// TODO: store TODOs in Todo struct
		for todo in re.captures_iter(line) {
			println!("{}\tTODO\t{}", line_num, &todo[1].trim());
		}
	}
}