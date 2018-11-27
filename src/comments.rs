// Module for the structs that hold the comment types

use regex::escape;


/// An enum for custom comment types.
///
/// There are two types of comments:
/// 	SingleLine: for single line comments like `// comment`
/// 	Block: for block comments like `/* comment */`
///
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CommentType {
	/// Stores a single-line comment type.
	SingleLine {
		#[serde(rename = "single")]
		prefix: String,
	},

	/// Stores a single-line comment type.
	Block {
		prefix: String,
		suffix: String,
	},
}

// TODO: move escaping to creation and get serde to deserialize escaped
impl CommentType {
	/// Creates new single-line comment type
	pub fn new_single(prefix: &str) -> CommentType {
		CommentType::SingleLine {
			prefix: prefix.to_string(),
		}
	}

	/// Creates new block comment type
	pub fn new_block(prefix: &str, suffix: &str) -> CommentType {
		CommentType::Block {
			prefix: prefix.to_string(),
			suffix: suffix.to_string(),
		}
	}

	/// Returns prefix token for comment.
	pub fn prefix(&self) -> String {
		match self {
			CommentType::SingleLine{prefix} => escape(prefix),
			CommentType::Block{prefix, ..} => escape(prefix),
		}
	}

	/// Returns suffix token for comment.
	pub fn suffix(&self) -> String {
		match self {
			CommentType::SingleLine{..} => "$".to_string(),
			CommentType::Block{suffix, ..} => escape(suffix),
		}
	}
}

/// Struct for storing a collection of CommentType enums that correspond to a specifix content type.
/// It behaves as a wrapper for Vec<CommentType>.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
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

	/// Creates new CommentTypes struct from CommentsConfig.
	pub(crate) fn from_config(config: CommentsConfig) -> CommentTypes {
		CommentTypes {
			comment_types: config.types,
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

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct CommentsConfig {
	#[serde(default)]
	pub ext: String,
	#[serde(default)]
	pub exts: Vec<String>,
	pub(self) types: Vec<CommentType>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub(crate) struct TodorConfigFileSerial {
	#[serde(default)]
	pub tags: Vec<String>,
	#[serde(default)]
	pub ignore: Vec<String>,
	#[serde(default)]
	pub comments: Vec<CommentsConfig>,
}