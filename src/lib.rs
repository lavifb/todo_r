pub mod comments;
mod custom_tags;
mod display;
mod parser;
mod remover;
pub mod todo;

pub mod errors {
    use failure::Fail;

    /// Custom Errors for TodoR
    #[derive(Debug, Fail)]
    pub enum TodoRError {
        /// Error for when provided file path is a directory.
        #[fail(display = "'{}' is a directory", filepath)]
        InputIsDir { filepath: String },
        /// Error for when provided file cannot be accessed for some reason.
        #[fail(display = "cannot access '{}'", filepath)]
        CannotAccessFile { filepath: String },
        /// Error for when provided file extension is not supported.
        #[fail(display = "'{}' is an invalid extension", ext)]
        InvalidExtension { ext: String },
        /// Error for when provided filepath for modification is not tracked.
        #[fail(display = "'{}' is not a tracked file", filepath)]
        FileNotTracked { filepath: String },
        /// Error for when provided TODO line is not found.
        #[fail(display = "TODO comment not found in line {}", line)]
        TodoNotFound { line: usize },
        /// Error for when provided default file extension is not supported.
        #[fail(display = "'{}' is an invalid default extension", ext)]
        InvalidDefaultExtension { ext: String },
        /// Error for invalid config file.
        #[fail(display = "invalid config file: {}", message)]
        InvalidConfigFile { message: String },
        /// Error for invalid ignore path.
        #[fail(display = "invalid ignore path: {}", message)]
        InvalidIgnorePath { message: String },
    }
}

use std::borrow::Cow;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, BufReader, Cursor, Write};
use std::path::Path;

use failure::Error;
use fnv::FnvHashMap;
use globset::{Glob, GlobSet, GlobSetBuilder};
use log::debug;
use regex::Regex;

use crate::comments::{CommentTypes, TodorConfigFileSerial};
use crate::display::*;
use crate::errors::TodoRError;
use crate::parser::{build_parser_regexs, parse_content};
use crate::todo::{Todo, TodoFile};

static DEFAULT_CONFIG: &str = include_str!("default_config.json");
static EXAMPLE_CONFIG: &str = include_str!("example_config.hjson");

/// A builder to create a TodoR with a custom configuration.
/// Customization occurs in two forms: using manual functions and adding config files.
///
/// ### Functions
/// Use functions such as `add_tag()` to add to the config.
/// Functions that have `override` in them will fully override settings from config files.
///
/// ### Config files
/// Config files are added using `add_config_file()`.
///
/// For an example config file, use `todo_r::write_example_config()`.

#[derive(Debug, Default, Clone)]
pub struct TodoRBuilder {
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
        builder
            .inner_config
            .merge(config::File::from_str(
                DEFAULT_CONFIG,
                config::FileFormat::Json,
            ))
            .unwrap();

