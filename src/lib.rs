#[macro_use] extern crate failure;
extern crate regex;
extern crate ansi_term;

mod parser;
mod display;
mod custom_tags;
mod remover;

pub mod errors {
	use failure::Error;

	/// Custom Errors for TodoR
	#[derive(Debug, Fail)]
	pub enum TodoRError {
		/// Error for when provided file path is a directory
		#[fail(display = "'{}' is a directory.", filepath)]
		FileIsDir {
			filepath: String,
		},
		/// Error for when provided file extension is not supported
		#[fail(display = "'{}' is an invalid extension.", ext)]
		InvalidExtension {
			ext: String,
		},
		/// Error for when provided filepath for modification is not tracked
		#[fail(display = "'{}' is not a tracked file.", filepath)]
		FileNotTracked {
			filepath: String,
		},
		/// Error for when provided TODO line is not found
		#[fail(display = "TODO comment not found in line {}.", line)]
		TodoNotFound {
			line: usize
		},
	}

	use ansi_term::Colour::Red;

	/// Prints err to stderr
	pub fn eprint_error(err: &Error) {
		match err {
			_ => eprintln!("{}: {}", Red.paint("[todo_r error]"), err.to_string()),
		};
	}
}

use std::fs::File;
use std::path::Path;
use std::ffi::OsStr;
use std::io::{self, Write, BufReader, Cursor};
use std::collections::HashMap;

use failure::Error;
use errors::TodoRError;

use parser::parse_content;
use display::{StyleConfig, write_file_todos, TodoFile};
use custom_tags::CommentType;


/// Configuration for `TodoR`.
///
/// `verbose` holds whether to print extra content.
/// `todo_words` gives a list of the TODO terms to search for.
pub struct TodoRConfig {
	pub verbose: bool,
	pub todo_words: Vec<String>,
	styles: StyleConfig,
	ext_to_comment_types: HashMap<String, Vec<CommentType>>,
	default_comment_types: Vec<CommentType>,
}

impl TodoRConfig {
	pub fn new() -> TodoRConfig {
		TodoRConfig {
			verbose: false,
			todo_words: Vec::new(),
			styles: StyleConfig::default(),
			ext_to_comment_types: default_comment_types_map(),
			default_comment_types: vec![CommentType::new_one_line("#")],
		}
	}

	// TODO: take String or &str using Cow or something
	pub fn with_todo_words(todo_words: &[String]) -> TodoRConfig {
		// let todo_word_strings: Vec<String> = .collect();
		
		TodoRConfig {
			verbose: false,
			todo_words: todo_words.to_vec(),
			styles: StyleConfig::default(),
			ext_to_comment_types: default_comment_types_map(),
			default_comment_types: vec![CommentType::new_one_line("#")],
		}
	}

	pub fn set_no_style(&mut self) {
		self.styles = StyleConfig::no_style();
	}

	pub fn set_default_ext(&mut self, ext: &str) -> Result<(), Error> {
		self.default_comment_types = self.ext_to_comment_types.get(ext).ok_or(
			TodoRError::InvalidExtension {
				ext: ext.to_string()
			}
		).unwrap().to_vec();

		Ok(())
	}

	// TODO: function to add comment types
	// TODO: function to add default comment types (not by ext)
}

/// Parser for finding TODOs in comments and storing them on a per-file basis.
pub struct TodoR {
	pub config: TodoRConfig,
	todo_files: Vec<TodoFile>,
}

impl TodoR {
	/// Creates new TodoR that looks for provided keywords.
	pub fn new() -> TodoR {
		TodoR {
			config: TodoRConfig::new(),
			todo_files: Vec::new(),
		}
	}

	pub fn with_todo_words(todo_words: &[String]) -> TodoR {
		TodoR {
			config: TodoRConfig::with_todo_words(todo_words),
			todo_files: Vec::new(),
		}
	}

	/// Creates new TodoR using given configuration.
	pub fn with_config(config: TodoRConfig) -> TodoR {
		TodoR {
			config,
			todo_files: Vec::new(),
		}
	}

	/// Returns the number of files currently tracked by TodoR
	pub fn num_files(&self) -> usize {
		self.todo_files.len()
	}

