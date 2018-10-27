// Module for creating regexs for custom tags

use regex::Regex;

/// Enum storing the different supported comment types.
/// They are named after the comment symbol with the first letter repeated the number of time the symbol is repeated.
/// For instance, `CommentType::SSlash` refers to `//`
// TODO: change to a hashmap to support adding more comment types
pub enum CommentType {
	SSlash,
	Hash,
	Percent,
	DDash,
}

// TODO: add more languages/patterns
impl CommentType {
	fn prefix(&self) -> &str {
		match self {
			CommentType::SSlash  => "//",
			CommentType::Hash    => "#",
			CommentType::Percent => "%",
			CommentType::DDash   => "--",
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