// Module for creating regexs for custom tags

use regex::Regex;

// TODO: add custom TODO keywords
// TODO: add more regexs for other languages/patterns
// TODO: use a better regex to find TODOs
static TODO_REGEX: &str = r"^\s*//\s*TODO\s*:?(.*)$";

pub fn get_regex(custom_tags: Vec<&str>) -> Regex {
	let re = Regex::new(TODO_REGEX).unwrap();

	re
}