        builder
    }

    /// Creates TodoRBuilder with no configuration.
    pub fn with_no_config() -> TodoRBuilder {
        TodoRBuilder::default()
    }

    /// Consumes self and builds TodoR.
    pub fn build<'a>(self) -> Result<TodoR<'a>, Error> {
        let mut config_struct: TodorConfigFileSerial =
            self.inner_config
                .try_into()
                .map_err(|err| TodoRError::InvalidConfigFile {
                    message: err.to_string(),
                })?;

        let mut tags = self
            .override_tags
            .unwrap_or_else(|| config_struct.tags.to_owned());
        let default_ext = self
            .override_default_ext
            .unwrap_or_else(|| config_struct.default_ext.to_owned());
        tags.append(&mut self.added_tags.clone());

        let ignore_paths = match self.override_ignore_paths {
            Some(glob_builder) => glob_builder.build()?,
            None => {
                let mut gb = GlobSetBuilder::new();
                for path in config_struct.ignore {
                    gb.add(
                        Glob::new(&path).map_err(|err| TodoRError::InvalidConfigFile {
                            message: format!("invalid ignore path: {}", err),
                        })?,
                    );
                }
                gb.build()?
            }
        };

        let mut ext_to_comment_types: FnvHashMap<String, CommentTypes> = FnvHashMap::default();

        // Put default comment types in hashmap.
        for comment_config in config_struct
            .default_comments
            .drain(..)
            // Put config comment types in hashmap.
            .chain(config_struct.comments.drain(..))
        {
            let (ext, mut exts, comment_types) = comment_config.break_apart();

            for extt in exts.drain(..) {
                ext_to_comment_types.insert(extt, comment_types.clone());
            }
            ext_to_comment_types.insert(ext, comment_types);
        }

        let default_comment_types = ext_to_comment_types
            .get(&default_ext)
            .ok_or(TodoRError::InvalidDefaultExtension { ext: default_ext })?
            .clone();

        let config = TodoRConfig {
            tags,
            ignore_paths,
            styles: self.styles,
            ext_to_comment_types,
            default_comment_types,
        };

        debug!("todor parser built: {:?}", config);

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
    pub fn add_tag<'a, S: Into<Cow<'a, str>>>(&mut self, tag: S) -> &mut Self {
        self.added_tags.push(tag.into().into_owned());
        self
    }

    /// Adds tags for TodoR to look for without overriding tags from config files.
    pub fn add_tags<'a, I, S>(&mut self, tags: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Cow<'a, str>>,
    {
        self.added_tags
            .extend(tags.into_iter().map(|s| s.into().into_owned()));
        self
    }

    /// Adds tag for TodoR to look for. This overrides tags from config files.
    pub fn add_override_tag<'a, S: Into<Cow<'a, str>>>(&mut self, tag: S) -> &mut Self {
        self.override_tags
            .get_or_insert_with(Vec::new)
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

            tws.extend(tags.into_iter().map(|s| s.into().into_owned()))
        }
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
        let new_glob = Glob::new(path).map_err(|err| TodoRError::InvalidIgnorePath {
            message: err.to_string(),
        })?;
        self.override_ignore_paths
            .get_or_insert_with(GlobSetBuilder::new)
            .add(new_glob);
        Ok(self)
    }

    /// Adds paths for TodoR to ignore. This overrides ignore paths from config files.
    pub fn add_override_ignore_paths<I, S>(&mut self, paths: I) -> Result<&mut Self, Error>
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
}

/// Writes the default configuration file to out_buffer.
pub fn write_example_config(out_buffer: &mut impl Write) -> Result<(), Error> {
    out_buffer.write_all(EXAMPLE_CONFIG.as_bytes())?;
    Ok(())
}

/// Configuration for `TodoR`.
///
/// `tags` gives a list of the TODO terms to search for.
#[derive(Debug, Clone)]
struct TodoRConfig {
    tags: Vec<String>,
    styles: StyleConfig,
    ignore_paths: GlobSet,
    ext_to_comment_types: FnvHashMap<String, CommentTypes>,
    default_comment_types: CommentTypes,
}

/// Parser for finding TODOs in comments and storing them on a per-file basis.
#[derive(Debug, Clone)]
pub struct TodoR<'a> {
    config: TodoRConfig,
    todo_files: Vec<TodoFile<'a>>,
    ext_to_regexs: FnvHashMap<String, Vec<Regex>>,
}

impl<'a> Default for TodoR<'a> {
    fn default() -> TodoR<'a> {
        TodoRBuilder::new().build().unwrap()
    }
}

impl<'a> TodoR<'a> {
    /// Creates new TodoR that looks for provided keywords.
    pub fn new<'b>() -> TodoR<'b> {
        TodoR::default()
    }

    pub fn with_tags<'b, I, S>(tags: I) -> TodoR<'b>
    where
        I: IntoIterator<Item = S>,
        S: Into<Cow<'b, str>>,
    {
        let mut builder = TodoRBuilder::new();
        builder.add_override_tags(tags);
        builder.build().unwrap()
    }

    /// Creates new TodoR using given configuration.
    fn with_config<'b>(config: TodoRConfig) -> TodoR<'b> {
        TodoR {
            config,
            todo_files: Vec::new(),
            ext_to_regexs: FnvHashMap::default(),
        }
    }

    /// Returns the number of files currently tracked by TodoR
    pub fn num_files(&self) -> usize {
        self.todo_files.len()
    }

    /// Returns the number of TODOs currently tracked by TodoR
    pub fn num_todos(&self) -> usize {
        self.todo_files.iter().fold(0, |s, tf| s + tf.todos.len())
    }

    /// Returns all tracked files that contain TODOs
    pub fn get_tracked_files(&self) -> Vec<&str> {
        self.todo_files
            .iter()
            .filter(|tf| !tf.todos.is_empty())
            .map(|tf| tf.filepath.to_str().unwrap())
            .collect()
    }

