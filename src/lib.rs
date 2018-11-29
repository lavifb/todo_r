#[macro_use] extern crate failure;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate regex;
extern crate ansi_term;
extern crate config;
extern crate globset;

mod parser;
mod display;
mod custom_tags;
mod remover;
pub mod comments;

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
use globset::{Glob, GlobSet, GlobSetBuilder};

use parser::parse_content;
use display::{StyleConfig, write_file_todos, TodoFile};
use comments::{CommentTypes, TodorConfigFileSerial};

static DEFAULT_CONFIG: &str = include_str!("default_config.json");

// TODO: add doc comments
pub struct TodoRBuilder {
	pub override_verbose: Option<bool>,
	pub override_todo_words: Option<Vec<String>>,
	override_ignore_paths: Option<GlobSetBuilder>,
	styles: StyleConfig,
	// Config from files. Parameters above override inner_config.
	inner_config: config::Config,
}

impl Default for TodoRBuilder {
	// TODO: add doc comments
	fn default() -> TodoRBuilder {
		let mut inner_config = config::Config::new();
		inner_config.merge(
			config::File::from_str(DEFAULT_CONFIG, config::FileFormat::Json)
		).unwrap();

		TodoRBuilder {
			override_verbose: None,
			override_todo_words: None,
			override_ignore_paths: None,
			inner_config,
			styles: StyleConfig::default(),
		}
	}
}

impl TodoRBuilder {
	// TODO: add doc comments
	pub fn new() -> TodoRBuilder {
		TodoRBuilder {
			inner_config: config::Config::new(),
			..Default::default()
		}
	}

	pub fn with_todo_words<S: ToString>(todo_words: &[S]) -> TodoRBuilder {
		let todo_word_strings: Vec<String> = todo_words
			.iter()
			.map(|s| s.to_string())
			.collect();
		
		TodoRBuilder {
			override_todo_words: Some(todo_word_strings),
			..Default::default()
		}
	}

	// TODO: add doc comments
	pub fn build(self) -> TodoR {
		unimplemented!();
	}

	// TODO: add doc comments
	pub fn add_ignore_path(&mut self, path: &str) -> Result<&mut Self, Error> {
		let new_glob = Glob::new(path)?;
		self.override_ignore_paths.get_or_insert_with(|| GlobSetBuilder::new())
			.add(new_glob);
		Ok(self)
	}

	// TODO: add doc comments
	pub fn add_config_file(&mut self, config_path: &Path) -> Result<&mut Self, Error> {
		self.inner_config.merge(config::File::from(config_path))?;
		Ok(self)
	}
}

/// Configuration for `TodoR`.
///
/// `verbose` holds whether to print extra content.
/// `todo_words` gives a list of the TODO terms to search for.
pub struct TodoRConfig {
	pub verbose: bool,
	pub todo_words: Vec<String>,
	inner_config: config::Config,
	styles: StyleConfig,
	ignore_paths: GlobSet,
	ext_to_comment_types: HashMap<String, CommentTypes>,
	default_comment_types: CommentTypes,
}

impl TodoRConfig {
	/// Creates new TodoR configuration with the default parameters.
	pub fn new() -> TodoRConfig {
		TodoRConfig {
			..Default::default()
		}
	}

	/// Creates new TodoR configuration with the given TODO comment types.
	pub fn with_todo_words<S: ToString>(todo_words: &[S]) -> TodoRConfig {
		let todo_word_strings: Vec<String> = todo_words
			.iter()
			.map(|s| s.to_string())
			.collect();
		
		TodoRConfig {
			todo_words: todo_word_strings,
			..Default::default()
		}
	}

	/// Creates new TodoR configuration from the given configuration file.
	/// Note that this config overrides the defaults.
	pub fn from_config_file(config_path: &Path) -> Result<TodoRConfig, Error> {
		let mut config_from_file = config::Config::new();
		config_from_file.merge(config::File::from(config_path))?;

		let mut config = TodoRConfig {
			inner_config: config_from_file,
			..Default::default()
		};
		config.reload_config()?;

		Ok(config)
	}

	/// Creates new TodoR configuration from the given configuration file with defaults as a fall-back.
	pub fn default_with_config_file(config_path: &Path) -> Result<TodoRConfig, Error> {
		let mut inner_config = config::Config::new();
		inner_config.merge(config::File::from_str(include_str!("default_config.json"), config::FileFormat::Json)).unwrap();
		inner_config.merge(config::File::from(config_path))?;

		let mut config = TodoRConfig {
			inner_config,
			..Default::default()
		};
		config.reload_config()?;

		Ok(config)
	}

