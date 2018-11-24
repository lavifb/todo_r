// Module for the structs that hold the comment types

use regex::escape;


// MAYB: May need to be enum to really treat single and block separately
pub trait CommentType {
	fn prefix(&self) -> &str;
	fn suffix(&self) -> &str;
}

/// Struct for storing a type of single-line comment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct SingleLineComment {
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
	fn prefix(&self) -> &str {
		&self.token
	}

	fn suffix(&self) -> &str {
		"$"
	}
}

/// Struct for storing a type of block comment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct BlockComment {
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
	fn prefix(&self) -> &str {
		&self.prefix
	}

	fn suffix(&self) -> &str {
		&self.suffix
	}
}

/// Struct for storing a collection of CommentTypes that correspond to a specifix content type.
#[derive(Debug, Clone, Serialize, Deserialize)]
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

	/// Creates new CommentTypes struct from CommentsConfig.
	pub(crate) fn from_config(config: CommentsConfig) -> CommentTypes {
		CommentTypes {
			single: config.single,
			block: config.block,
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
	pub fn iter_comment_types(&self) -> impl Iterator<Item = &dyn CommentType> {
		self.single.iter().map(|c| c as &CommentType).chain(self.block.iter().map(|c| c as &CommentType))
	}
}

impl Default for CommentTypes {
	fn default() -> CommentTypes {
		Self::new()
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct CommentsConfig {
	pub ext: String,
	pub(self) single: Vec<SingleLineComment>,
	pub(self) block: Vec<BlockComment>,
}