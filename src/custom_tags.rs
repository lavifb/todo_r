// Module for creating regexs for custom tags

use regex::Regex;

/// Enum storing the different supported comment types
// TODO: change to a hashmap to support adding more comment types
pub enum CommentType {
	C,
	Py,
}

// TODO: add more languages/patterns
impl CommentType {
	fn prefix(&self) -> &str {
		match self {
			CommentType::C =>  "//",
			CommentType::Py => "#",
		}
	}
}

// TODO: use a better regex to find TODOs
pub fn get_regex(custom_tags: &[&str], comment_type: CommentType) -> Regex {
	let tags_string: String = custom_tags.join("|");

	let todo_regex: &str = 
		&format!(r"(?i)^\s*{}\s*({})\s*:?\s+{}$",  // whitespace and optional colon
		         comment_type.prefix(),            // comment prefix token
		         tags_string,                      // custom tags
		         r"(.*)",                          // TODO content
		);

	let re = Regex::new(todo_regex).unwrap();
	re
}