	/// Returns all tracked files that contain TODOs
	pub fn get_tracked_files<'a>(&'a self) -> Vec<&'a str> {
		self.todo_files.iter()
			.filter(|tf| tf.todos.len() > 0)
			.map(|tf| tf.filepath.to_str().unwrap())
			.collect()
	}

	/// Returns all tracked files even if they have no TODOs
	pub fn get_all_tracked_files<'a>(&'a self) -> Vec<&'a str> {
		self.todo_files.iter()
			.map(|tf| tf.filepath.to_str().unwrap())
			.collect()
	}

	/// Opens file at given filepath and process it by finding all its TODOs.
	pub fn open_todos(&mut self, filepath: &Path) -> Result<(), Error> {
		let mut todo_file = TodoFile::new(filepath);

		// Make sure the file is not a directory
		if filepath.metadata()?.is_dir() {
			return Err(TodoRError::FileIsDir {
				filepath: filepath.to_string_lossy().to_string()
			}.into());
		}

		let file_ext = filepath.extension().unwrap_or_else(|| OsStr::new(".sh"));
		let comment_types = self.config.ext_to_comment_types.get(file_ext.to_str().unwrap())
								.unwrap_or(&self.config.default_comment_types);
		
		let file = File::open(filepath)?;
		let mut file_reader = BufReader::new(file);
		todo_file.set_todos(parse_content(&mut file_reader, &comment_types, &self.config.todo_words)?);

		self.todo_files.push(todo_file);
		Ok(())
	}

	/// Finds TODO comments in the given content
	pub fn find_todos(&mut self, content: &str) -> Result<(), Error> {
		let mut todo_file = TodoFile::new(Path::new(""));
		let mut content_buf = Cursor::new(content);
		todo_file.set_todos(parse_content(&mut content_buf, &self.config.default_comment_types, &self.config.todo_words)?);

		self.todo_files.push(todo_file);
		Ok(())
	}

	/// Prints TODOs to stdout.
	pub fn print_todos(&self) {
		// lock stdout to print faster
		let stdout = io::stdout();
		let lock = stdout.lock();
		let mut out_buffer = io::BufWriter::new(lock);

		self.write_todos(&mut out_buffer);
	}

	/// Writes TODOs to out_buffer.
	pub fn write_todos(&self, out_buffer: &mut Write) {
		for todo_file in &self.todo_files {
			if todo_file.is_empty() && !self.config.verbose {
				continue
			}

			write_file_todos(out_buffer, &todo_file, &self.config.styles);
		}
	}

	/// Writes TODOs to out_buffer.
	// MAYB: change self.todo_files to Hashmap for easier finding
	pub fn write_todos_from_file(&self, filepath: &Path, out_buffer: &mut Write) {
		for todo_file in &self.todo_files {
			if todo_file.filepath == filepath {
				write_file_todos(out_buffer, &todo_file, &self.config.styles);
				break;
			}
		}
	}

	/// Deletes TODO line from given filepath corresponding to the given index.
	pub fn remove_todo(&mut self, filepath: &Path, todo_index: usize) -> Result<(), Error> {
		for mut todo_file in &mut self.todo_files {
			if filepath == todo_file.filepath {
				remover::remove_todo_by_index(&mut todo_file, todo_index)?;

				return Ok(());
			}
		}

		Err(TodoRError::FileNotTracked {
			filepath: filepath.to_string_lossy().to_string()
		}.into())
	}

	/// Deletes TODO line from given filepath corresponding to the given line.
	pub fn remove_todo_line(&mut self, filepath: &Path, line: usize) -> Result<(), Error> {
		for mut todo_file in &mut self.todo_files {
			if filepath == todo_file.filepath {
				remover::remove_todo_by_line(&mut todo_file, line)?;

				return Ok(());
			}
		}

		Err(TodoRError::FileNotTracked {
			filepath: filepath.to_string_lossy().to_string()
		}.into())
	}
}

