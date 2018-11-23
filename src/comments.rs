// Module for the structs that hold the comment types

use regex::escape;


// MAYB: May need to be enum to really treat single and block separately
pub trait CommentType {
	fn prefix<'a>(&'a self) -> &'a str;
	fn suffix<'a>(&'a self) -> &'a str;
}

/// Struct for storing a type of single-line comment.
pub struct SingleLineComment {
	token: String,
}

impl SingleLineComment {
	/// Creates new SingleLineComment with the provided comment token.
	pub fn new(token: &str) -> SingleLineComment {
		SingleLineComment {
			token: escape(token),
		}
	}
}

impl CommentType for SingleLineComment {
	fn prefix<'a>(&'a self) -> &'a str {
		&self.token
	}

	fn suffix<'a>(&'a self) -> &'a str {
		"$"
	}
}

/// Struct for storing a type of block comment.
pub struct BlockComment {
	prefix: String,
	suffix: String,
}

impl BlockComment {
	/// Creates new BlockComment with the provided prefix and suffix tokens.
	pub fn new(prefix: &str, suffix: &str) -> BlockComment {
		BlockComment {
			prefix: escape(prefix),
			suffix: escape(suffix),
		}
	}
}

impl CommentType for BlockComment {
	fn prefix<'a>(&'a self) -> &'a str {
		&self.prefix
	}

	fn suffix<'a>(&'a self) -> &'a str {
		&self.suffix
	}
}