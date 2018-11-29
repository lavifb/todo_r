// Module for the structs that hold the comment types

use serde::{Deserialize, Deserializer};
use regex::escape;


/// An enum for custom comment types.
///
/// There are two types of comments:
/// 	SingleLine: for single line comments like `// comment`
/// 	Block: for block comments like `/* comment */`
///
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum CommentType {
	SingleLine(SingleLineComment),
	Block(BlockComment),
}

impl CommentType {
	/// Creates new single-line comment type
	pub fn new_single(prefix: &str) -> CommentType {
		SingleLineComment::new(prefix).into()
	}

	/// Creates new block comment type
	pub fn new_block(prefix: &str, suffix: &str) -> CommentType {
		BlockComment::new(prefix, suffix).into()
	}

	/// Returns prefix token for comment.
	pub fn prefix(&self) -> &str {
		match self {
			CommentType::SingleLine(c) => &c.prefix,
			CommentType::Block(c) => &c.prefix,
		}
	}

	/// Returns suffix token for comment.
	pub fn suffix(&self) -> &str {
		match self {
			CommentType::SingleLine(_c) => "$",
			CommentType::Block(c) => &c.suffix,
		}
	}
}

/// Stores a single-line comment type.
/// This holds the prefix for single-lines comments.
/// For Rust comments it should hold `//`.
#[derive(Debug, Clone, Deserialize)]
pub struct SingleLineComment {
	#[serde(rename = "single")]
	#[serde(deserialize_with = "escape_deserialize")]
	prefix: String,
}

impl SingleLineComment {
	pub fn new(prefix: &str) -> SingleLineComment {
		SingleLineComment {
			prefix: escape(prefix),
		}
	}
}

impl Into<CommentType> for SingleLineComment {
	fn into(self) -> CommentType {
		CommentType::SingleLine(self)
	}
}

/// Stores a block comment type.
/// This holds the prefix and suffix for block comments.
/// For Rust comments it should hold `/*` and `*/`.
#[derive(Debug, Clone, Deserialize)]
pub struct BlockComment {
	#[serde(deserialize_with = "escape_deserialize")]
	prefix: String,
	#[serde(deserialize_with = "escape_deserialize")]
	suffix: String,
}

impl BlockComment {
	pub fn new(prefix: &str, suffix: &str) -> BlockComment {
		BlockComment {
			prefix: escape(prefix),
			suffix: escape(suffix),
		}
	}
}

impl Into<CommentType> for BlockComment {
	fn into(self) -> CommentType {
		CommentType::Block(self)
	}
}

fn escape_deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
where 
	D: Deserializer<'de>,
{
	let s: String = Deserialize::deserialize(deserializer)?;
	Ok(escape(&s))
}

/// Struct for storing a collection of CommentType enums that correspond to a specifix content type.
/// It behaves as a wrapper for Vec<CommentType>.
#[derive(Debug, Default, Clone, Deserialize)]
pub struct CommentTypes {
	comment_types: Vec<CommentType>,
}

impl CommentTypes {
	/// Creates new CommentTypes struct.
	pub fn new() -> CommentTypes {
		CommentTypes {
			..Default::default()
		}
	}

	/// Adds a single-line comment type with the provided prefix.
	/// For Rust single-line comments you might use `CommentTypes::new().add_single("//")`
	pub fn add_single(mut self, prefix: &str) -> Self {
		self.comment_types.push(CommentType::new_single(prefix));
		self
	}

	/// Adds a block comment type with the provided prefix and suffix.
	/// For Rust block comments you might use `CommentTypes::new().add_block("/*", "*/")`
	pub fn add_block(mut self, prefix: &str, suffix: &str) -> Self {
		self.comment_types.push(CommentType::new_block(prefix, suffix));
		self
	}

	/// Returns an iterator over all of the comment types in the struct.
	pub fn iter(&self) -> std::slice::Iter<CommentType> {
		self.into_iter()
	}
}

impl IntoIterator for CommentTypes {
	type Item = CommentType;
	type IntoIter = std::vec::IntoIter<CommentType>;

	fn into_iter(self) -> std::vec::IntoIter<CommentType> {
		self.comment_types.into_iter()
	}
}

impl<'a> IntoIterator for &'a CommentTypes {
	type Item = &'a CommentType;
	type IntoIter = std::slice::Iter<'a, CommentType>;

	fn into_iter(self) -> std::slice::Iter<'a, CommentType> {
		self.comment_types.iter()
	}
}

#[derive(Debug, Default, Clone, Deserialize)]
pub(crate) struct CommentsConfig {
	#[serde(default)]
	pub ext: String,
	#[serde(default)]
	pub exts: Vec<String>,
	pub(self) types: Vec<CommentType>,
}

impl CommentsConfig {
	pub fn break_apart(self) -> (String, Vec<String>, CommentTypes) {
		(self.ext, self.exts, CommentTypes {comment_types: self.types})
	}
}

#[derive(Debug, Default, Clone, Deserialize)]
pub(crate) struct TodorConfigFileSerial {
	#[serde(default)]
	pub verbose: bool,
	#[serde(default)]
	pub tags: Vec<String>,
	#[serde(default)]
	pub ignore: Vec<String>,
	#[serde(default)]
	pub comments: Vec<CommentsConfig>,
}