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
use std::borrow::Cow;

use failure::Error;
use errors::TodoRError;
use globset::{Glob, GlobSet, GlobSetBuilder};

use parser::parse_content;
use display::{StyleConfig, write_file_todos, TodoFile};
use comments::{CommentTypes, TodorConfigFileSerial};

static DEFAULT_CONFIG: &str = include_str!("default_config.json");

/// Type for building TodoR with a custom configuration.
pub struct TodoRBuilder {
	override_verbose: Option<bool>,
	// TODO: rename "todo_words" to "tags"
	added_todo_words: Vec<String>,
	override_todo_words: Option<Vec<String>>,
	override_ignore_paths: Option<GlobSetBuilder>,
	override_default_ext: Option<String>,
	styles: StyleConfig,
	// Config from files. Parameters with override_ override inner_config.
	inner_config: config::Config,
}

impl Default for TodoRBuilder {
	/// Creates TodoRBuilder using the default configuration.
	fn default() -> TodoRBuilder {
		let mut inner_config = config::Config::new();
		inner_config.merge(
			config::File::from_str(DEFAULT_CONFIG, config::FileFormat::Json)
		).unwrap();

		TodoRBuilder {
			override_verbose: None,
			added_todo_words: Vec::new(),
			override_todo_words: None,
			override_ignore_paths: None,
			override_default_ext: None,
			styles: StyleConfig::default(),
			inner_config,
		}
	}
}

impl TodoRBuilder {
	/// Creates TodoRBuilder using the default configuration.
	pub fn new() -> TodoRBuilder {
		TodoRBuilder::default()
	}

	/// Creates TodoRBuilder with no configuration.
	pub fn with_no_config() -> TodoRBuilder {
		TodoRBuilder {
			inner_config: config::Config::new(),
			..Default::default()
		}
	}

	/// Consumes self and builds TodoR.
	pub fn build(self) -> Result<TodoR, Error> {
		let mut config_struct: TodorConfigFileSerial = self.inner_config.try_into()?;

		let verbose = self.override_verbose.unwrap_or_else(|| config_struct.verbose);
		let mut todo_words = self.override_todo_words.unwrap_or_else(|| config_struct.tags.to_owned());
		todo_words.append(&mut self.added_todo_words.clone());

		let ignore_paths = match self.override_ignore_paths {
			Some(glob_builder) => glob_builder.build()?,
			None => {
				let mut gb = GlobSetBuilder::new();
				for path in config_struct.ignore {
					gb.add(Glob::new(&path)?);
				}
				gb.build()?
			}
		};

		if verbose {
			println!("TODO tags: {}", todo_words.join(", ").to_uppercase());
		}

		let mut ext_to_comment_types: HashMap<String, CommentTypes> = HashMap::new();

		for comment_config in config_struct.comments.drain(..) {
			let (ext, mut exts, comment_types) = comment_config.break_apart();

			for extt in exts.drain(..) {
				ext_to_comment_types.insert(extt, comment_types.clone());
			}
			ext_to_comment_types.insert(ext, comment_types);
		}

		// TODO: add default_ext to config file
		let default_comment_types = match self.override_default_ext {
			Some(default_ext) => {ext_to_comment_types.get(&default_ext)
				.ok_or(TodoRError::InvalidExtension {ext: default_ext})?
				.clone()
			},
			None => CommentTypes::new().add_single("#"),
		};

		let config = TodoRConfig {
			verbose,
			todo_words,
			ignore_paths,
			styles: self.styles,
			ext_to_comment_types,
			default_comment_types,
		};


		Ok(TodoR::with_config(config))
	}

	/// Adds config file for TodoR.
	pub fn add_config_file(&mut self, config_path: &Path) -> Result<&mut Self, Error> {
		self.inner_config.merge(config::File::from(config_path))?;
		Ok(self)
	}

	/// Adds tag for TodoR to look for without overriding tags from config files.
	pub fn add_todo_word<'a, S: Into<Cow<'a, str>>>(&mut self, tag: S) -> &mut Self {
		self.added_todo_words.push(tag.into().into_owned());
		self
	}

