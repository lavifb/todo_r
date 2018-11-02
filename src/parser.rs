// Module for finding TODOs in files

use regex::Regex;
use std::fmt;
use ansi_term::Style;

use custom_tags::{get_regex_string, CommentType};

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
	pub fn style_string(&self, line_style: &Style, todo_style: &Style, content_style: &Style) -> String {
		format!("  {}  {}  {}", 
			// MAYB: figure out biggest line number to use for formatting
			line_style.paint(format!("line {:<5}", self.line)), // works up to 100,000 lines which should be a long enough file...
			todo_style.paint(format!("{:5}", &self.todo_type)),
			content_style.paint(&self.content),
			)
	}
}

impl fmt::Display for Todo {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "line {}\t{}\t{}", self.line, self.todo_type, self.content)
	}
}

/// Parses content and Creates a list of TODOs found in content
// MAYB: return iterator instead of Vec 
pub fn parse_content(content: &str, file_ext: &str, todo_words: &[String]) -> Vec<Todo> {
	// TODO: change to a hashmap to support adding more comment types
	// TODO: only store pointers to CommentType to avoid so much repitition in hashmap
	let comment_types: Vec<CommentType> = match file_ext {
		"rs" | "c" | "cpp" => vec![CommentType::new_one_line("//"), CommentType::new_block("/*", "*/")],
		"py" => vec![CommentType::new_one_line("#"), CommentType::new_block("\"\"\"", "\"\"\"")],
		"tex" => vec![CommentType::new_one_line("%")],
		"hs" => vec![CommentType::new_one_line("--")],
		"sql" => vec![CommentType::new_one_line("--")],
		"html" | "md" => vec![CommentType::new_block("<!--", "-->")],
		".gitignore" => vec![CommentType::new_one_line("#")],
		_ => vec![CommentType::new_one_line("//")],
	};

	let mut regexs: Vec<Regex> = Vec::new();

	for comment_type in comment_types.iter() {
		let regex_string = get_regex_string(todo_words, comment_type);
		regexs.push(Regex::new(&regex_string).unwrap());
	}
	let mut todos = Vec::new();

	for (line_num, line) in content.lines().enumerate() {
		for re in regexs.iter() {
			let todo_content = re.captures(line);
			
			if let Some(todo_content) = todo_content {
				let todo = Todo::new(line_num+1, &todo_content[1].trim().to_uppercase(), todo_content[2].trim());
				todos.push(todo);
			};
		}
	}

	todos
}

#[cfg(test)]
mod tests {
	use super::*;

	fn test_content(content: &str, exp_result: &str, file_ext: &str) {

		let todos = parse_content(content, file_ext, &["TODO".to_string()]);
		if todos.is_empty() {
			assert_eq!(exp_result, "NONE");
		} else {
			assert_eq!(exp_result, todos[0].content);
		}
	}

	#[test]
	fn find_todos_block_and_line1() {
		test_content("/* // todo: i
			tem */", "NONE", "rs");
	}

	#[test]
	fn find_todos_block_and_line2() {
		test_content("/* todo: // item */", "// item", "rs");
	}

	#[test]
	fn find_todos_block_and_line3() {
		test_content(" // /* todo: item */", "NONE", "rs");
	}

	#[test]
	fn find_todos_block_and_line4() {
		test_content(" //   todo:  /* item */", "/* item */", "rs");
	}

	#[test]
	fn find_todos_py_in_c_file() {
		test_content("# todo: item \t ", "NONE", "c");
	}

	#[test]
	fn find_todos_c_comment_in_py_comment() {
		test_content("# todo: \\ todo: item \t ", "\\ todo: item", "py");
	}

	#[test]
	fn find_todos_c_comment_in_py_comment_in_c_file() {
		test_content("# todo: \\ todo: item \t ", "NONE", "c");
	}
}