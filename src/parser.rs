// Module for finding TODOs in files

use regex::Regex;
use std::fmt;
use ansi_term::Colour;

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

	pub fn color_print(&self, line_color: &Colour, todo_color: &Colour, content_color: &Colour) {
		println!("  {}{}\t{}\t{}", 
			line_color.paint("line "), line_color.paint(self.line.to_string()),
			todo_color.paint(&self.todo_type),
			content_color.paint(&self.content),
			);
	}
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {}\t{}\t{}", self.line, self.todo_type, self.content)
    }
}

// TODO: add custom TODO keywords
// TODO: add more regexs for other languages/patterns
// TODO: use a better regex to find TODOs
static TODO_REGEX: &str = r"^\s*//\s*TODO:(.*)$";

/// Creates a list of TODOs found in content
// TODO: Maybe return iterator instead of Vec 
pub fn find_todos(content: &str) -> Vec<Todo> {
	
	let re = Regex::new(TODO_REGEX).unwrap();
	let mut todos = Vec::new();

	for (line_num, line) in content.lines().enumerate() {
		let todo_content = re.captures(line);
		match todo_content {
			Some(todo_content) => {
				let todo = Todo::new(line_num+1, "TODO", todo_content[1].trim());
				todos.push(todo);
			},
			None => {},
		};
	}

	todos
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn regex_whitespace() {
		let content = "\t\t\t\t  //  TODO:  item \t";

		let re = Regex::new(TODO_REGEX).unwrap();
		let cap = re.captures(content).unwrap();

		let output = cap[1].trim();

		assert_eq!("item", output);
	}

	#[test]
	fn regex_todo_in_comment() {
		let content = "//  TODO:  item // TODO: item \t";

		let re = Regex::new(TODO_REGEX).unwrap();
		let cap = re.captures(content).unwrap();

		let output = cap[1].trim();

		assert_eq!("item // TODO: item", output);
	}
}