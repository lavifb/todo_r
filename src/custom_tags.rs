// Module for creating regexs for custom tags

use regex::Regex;

// TODO: add more regexs for other languages/patterns
// TODO: use a better regex to find TODOs
pub fn get_regex(custom_tags: Vec<&str>) -> Regex {
	let tags_string: String = custom_tags.join("|");

	let todo_regex: &str = &format!(r"(?i)^\s*{}\s*({})\s*:?\s+{}$",	// whitespace and optional colon
									r"//", 								// comment token
									tags_string, 						// custom tags
									r"(.*)");							// TODO content

	let re = Regex::new(todo_regex).unwrap();

	re
}