fn default_comment_types_map() -> HashMap<String, Vec<CommentType>> {
	// TODO: move default CommentTypes into predefined ones in custom_tags
	// MAYB: Use a Box or something to not alloc the same Vec over and over again
	let mut comment_types_map = HashMap::new();

	comment_types_map.insert("rs".to_string(), vec![CommentType::new_one_line("//"), CommentType::new_block("/*", "*/")]);
	comment_types_map.insert("c".to_string(), vec![CommentType::new_one_line("//"), CommentType::new_block("/*", "*/")]);
	comment_types_map.insert("h".to_string(), vec![CommentType::new_one_line("//"), CommentType::new_block("/*", "*/")]);
	comment_types_map.insert("cpp".to_string(), vec![CommentType::new_one_line("//"), CommentType::new_block("/*", "*/")]);
	comment_types_map.insert("cs".to_string(), vec![CommentType::new_one_line("//"), CommentType::new_block("/*", "*/")]);
	comment_types_map.insert("go".to_string(), vec![CommentType::new_one_line("//"), CommentType::new_block("/*", "*/")]);
	comment_types_map.insert("java".to_string(), vec![CommentType::new_one_line("//"), CommentType::new_block("/*", "*/")]);
	comment_types_map.insert("js".to_string(), vec![CommentType::new_one_line("//"), CommentType::new_block("/*", "*/")]);
	comment_types_map.insert("es".to_string(), vec![CommentType::new_one_line("//"), CommentType::new_block("/*", "*/")]);
	comment_types_map.insert("es6".to_string(), vec![CommentType::new_one_line("//"), CommentType::new_block("/*", "*/")]);
	comment_types_map.insert("ts".to_string(), vec![CommentType::new_one_line("//"), CommentType::new_block("/*", "*/")]);
	comment_types_map.insert("tsx".to_string(), vec![CommentType::new_one_line("//"), CommentType::new_block("/*", "*/")]);
	comment_types_map.insert("styl".to_string(), vec![CommentType::new_one_line("//"), CommentType::new_block("/*", "*/")]);
	comment_types_map.insert("swift".to_string(), vec![CommentType::new_one_line("//"), CommentType::new_block("/*", "*/")]);
	comment_types_map.insert("less".to_string(), vec![CommentType::new_one_line("//"), CommentType::new_block("/*", "*/")]);
	comment_types_map.insert("scss".to_string(), vec![CommentType::new_one_line("//"), CommentType::new_block("/*", "*/")]);
	comment_types_map.insert("sass".to_string(), vec![CommentType::new_one_line("//"), CommentType::new_block("/*", "*/")]);
	comment_types_map.insert("m".to_string(), vec![CommentType::new_one_line("//"), CommentType::new_block("/*", "*/")]);
	comment_types_map.insert("mm".to_string(), vec![CommentType::new_one_line("//"), CommentType::new_block("/*", "*/")]);
	comment_types_map.insert("php".to_string(), vec![CommentType::new_one_line("//"), CommentType::new_block("/*", "*/")]);
	comment_types_map.insert("py".to_string(), vec![CommentType::new_one_line("#"), CommentType::new_block("\"\"\"", "\"\"\"")]);
	comment_types_map.insert("rb".to_string(), vec![CommentType::new_one_line("#")]);
	comment_types_map.insert("pl".to_string(), vec![CommentType::new_one_line("#")]);
	comment_types_map.insert("pm".to_string(), vec![CommentType::new_one_line("#")]);
	comment_types_map.insert("coffee".to_string(), vec![CommentType::new_one_line("#")]);
	comment_types_map.insert("tex".to_string(), vec![CommentType::new_one_line("%")]);
	comment_types_map.insert("hs".to_string(), vec![CommentType::new_one_line("--")]);
	comment_types_map.insert("sql".to_string(), vec![CommentType::new_one_line("--")]);
	comment_types_map.insert("html".to_string(), vec![CommentType::new_block("<!--", "-->")]);
	comment_types_map.insert("htm".to_string(), vec![CommentType::new_block("<!--", "-->")]);
	comment_types_map.insert("md".to_string(), vec![CommentType::new_block("<!--", "-->")]);
	comment_types_map.insert("gitignore".to_string(), vec![CommentType::new_one_line("#")]);
	comment_types_map.insert("yaml".to_string(), vec![CommentType::new_one_line("#")]);
	comment_types_map.insert("yml".to_string(), vec![CommentType::new_one_line("#")]);
	comment_types_map.insert("sh".to_string(), vec![CommentType::new_one_line("#")]);
	comment_types_map.insert("bash".to_string(), vec![CommentType::new_one_line("#")]);
	comment_types_map.insert("zsh".to_string(), vec![CommentType::new_one_line("#")]);

	comment_types_map
}