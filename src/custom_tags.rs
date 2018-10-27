// Module for creating regexs for custom tags

/// Enum storing the different supported comment types.
/// They are named after the comment symbol with the first letter repeated the number of time the symbol is repeated.
/// For instance, `CommentType::SSlash` refers to `//`
// TODO: change to a hashmap to support adding more comment types
// TODO: instead of an enum use a struct that contains a prefix and suffix and is pointed to by a hashmap
pub enum CommentType {
	SSlash,
	Hash,
	Percent,
	DDash,
	SlashStar,
}

// TODO: add more languages/patterns
impl CommentType {
	/// Returns comment prefix token
	fn prefix(&self) -> &str {
		match self {
			CommentType::SSlash    => "//",
			CommentType::Hash      => "#",
			CommentType::Percent   => "%",
			CommentType::DDash     => "--",
			CommentType::SlashStar => r"/\*",
		}
	}

	/// Returns comment suffix token. Single line comments have an EOL `$` as their suffix
	fn suffix(&self) -> &str {
		match self {
			CommentType::SSlash    => "$",
			CommentType::Hash      => "$",
			CommentType::Percent   => "$",
			CommentType::DDash     => "$",
			CommentType::SlashStar => r"\*/",
		}
	}
}

// MAYB: use a better regex to find TODOs
pub fn get_regex_string(custom_tags: &[&str], comment_type: CommentType) -> String {
	let tags_string: String = custom_tags.join("|");

	format!(r"(?i)^\s*{}\s*({})\s*:?\s+{}{}",  // whitespace and optional colon
	         comment_type.prefix(),            // comment prefix token
	         tags_string,                      // custom tags
	         r"(.*?)",                         // TODO content
	         comment_type.suffix(),            // comment prefix token
	)
}