	/// Merges configuration file into the configuration.
	pub fn merge_config_file(&mut self, config_path: &Path) -> Result<(), Error> {
		self.inner_config.merge(config::File::from(config_path))?;
		self.reload_config()?;

		Ok(())
	}

	// TODO: Use a TodoRConfigBuilder struct
	/// Parses and loads inner_config. Use after merging into inner_config.
	fn reload_config(&mut self) -> Result<(), Error> {
		let inner_config = self.inner_config.clone();
		let config_struct: TodorConfigFileSerial = inner_config.try_into()?;

		self.todo_words = config_struct.tags;
		self.set_ignore_paths(&config_struct.ignore)?;

		for comment_config in config_struct.comments {
			// TODO: deal with error
			let exts = comment_config.exts.to_owned();
			let ext  = comment_config.ext.to_owned();
			let comment_types = CommentTypes::from_config(comment_config);

			self.set_exts_comment_types(&exts, comment_types.clone());
			self.set_ext_comment_types(&ext, comment_types);
		}

		Ok(())
	}

	/// Writes the default configuration file to out_buffer.
	pub fn write_default_config(out_buffer: &mut Write) -> Result<(), Error> {
		out_buffer.write_all(DEFAULT_CONFIG.as_bytes())?;
		Ok(())
	}

	/// Sets output to be without colors or styles.
	pub fn set_no_style(&mut self) {
		self.styles = StyleConfig::no_style();
	}

	/// Sets the list of paths that will be ignored.
	/// These paths can include globs such as `*.rs`, `src/**/*.rs`, or `src/**`.
	///  
	// TODO: allow dirs in ignore_paths
	/// Note that listing just the directory (ex: `src/`) does not work. 
	/// You must add the `**` to make `src/**`.
	pub fn set_ignore_paths<S: AsRef<str>>(&mut self, ignore_paths: &[S]) -> Result<(), Error> {
		let mut glob_builder = GlobSetBuilder::new();

		for path in ignore_paths {
			glob_builder.add(Glob::new(path.as_ref())?);
		}

		self.ignore_paths = glob_builder.build()?;
		Ok(())
	}

	/// Sets the default fall-back extension for comments.
	///
	/// For instance if you want to parse unknown extensions using C style comments,
	/// use `todor.set_default_ext("c")`.
	pub fn set_default_ext(&mut self, ext: &str) -> Result<(), Error> {
		self.default_comment_types = self.ext_to_comment_types.get(ext).ok_or(
			TodoRError::InvalidExtension {
				ext: ext.to_string()
			}
		).unwrap().clone();

		Ok(())
	}

	// TODO: use Cow<str>
	/// Sets the comment tokens for the provided extension.
	pub fn set_ext_comment_types(&mut self, ext: &str, comment_types: CommentTypes) {
		self.ext_to_comment_types.insert(ext.to_string(), comment_types);
	}

	/// Sets the comment tokens for the list of provided extensions.
	pub fn set_exts_comment_types<S: ToString>(&mut self, exts: &[S], comment_types: CommentTypes) {
		for ext in exts {
			self.ext_to_comment_types.insert(ext.to_string(), comment_types.clone());
		}
	}
}

impl Default for TodoRConfig {
	fn default() -> TodoRConfig {
		let mut inner_config = config::Config::new();
		inner_config.merge(config::File::from_str(DEFAULT_CONFIG, config::FileFormat::Json)).unwrap();

		let mut config = TodoRConfig {
			verbose: false,
			todo_words: Vec::new(),
			inner_config,
			styles: StyleConfig::default(),
			ignore_paths: GlobSet::empty(),
			ext_to_comment_types: HashMap::new(),
			default_comment_types: CommentTypes::new().add_single("#"),
		};

		config.reload_config().unwrap();
		config
	}
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
			..Default::default()
		}
	}

	pub fn with_todo_words<S: ToString>(todo_words: &[S]) -> TodoR {
		TodoR {
			config: TodoRConfig::with_todo_words(todo_words),
			..Default::default()
		}
	}

	/// Creates new TodoR using given configuration.
	pub fn with_config(config: TodoRConfig) -> TodoR {
		TodoR {
			config,
			..Default::default()
		}
	}

	/// Returns the number of files currently tracked by TodoR
	pub fn num_files(&self) -> usize {
		self.todo_files.len()
	}

	/// Returns all tracked files that contain TODOs
	pub fn get_tracked_files<'a>(&'a self) -> Vec<&'a str> {
		self.todo_files.iter()
			.filter(|tf| !tf.todos.is_empty())
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

		if self.config.ignore_paths.is_match(filepath) {
			return Ok(());
		}

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

impl Default for TodoR {
	fn default() -> TodoR {
		TodoR {
			config: TodoRConfig::new(),
			todo_files: Vec::new(),
		}
	}
}