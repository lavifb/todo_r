// Module for the structs that hold the comment types

use regex::escape;


// MAYB: May need to be enum to really treat single and block separately
pub trait CommentType {
	fn prefix<'a>(&'a self) -> &'a str;
	fn suffix<'a>(&'a self) -> &'a str;
}

/// Struct for storing a type of single-line comment.
#[derive(Clone)]
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
#[derive(Clone)]
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

/// Struct for storing a collection of CommentTypes that correspond to a specifix content type.
#[derive(Clone)]
pub struct CommentTypes {
	single: Vec<SingleLineComment>,
	block: Vec<BlockComment>,
}

impl CommentTypes {
	/// Creates new CommentTypes struct.
	pub fn new() -> CommentTypes {
		CommentTypes {
			single: Vec::new(),
			block: Vec::new(),
		}
	}

	/// Adds a single-line comment type.
	pub fn add_single(mut self, token: &str) -> Self {
		self.single.push(SingleLineComment::new(token));
		self
	}

	/// Adds a block comment type.
	pub fn add_block(mut self, prefix: &str, suffix: &str) -> Self {
		self.block.push(BlockComment::new(prefix, suffix));
		self
	}

	// TODO: use IntoIter
	/// Returns an iterator over all of the comment types in the struct.
	pub fn iter_comment_types<'a>(&'a self) -> Box<Iterator<Item = &CommentType> + 'a> {
		Box::new(self.single.iter().map(|c| c as &CommentType).chain(self.block.iter().map(|c| c as &CommentType)))
	}
}