#[macro_use] extern crate failure;

extern crate regex;
extern crate ansi_term;

mod parser;
mod display;
mod custom_tags;

pub mod errors {
	use failure::Error;

	#[derive(Debug, Fail)]
	pub enum TodoRError {
		#[fail(display = "'{}' is a directory.", filename)]
		FileIsDir {
			filename: String,
		},
	}

	use ansi_term::Colour::Red;

	pub fn eprint_error(err: &Error) {
		match err {
			_ => eprintln!("{}: {}", Red.paint("[todo_r error]"), err.to_string()),
		};
	}
}

use std::fs::File;
use std::io::{self, Write, BufReader, Cursor};
use std::collections::HashMap;

use errors::TodoRError;
use failure::Error;

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
	pub fn new<T: AsRef<str>>(todo_words: &[T]) -> TodoRConfig {
		let todo_word_strings: Vec<String> = todo_words.iter().map(|s| s.as_ref().to_string()).collect();
		
		TodoRConfig {
			verbose: false,
			todo_words: todo_word_strings,
			styles: StyleConfig::default(),
			ext_to_comment_types: default_comment_types_map(),
			default_comment_types: vec![CommentType::new_one_line("#")],
		}
	}

	pub fn set_no_style(&mut self) {
		self.styles = StyleConfig::no_style();
	}
}

/// Parser for finding TODOs in comments and storing them on a per-file basis.
pub struct TodoR {
	pub config: TodoRConfig,
	todo_files: Vec<TodoFile>,
}

impl TodoR {
	/// Creates new TodoR that looks for provided keywords.
	pub fn new<T: AsRef<str>>(todo_words: &[T]) -> TodoR {
		TodoR {
			config: TodoRConfig::new(todo_words),
			todo_files: Vec::new(),
		}
	}

	/// Opens file at given filename and process it by finding all its TODOs.
	pub fn open_todos(&mut self, filename: &str) -> Result<(), Error> {
		let mut todo_file = TodoFile::new(filename);
		let file_ext = filename.rsplitn(2, '.').next().unwrap();
		let comment_types = self.config.ext_to_comment_types.get(file_ext).unwrap_or(&self.config.default_comment_types);
		
		let file = File::open(filename)?;
		// check the file is not a directory
		if file.metadata()?.is_dir() {
			return Err(TodoRError::FileIsDir {
				filename: filename.to_string()
			}.into());
		}

		let mut file_reader = BufReader::new(file);
		todo_file.set_todos(parse_content(&mut file_reader, &comment_types, &self.config.todo_words)?);

		self.todo_files.push(todo_file);
		Ok(())
	}

	/// Finds TODO comments in the given content
	pub fn find_todos(&mut self, content: &str) -> Result<(), Error> {
		let mut todo_file = TodoFile::new("");
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