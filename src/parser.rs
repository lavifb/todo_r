// Module for finding TODOs in files

use regex::Regex;
use ansi_term::Style;
use std::fmt;
use std::io::BufRead;

use custom_tags::get_regex_for_comment;
use comments::CommentTypes;

/// A struct holding the TODO and all the needed meta-information for it.
pub struct Todo {
	pub line: usize,
	todo_type: String,
	content: String,
}

impl Todo {
	/// Create new TODO struct
	fn new(line: usize, todo_type_str: &str, content_str: &str) -> Todo {
		Todo {
			line,
			todo_type: todo_type_str.to_uppercase(),
			content: content_str.to_string(),
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
pub(crate) fn parse_content<B>(content_buf: &mut B, comment_types: &CommentTypes, tags: &[String]) -> Result<Vec<Todo>, std::io::Error>
where
	B: BufRead,
{
	let regexs: Vec<Regex> = comment_types
		.iter()
		.map(|c| get_regex_for_comment(tags, c).unwrap())
		.collect();

	// MAYB: do this as iterator and collect
	let mut todos = Vec::new();
	for (line_num, line_result) in content_buf.lines().enumerate() {
		let line = line_result?;

		for re in regexs.iter() {
			if let Some(todo_content) = re.captures(&line) {
				let todo = Todo::new(line_num+1, todo_content[1].trim(), todo_content[2].trim());
				todos.push(todo);
			};
		}
	}

	Ok(todos)
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::io::Cursor;

	use comments::CommentTypes;

	fn test_content(content: &str, exp_result: &str, file_ext: &str) {
		let comment_types = match file_ext {
			"rs" => CommentTypes::new().add_single("//").add_block("/*", "*/"),
			"c"  => CommentTypes::new().add_single("//").add_block("/*", "*/"),
			"py" => CommentTypes::new().add_single("#").add_block("\"\"\"", "\"\"\""),
			_    => CommentTypes::new().add_single("//").add_block("/*", "*/"),
		};

		let mut content_buf = Cursor::new(content);
		let todos = parse_content(&mut content_buf, &comment_types, &["TODO".to_string()]).unwrap();
		if todos.is_empty() {
			assert_eq!(exp_result, "NONE");
		} else {
			assert_eq!(exp_result, todos[0].content);
		}
	}

	#[test]
	fn find_todos_block_and_line1() {
		test_content("/* // todo: item */", "NONE", "rs");
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