    /// Returns all tracked files even if they have no TODOs
    pub fn get_all_tracked_files<'b>(&'b self) -> Vec<&'b str> {
        self.todo_files
            .iter()
            .map(|tf| tf.filepath.to_str().unwrap())
            .collect()
    }

    /// Returns the parser regexs for the provided extension.
    /// Results are cached so regexes do not have to be rebuilt.
    fn get_parser_regexs(&mut self, ext: impl AsRef<str>) -> &Vec<Regex> {
        let config = &self.config;

        self.ext_to_regexs
            .entry(ext.as_ref().to_string())
            .or_insert_with(|| {
                debug!(
                    "Regexs for `{}` not found. Building regexs...",
                    ext.as_ref()
                );
                let comment_types = config
                    .ext_to_comment_types
                    .get(ext.as_ref())
                    .unwrap_or(&config.default_comment_types);

                build_parser_regexs(comment_types, &config.tags)
            })
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
                    filepath: filepath.to_string_lossy().to_string(),
                }
                .into());
            } else {
                return Err(TodoRError::CannotAccessFile {
                    filepath: filepath.to_string_lossy().to_string(),
                }
                .into());
            }
        }

        let file_ext = filepath
            .extension()
            .unwrap_or_else(|| OsStr::new(".sh"))
            .to_str()
            .unwrap();
        let parser_regexs = self.get_parser_regexs(file_ext);

        let file = File::open(filepath)?;
        let mut file_reader = BufReader::new(file);
        todo_file.set_todos(parse_content(&mut file_reader, &parser_regexs)?);

        debug!(
            "found {} TODOs in `{}`",
            todo_file.len(),
            filepath.display()
        );

        self.todo_files.push(todo_file);
        Ok(())
    }

    // MAYB: make open_user_todos() that only searches for todos with user tagged

    /// Finds TODO comments in the given content
    pub fn find_todos(&mut self, content: &str) -> Result<(), Error> {
        let mut todo_file = TodoFile::new(Path::new(""));
        let mut content_buf = Cursor::new(content);
        let parser_regexs = self.get_parser_regexs(".sh");

        todo_file.set_todos(parse_content(&mut content_buf, &parser_regexs)?);

        self.todo_files.push(todo_file);
        Ok(())
    }

    /// Prints TODOs to stdout.
    pub fn print_todos(&self) {
        // lock stdout to print faster
        let stdout = io::stdout();
        let lock = stdout.lock();
        let mut out_buffer = io::BufWriter::new(lock);

        self.write_todos(&mut out_buffer).unwrap();
    }

    /// Writes TODOs to out_buffer.
    pub fn write_todos(&self, out_buffer: &mut Write) -> Result<(), Error> {
        for todo_file in &self.todo_files {
            if todo_file.is_empty() {
                continue;
            }

            write_file_todos(out_buffer, &todo_file, &self.config.styles)?;
        }

        Ok(())
    }

    /// Prints TODOs to stdout. Only prints TODOs that fulfill pred.
    pub fn print_filtered_todos<P>(&self, pred: &P)
    where
        P: Fn(&Todo) -> bool,
    {
        // lock stdout to print faster
        let stdout = io::stdout();
        let lock = stdout.lock();
        let mut out_buffer = io::BufWriter::new(lock);

        self.write_filtered_todos(&mut out_buffer, pred).unwrap();
    }

    /// Writes TODOs to out_buffer. Only writes TODOs that fulfill pred.
    pub fn write_filtered_todos<P>(&self, out_buffer: &mut Write, pred: &P) -> Result<(), Error>
    where
        P: Fn(&Todo) -> bool,
    {
        for todo_file in &self.todo_files {
            write_filtered_file_todos(out_buffer, &todo_file, &self.config.styles, pred)?;
        }

        Ok(())
    }

    /// Writes TODOs to out_buffer.
    // MAYB: change self.todo_files to Hashmap for easier finding
    pub fn write_todos_from_file(
        &self,
        filepath: &Path,
        out_buffer: &mut Write,
    ) -> Result<(), Error> {
        for todo_file in &self.todo_files {
            if todo_file.filepath == filepath {
                write_file_todos(out_buffer, &todo_file, &self.config.styles)?;
                break;
            }
        }

        Ok(())
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
            filepath: filepath.to_string_lossy().to_string(),
        }
        .into())
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
            filepath: filepath.to_string_lossy().to_string(),
        }
        .into())
    }
}
