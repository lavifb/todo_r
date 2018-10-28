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

/// Creates a list of TODOs found in content
// MAYB: return iterator instead of Vec 
pub fn find_todos(content: &str, file_ext: &str, todo_words: &[&str]) -> Vec<Todo> {
	// TODO: replace with hashmap as described in custom_tags.rs
	let comment_type = match file_ext {
		"rs" => CommentType::SSlash,
		"c" => CommentType::SSlash,
		"cpp" => CommentType::SSlash,
		"py" => CommentType::Hash,
		"tex" => CommentType::Percent,
		"hs" => CommentType::DDash,
		"sql" => CommentType::DDash,
		".gitignore" => CommentType::Hash,
		_ => CommentType::SSlash,
	};

	let regex_string = get_regex_string(todo_words, comment_type);
	// TODO: test multiple comment types at once
	let re = Regex::new(&regex_string).unwrap();
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

	fn test_content(content: &str, exp_result: &str, comment_type: CommentType) {
		let regex_string = get_regex_string(&["TODO", "FIXME"], comment_type);
		let re = Regex::new(&regex_string).unwrap();

		let cap = re.captures(content);
		match cap {
			Some(cap) => {
				let result = cap[2].trim();
				assert_eq!(exp_result, result);
			}
			None => {
				assert_eq!(exp_result, "NONE");
			}
		}
	}

	#[test]
	fn regex_whitespace() {
		test_content("\t\t\t\t  //  TODO:  item \t", "item", CommentType::SSlash);
	}

	#[test]
	fn regex_todo_in_comment() {
		test_content("//  TODO:  item // TODO: item \t", "item // TODO: item", CommentType::SSlash);
	}
	
	#[test]
	fn regex_optional_colon() {
		test_content("//  TODO  item // TODO: item \t", "item // TODO: item", CommentType::SSlash);
	}

	#[test]
	fn regex_case_insensitive() {
		test_content("// tODo: case ", "case", CommentType::SSlash);
	}

	#[test]
	fn regex_todop() {
		test_content("// todop: nope ", "NONE", CommentType::SSlash);
	}

	#[test]
	fn regex_todf() {
		test_content("// todf: nope ", "NONE", CommentType::SSlash);
	}

	#[test]
	fn regex_todofixme() {
		test_content("// todofixme : nope ", "NONE", CommentType::SSlash);
	}

	#[test]
	fn regex_py_comment() {
		test_content("# todo: item \t ", "item", CommentType::Hash);
	}

	#[test]
	fn regex_percent_comment() {
		test_content("% todo: item \t ", "item", CommentType::Percent);
	}

	#[test]
	fn regex_ddash_comment() {
		test_content("-- todo: item \t ", "item", CommentType::DDash);
	}

	#[test]
	fn regex_slashstar_comment() {
		test_content("/* todo: item \t */ \t ", "item", CommentType::SlashStar);
	}

	#[test]
	fn regex_slashstar_comment_double_prefix() {
		test_content("/* todo: item /* todo: decoy*/\t ", "item /* todo: decoy", CommentType::SlashStar);
	}

	#[test]
	fn regex_slashstar_comment_double_suffix() {
		test_content("/* todo: item */ \t other stuff */ ", "item", CommentType::SlashStar);
	}

	#[test]
	fn regex_py_in_c_file() {
		test_content("# todo: item \t ", "NONE", CommentType::SSlash);
	}

	#[test]
	fn regex_c_comment_in_py_comment() {
		test_content("# todo: \\ todo: item \t ", "\\ todo: item", CommentType::Hash);
	}

	#[test]
	fn regex_c_comment_in_py_comment_in_c_file() {
		test_content("# todo: \\ todo: item \t ", "NONE", CommentType::SSlash);
	}

	#[test]
	fn regex_comment_not_on_separate_line() {
		test_content("do_things(); \\ todo: item", "NONE", CommentType::SSlash);
	}
}