	/// Adds tags for TodoR to look for without overriding tags from config files.
	pub fn add_todo_words<'a, I, S>(&mut self, tags: I) -> &mut Self 
	where
		I: IntoIterator<Item = S>,
		S: Into<Cow<'a, str>>,
	{
		self.added_todo_words.extend(
			tags.into_iter()
			.map(|s| s.into().into_owned())
		);
		self
	}

	/// Adds tag for TodoR to look for. This overrides tags from config files.
	pub fn add_override_todo_word<'a, S: Into<Cow<'a, str>>>(&mut self, tag: S) -> &mut Self {
		self.override_todo_words.get_or_insert_with(Vec::new)
			.push(tag.into().into_owned());
		self
	}

	/// Adds tags for TodoR to look for. This overrides tags from config files.
	pub fn add_override_todo_words<'a, I, S>(&mut self, tags: I) -> &mut Self 
	where
		I: IntoIterator<Item = S>,
		S: Into<Cow<'a, str>>,
	{
		{
			let tws = self.override_todo_words.get_or_insert_with(Vec::new);

			tws.extend(
				tags.into_iter()
				.map(|s| s.into().into_owned())
			)
		}
		self
	}

	/// Overrides verbose from config files.
	pub fn set_verbose(&mut self, verbose: bool) -> &mut Self {
		self.override_verbose = Some(verbose);
		self
	}

	/// Sets the terminal output of TodoR to be with no styles.
	pub fn set_no_style(&mut self) -> &mut Self {
		self.styles = StyleConfig::no_style();
		self
	}

	/// Adds path for TodoR to ignore. This overrides ignore paths from config files.
	pub fn add_override_ignore_path(&mut self, path: &str) -> Result<&mut Self, Error> {
		let new_glob = Glob::new(path)?;
		self.override_ignore_paths.get_or_insert_with(GlobSetBuilder::new)
			.add(new_glob);
		Ok(self)
	}

	/// Adds paths for TodoR to ignore. This overrides ignore paths from config files.
	pub fn add_override_ignore_paths<I, S>(&mut self, paths: I) -> Result<&mut Self , Error>
	where
		I: IntoIterator<Item = S>,
		S: AsRef<str>,
	{
		for path in paths {
			self.add_override_ignore_path(path.as_ref())?;
		}
		Ok(self)
	}

	/// Sets the default fall-back extension for comments.
	///
	/// For instance if you want to parse unknown extensions using C style comments,
	/// use `builder.set_default_ext("c")`.
	pub fn set_default_ext<'a, S: Into<Cow<'a, str>>>(&mut self, ext: S) -> Result<(), Error> {
		self.override_default_ext = Some(ext.into().into_owned());

		Ok(())
	}

	/// Writes the default configuration file to out_buffer.
	pub fn write_default_config(out_buffer: &mut Write) -> Result<(), Error> {
		out_buffer.write_all(DEFAULT_CONFIG.as_bytes())?;
		Ok(())
	}
}

/// Configuration for `TodoR`.
///
/// `verbose` holds whether to print extra content.
/// `todo_words` gives a list of the TODO terms to search for.
struct TodoRConfig {
	verbose: bool,
	todo_words: Vec<String>,
	styles: StyleConfig,
	ignore_paths: GlobSet,
	ext_to_comment_types: HashMap<String, CommentTypes>,
	default_comment_types: CommentTypes,
}

/// Parser for finding TODOs in comments and storing them on a per-file basis.
pub struct TodoR {
	config: TodoRConfig,
	todo_files: Vec<TodoFile>,
}

impl Default for TodoR {
	fn default() -> TodoR {
		TodoRBuilder::default().build().unwrap()
	}
}

impl TodoR {
	/// Creates new TodoR that looks for provided keywords.
	pub fn new() -> TodoR {
		TodoR::default()
	}

	pub fn with_todo_words<'a, I, S>(todo_words: I) -> TodoR
	where
		I: IntoIterator<Item = S>,
		S: Into<Cow<'a, str>>,
	{
		let mut builder = TodoRBuilder::default();
		builder.add_override_todo_words(todo_words);
		builder.build().unwrap()
	}

	/// Creates new TodoR using given configuration.
	fn with_config(config: TodoRConfig) -> TodoR {
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