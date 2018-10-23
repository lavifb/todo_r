// Module for finding TODOs in files

use regex::Regex;
use std::fmt;
use ansi_term::Color;

use custom_tags::get_regex;

/// A struct holding the TODO and all the needed meta-information for it.
pub struct Todo {
	line: usize,
	todo_type: String,
	content: String,
}

impl Todo {
	/// Create new TODO struct
	fn new(line: usize, todo_type_str: &str, content_str: &str) -> Todo {
		let todo_type = todo_type_str.to_string();
		let content = content_str.to_string();

		Todo {
			line,
			todo_type,
			content,
		}
	}

	/// Returns colored output string
	pub fn color_string(&self, line_color: &Color, todo_color: &Color, content_color: &Color) -> String {
		// TODO: format using something other than \t tabs
		format!("  {}{}\t{}\t{}", 
			line_color.paint("line "), line_color.paint(self.line.to_string()),
			todo_color.paint(&self.todo_type),
			content_color.paint(&self.content),
			)
	}
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {}\t{}\t{}", self.line, self.todo_type, self.content)
    }
}

/// Creates a list of TODOs found in content
// TODO: Maybe return iterator instead of Vec 
pub fn find_todos(content: &str) -> Vec<Todo> {
	// TODO: add custom TODO keywords
	let re: Regex = get_regex(&["TODO", "fixme"]);
	let mut todos = Vec::new();

	for (line_num, line) in content.lines().enumerate() {
		let todo_content = re.captures(line);
		match todo_content {
			Some(todo_content) => {
				let todo = Todo::new(line_num+1, &todo_content[1].trim().to_uppercase(), todo_content[2].trim());
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

	fn test_content(content: &str, exp_result: &str) {
		let re: Regex = get_regex(vec!["TODO", "FIXME"]);
		let cap = re.captures(content);
		match cap {
			Some(cap) => {
				let result = cap[2].trim();
				assert_eq!(exp_result, result);
			}
			None => {
				assert_eq!(exp_result, "ERROR");
			}
		}

		
	}

	#[test]
	fn regex_whitespace() {
		test_content("\t\t\t\t  //  TODO:  item \t", "item");
	}

	#[test]
	fn regex_todo_in_comment() {
		test_content("//  TODO:  item // TODO: item \t", "item // TODO: item");
	}
	
	#[test]
	fn regex_optional_colon() {
		test_content("//  TODO  item // TODO: item \t", "item // TODO: item");
	}

	#[test]
	fn regex_case_insensitive() {
		test_content("// tODo: case ", "case");
	}

	#[test]
	fn regex_todop() {
		test_content("// todop: nope ", "ERROR");
	}

	#[test]
	fn regex_todf() {
		test_content("// todf: nope ", "ERROR");
	}

	#[test]
	fn regex_todofixme() {
		test_content("// todofixme : nope ", "ERROR");
	}
}