// Module for creating regexs for custom tags

use regex::escape;

// MAYB: seperate single-line and block comments for potential different treatment later...
#[derive(Clone)]
pub struct CommentType {
	prefix: String,
	suffix: String,
}

impl CommentType {
	pub fn single_line(prefix: &str) -> CommentType {
		CommentType {
			prefix: escape(prefix),
			suffix: "$".to_string(),
		}
	}

	pub fn block(prefix: &str, suffix: &str) -> CommentType {
		CommentType {
			prefix: escape(prefix),
			suffix: escape(suffix),
		}
	}
}

// MAYB: use a better regex to find TODOs
pub fn get_regex_string(custom_tags: &[String], comment_type: &CommentType) -> String {
	let tags_string: String = custom_tags.join("|");

	format!(r"(?i)^\s*{}\s*({})\s*:?\s+{}{}",  // whitespace and optional colon
	         comment_type.prefix,              // comment prefix token
	         tags_string,                      // custom tags
	         r"(.*?)",                         // content
	         comment_type.suffix,              // comment prefix token
	)
}

#[cfg(test)]
mod tests {
	use super::*;
	use regex::Regex;

	fn test_regex(content: &str, exp_result: &str, comment_type: &CommentType) {
		let regex_string = get_regex_string(&["TODO".to_string(), "FIXME".to_string()], comment_type);

		let re = Regex::new(&regex_string).unwrap();
		let todo_content = re.captures(content);
		match todo_content {
			Some(todo_content) => assert_eq!(exp_result, todo_content[2].trim()),
			None               => assert_eq!(exp_result, "NONE"),
		};
	}


	#[test]
	fn regex_whitespace() {
		test_regex("\t\t\t\t  //  TODO:  item \t", "item", &CommentType::single_line("//"));
	}

	#[test]
	fn regex_todo_in_comment() {
		test_regex("//  TODO:  item // TODO: item \t", "item // TODO: item", &CommentType::single_line("//"));
	}
	
	#[test]
	fn regex_optional_colon() {
		test_regex("//  TODO  item // TODO: item \t", "item // TODO: item", &CommentType::single_line("//"));
	}

	#[test]
	fn regex_case_insensitive() {
		test_regex("// tODo: case ", "case", &CommentType::single_line("//"));
	}

	#[test]
	fn regex_fixme() {
		test_regex("\t\t\t\t  //  fixMe:  item for fix \t", "item for fix", &CommentType::single_line("//"));
	}

	#[test]
	fn regex_todop() {
		test_regex("// todop: nope ", "NONE", &CommentType::single_line("//"));
	}

	#[test]
	fn regex_todf() {
		test_regex("// todf: nope ", "NONE", &CommentType::single_line("//"));
	}

	#[test]
	fn regex_todofixme() {
		test_regex("// todofixme : nope ", "NONE", &CommentType::single_line("//"));
	}

	#[test]
	fn regex_py_comment() {
		test_regex("# todo: item \t ", "item", &CommentType::single_line("#"));
	}

	#[test]
	fn regex_percent_comment() {
		test_regex("% todo: item \t ", "item", &CommentType::single_line("%"));
	}

	#[test]
	fn regex_ddash_comment() {
		test_regex("-- todo: item \t ", "item", &CommentType::single_line("--"));
	}

	#[test]
	fn regex_slashstar_comment() {
		test_regex("/* todo: item \t */ \t ", "item", &CommentType::block("/*", "*/"));
	}

	#[test]
	fn regex_slashstar_comment_double_prefix() {
		test_regex("/* todo: item /* todo: decoy*/\t ", "item /* todo: decoy", &CommentType::block("/*", "*/"));
	}

	#[test]
	fn regex_slashstar_comment_double_suffix() {
		test_regex("/* todo: item */ \t other stuff */ ", "item", &CommentType::block("/*", "*/"));
	}

	#[test]
	fn regex_comment_not_on_separate_line() {
		test_regex("do_things(); \\ todo: item", "NONE", &CommentType::single_line("//"));
	}

	#[test]
	fn regex_block_todo_before_function() {
		test_regex("/* todo: item */ do_things();", "item", &CommentType::block("/*", "*/"));
	}
}