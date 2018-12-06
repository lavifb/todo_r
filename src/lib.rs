#[macro_use] extern crate failure;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate regex;
extern crate fnv;
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
		#[fail(display = "'{}' is a directory", filepath)]
		InputIsDir {
			filepath: String,
		},
		/// Error for when provided file cannot be accessed for some reason
		#[fail(display = "cannot access '{}'", filepath)]
		CannotAccessFile {
			filepath: String,
		},
		/// Error for when provided file extension is not supported
		#[fail(display = "'{}' is an invalid extension", ext)]
		InvalidExtension {
			ext: String,
		},
		/// Error for when provided filepath for modification is not tracked
		#[fail(display = "'{}' is not a tracked file", filepath)]
		FileNotTracked {
			filepath: String,
		},
		/// Error for when provided TODO line is not found
		#[fail(display = "TODO comment not found in line {}", line)]
		TodoNotFound {
			line: usize
		},
		/// Error for when provided default file extension is not supported
		#[fail(display = "'{}' is an invalid default extension", ext)]
		InvalidDefaultExtension {
			ext: String,
		},
		/// Error for invalid config file.
		#[fail(display = "invalid config file: {}", message)]
		InvalidConfigFile {
			message: String,
		},
		/// Error for invalid ignore path.
		#[fail(display = "invalid ignore path: {}", message)]
		InvalidIgnorePath {
			message: String,
		},
	}

	use ansi_term::Colour::Red;

	/// Prints err to stderr
	pub fn eprint_error(err: &Error) {
		match err {
			_ => eprintln!("{}: {}", Red.paint("[todor error]"), err.to_string()),
		};
	}
}

use std::fs::File;
use std::path::Path;
use std::ffi::OsStr;
use std::io::{self, Write, BufReader, Cursor};
use std::borrow::Cow;

use failure::Error;
use errors::TodoRError;
use fnv::FnvHashMap;
use globset::{Glob, GlobSet, GlobSetBuilder};

use parser::parse_content;
use display::{StyleConfig, write_file_todos, TodoFile};
use comments::{CommentTypes, TodorConfigFileSerial};

static DEFAULT_CONFIG: &str = include_str!("default_config.json");
static EXAMPLE_CONFIG: &str = include_str!("example_config.hjson");

/// Type for building TodoR with a custom configuration.
#[derive(Debug, Default, Clone)]
pub struct TodoRBuilder {
	override_verbose: Option<bool>,
	added_tags: Vec<String>,
	override_tags: Option<Vec<String>>,
	override_ignore_paths: Option<GlobSetBuilder>,
	override_default_ext: Option<String>,
	styles: StyleConfig,
	// Config from files. Parameters with override_ override inner_config.
	inner_config: config::Config,
}

impl TodoRBuilder {
	/// Creates TodoRBuilder using the default configuration.
	pub fn new() -> TodoRBuilder {
		let mut builder = TodoRBuilder::default();
		builder.inner_config.merge(
			config::File::from_str(DEFAULT_CONFIG, config::FileFormat::Json)
		).unwrap();

		builder
	}

	/// Creates TodoRBuilder with no configuration.
	pub fn with_no_config() -> TodoRBuilder {
		TodoRBuilder::default()
	}

	/// Consumes self and builds TodoR.
	pub fn build(self) -> Result<TodoR, Error> {
		let mut config_struct: TodorConfigFileSerial = self.inner_config.try_into()
			.map_err(|err| TodoRError::InvalidConfigFile{message: err.to_string()})?;

		let verbose = self.override_verbose.unwrap_or_else(|| config_struct.verbose);
		let mut tags = self.override_tags.unwrap_or_else(|| config_struct.tags.to_owned());
		let default_ext = self.override_default_ext.unwrap_or_else(|| config_struct.default_ext.to_owned());
		tags.append(&mut self.added_tags.clone());

		let ignore_paths = match self.override_ignore_paths {
			Some(glob_builder) => glob_builder.build()?,
			None => {
				let mut gb = GlobSetBuilder::new();
				for path in config_struct.ignore {
					gb.add(
						Glob::new(&path)
						.map_err(|err| TodoRError::InvalidConfigFile{message: format!("invalid ignore path: {}", err)})?
					);
				}
				gb.build()?
			}
		};

		if verbose {
			println!("TODO tags: {}", tags.join(", ").to_uppercase());
		}

		let mut ext_to_comment_types: FnvHashMap<String, CommentTypes> = FnvHashMap::default();

		// Put default comment types in hashmap.
		for comment_config in config_struct.default_comments.drain(..)
			// Put config comment types in hashmap.
			.chain(config_struct.comments.drain(..)) 
		{
			let (ext, mut exts, comment_types) = comment_config.break_apart();

			for extt in exts.drain(..) {
				ext_to_comment_types.insert(extt, comment_types.clone());
			}
			ext_to_comment_types.insert(ext, comment_types);
		}

		let default_comment_types = ext_to_comment_types.get(&default_ext)
			.ok_or(TodoRError::InvalidDefaultExtension {ext: default_ext})?
			.clone();

		let config = TodoRConfig {
			verbose,
			tags,
			ignore_paths,
			styles: self.styles,
			ext_to_comment_types,
			default_comment_types,
		};


		Ok(TodoR::with_config(config))
	}

