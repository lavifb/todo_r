// Module for finding TODOs in files

use regex::Regex;
use std::fmt;

/// A struct holding the TODO and all the needed meta-information for it.
pub struct Todo {
	line: usize,
	todo_type: String,
	content: String,
}

impl Todo {
	fn new(line: usize, todo_type_str: &str, content_str: &str) -> Todo {
		
		let todo_type = todo_type_str.to_string();
		let content = content_str.to_string();

		Todo {
			line,
			todo_type,
			content,
		}
	}
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\t{}\t{}", self.line, self.todo_type, self.content)
    }
}

static TODO_REGEX: &str = r"^\s*//\s*TODO:(.*)$";

/// Creates a list of TODOs found in content
// TODO: return list of TODOs
pub fn find_todos(content: &str) -> Vec<Todo> {
	
	// TODO: add custom TODO keywords
	// TODO: use a better regex to find TODOs
	// TODO: add more regexs for other languages/patterns
	let re = Regex::new(TODO_REGEX).unwrap();

	let mut todos = Vec::new();
	let mut line_num: usize = 0;

	for line in content.lines() {
		line_num += 1;

		for todo_content in re.captures_iter(line) {
			let todo = Todo::new(line_num, "TODO", todo_content[1].trim());

			todos.push(todo)
		}
	}

	todos
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn regex_whitespace() {
		let content = "\t\t\t\t//  TODO:  i3 \t";

		let re = Regex::new(TODO_REGEX).unwrap();
		let cap = re.captures(content).unwrap();

		let output = cap[1].trim();

		assert_eq!("i3", output);
	}
}