	/// Adds config file for TodoR.
	pub fn add_config_file(&mut self, config_path: &Path) -> Result<&mut Self, Error> {
		let mut merge_file = config::File::from(config_path);
		if config_path.file_name() == Some(OsStr::new(".todor")) {
			merge_file = merge_file.format(config::FileFormat::Hjson);
		}
		self.inner_config.merge(merge_file)?;
		Ok(self)
	}

	/// Adds tag for TodoR to look for without overriding tags from config files.
	pub fn add_todo_word<'a, S: Into<Cow<'a, str>>>(&mut self, tag: S) -> &mut Self {
		self.added_tags.push(tag.into().into_owned());
		self
	}

	/// Adds tags for TodoR to look for without overriding tags from config files.
	pub fn add_tags<'a, I, S>(&mut self, tags: I) -> &mut Self 
	where
		I: IntoIterator<Item = S>,
		S: Into<Cow<'a, str>>,
	{
		self.added_tags.extend(
			tags.into_iter()
			.map(|s| s.into().into_owned())
		);
		self
	}

	/// Adds tag for TodoR to look for. This overrides tags from config files.
	pub fn add_override_todo_word<'a, S: Into<Cow<'a, str>>>(&mut self, tag: S) -> &mut Self {
		self.override_tags.get_or_insert_with(Vec::new)
			.push(tag.into().into_owned());
		self
	}

	/// Adds tags for TodoR to look for. This overrides tags from config files.
	pub fn add_override_tags<'a, I, S>(&mut self, tags: I) -> &mut Self 
	where
		I: IntoIterator<Item = S>,
		S: Into<Cow<'a, str>>,
	{
		{
			let tws = self.override_tags.get_or_insert_with(Vec::new);

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
	// MAYB: use ignore crate instead of globset
	pub fn add_override_ignore_path(&mut self, path: &str) -> Result<&mut Self, Error> {
		let new_glob = Glob::new(path)
			.map_err(|err| TodoRError::InvalidIgnorePath{message: err.to_string()})?;
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
	pub fn write_example_config(out_buffer: &mut impl Write) -> Result<(), Error> {
		out_buffer.write_all(EXAMPLE_CONFIG.as_bytes())?;
		Ok(())
	}
}

/// Configuration for `TodoR`.
///
/// `verbose` holds whether to print extra content.
/// `tags` gives a list of the TODO terms to search for.
struct TodoRConfig {
	verbose: bool,
	tags: Vec<String>,
	styles: StyleConfig,
	ignore_paths: GlobSet,
	ext_to_comment_types: FnvHashMap<String, CommentTypes>,
	default_comment_types: CommentTypes,
}

/// Parser for finding TODOs in comments and storing them on a per-file basis.
pub struct TodoR {
	config: TodoRConfig,
	todo_files: Vec<TodoFile>,
}

impl Default for TodoR {
	fn default() -> TodoR {
		TodoRBuilder::new().build().unwrap()
	}
}

impl TodoR {
	/// Creates new TodoR that looks for provided keywords.
	pub fn new() -> TodoR {
		TodoR::default()
	}

	pub fn with_tags<'a, I, S>(tags: I) -> TodoR
	where
		I: IntoIterator<Item = S>,
		S: Into<Cow<'a, str>>,
	{
		let mut builder = TodoRBuilder::new();
		builder.add_override_tags(tags);
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
		if !filepath.is_file() {
			if filepath.is_dir() {
				return Err(TodoRError::InputIsDir {
					filepath: filepath.to_string_lossy().to_string()
				}.into());
			} else {
				return Err(TodoRError::CannotAccessFile {
					filepath: filepath.to_string_lossy().to_string()
				}.into());
			}
		}

		let file_ext = filepath.extension().unwrap_or_else(|| OsStr::new(".sh"));
		let comment_types = self.config.ext_to_comment_types.get(file_ext.to_str().unwrap())
								.unwrap_or(&self.config.default_comment_types);

		let file = File::open(filepath)?;
		let mut file_reader = BufReader::new(file);
		todo_file.set_todos(parse_content(&mut file_reader, &comment_types, &self.config.tags)?);

		self.todo_files.push(todo_file);
		Ok(())
	}

	/// Finds TODO comments in the given content
	pub fn find_todos(&mut self, content: &str) -> Result<(), Error> {
		let mut todo_file = TodoFile::new(Path::new(""));
		let mut content_buf = Cursor::new(content);
		todo_file.set_todos(parse_content(&mut content_buf, &self.config.default_comment_types, &self.config.tags)?);

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