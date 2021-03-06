pub mod comments;
mod configs;
mod custom_tags;
mod display;
pub mod format;
mod maps;
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
        /// Error for unsupported output format.
        #[fail(display = "invalid output format: {}", message)]
        InvalidOutputFormat { message: String },
    }
}

use failure::Error;
use log::debug;
use serde::ser::{Serialize, SerializeSeq, Serializer};
use std::borrow::Cow;
use std::fs::File;
use std::io::{self, BufReader, Cursor, Write};
use std::path::Path;

use crate::comments::CommentTypes;
use crate::configs::TodoRConfigFileSerial;
use crate::display::{write_file_todos, TodoRStyles};
use crate::errors::TodoRError;
use crate::maps::CommentRegexMultiMap;
use crate::parser::{parse_content, parse_content_with_filter};
use crate::todo::{PathedTodo, Todo, TodoFile};

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
    override_default_ext: Option<String>,
    override_styles: Option<TodoRStyles>,
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
    pub fn build(self) -> Result<TodoR, Error> {
        let config_struct: TodoRConfigFileSerial =
            self.inner_config
                .try_into()
                .map_err(|err| TodoRError::InvalidConfigFile {
                    message: err.to_string(),
                })?;
        debug!("configuration successfully loaded");

        let mut tags = self
            .override_tags
            .unwrap_or_else(|| config_struct.tags.to_owned());
        let default_ext = self
            .override_default_ext
            .unwrap_or_else(|| config_struct.default_ext.to_owned());
        tags.append(&mut self.added_tags.clone());

        let config_styles = config_struct.styles;
        let styles = self
            .override_styles
            .unwrap_or(config_styles.into_todo_r_styles()?);

        let mut ext_to_regexs = CommentRegexMultiMap::new(CommentTypes::new().add_single("#"));
        // Iter over default comment types
        for comment_config in config_struct
            .default_comments
            .into_iter()
            // Iter over config comment types
            .chain(config_struct.comments.into_iter())
        {
            let (config_ext, config_exts, comment_types) = comment_config.break_apart();
            let exts = config_exts.into_iter().flatten().chain(config_ext);

            // Add all extensions into ext_to_regexs
            ext_to_regexs.insert_keys(exts, comment_types);
        }

        ext_to_regexs
            .reset_fallback_key(&default_ext)
            .ok_or(TodoRError::InvalidDefaultExtension { ext: default_ext })?;

        let config = TodoRConfig {
            tags,
            styles,
            ext_to_regexs,
        };

        debug!("todor parser built: {:?}", config);

        Ok(TodoR::with_config(config))
    }

    /// Adds config file for TodoR.
    pub fn add_config_file(&mut self, config_path: impl AsRef<Path>) -> Result<&mut Self, Error> {
        self.inner_config
            .merge(config::File::from(config_path.as_ref()))?;
        Ok(self)
    }

    /// Adds config file for TodoR that is in the provided format.
    pub fn add_config_file_with_format(
        &mut self,
        config_path: impl AsRef<Path>,
        format: config::FileFormat,
    ) -> Result<&mut Self, Error> {
        let mut merge_file = config::File::from(config_path.as_ref());
        merge_file = merge_file.format(format);
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
    pub fn add_override_tag<'t, S: Into<Cow<'t, str>>>(&mut self, tag: S) -> &mut Self {
        self.override_tags
            .get_or_insert_with(Vec::new)
            .push(tag.into().into_owned());
        self
    }

    /// Adds tags for TodoR to look for. This overrides tags from config files.
    pub fn add_override_tags<'t, I, S>(&mut self, tags: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: Into<Cow<'t, str>>,
    {
        {
            let tws = self.override_tags.get_or_insert_with(Vec::new);

            tws.extend(tags.into_iter().map(|s| s.into().into_owned()))
        }
        self
    }

    /// Sets the terminal output of TodoR to be with no styles.
    pub fn set_no_style(&mut self) -> &mut Self {
        self.override_styles = Some(TodoRStyles::no_style());
        self
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
    styles: TodoRStyles,
    ext_to_regexs: CommentRegexMultiMap<String>,
}

/// Parser for finding TODOs in comments and storing them on a per-file basis.
#[derive(Debug, Clone)]
pub struct TodoR {
    config: TodoRConfig,
    todo_files: Vec<TodoFile>,
}

impl<'a> Default for TodoR {
    fn default() -> TodoR {
        TodoRBuilder::new().build().unwrap()
    }
}

impl TodoR {
    /// Creates new TodoR that looks for provided keywords.
    pub fn new() -> TodoR {
        TodoR::default()
    }

    pub fn with_tags<'t, I, S>(tags: I) -> TodoR
    where
        I: IntoIterator<Item = S>,
        S: Into<Cow<'t, str>>,
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

    /// Returns the number of TODOs currently tracked by TodoR
    pub fn num_todos(&self) -> usize {
        self.todo_files.iter().map(|tf| tf.todos.len()).sum()
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

    /// Opens file at given filepath and process it by finding all its TODOs.
    pub fn open_todos<F>(&mut self, filepath: F) -> Result<(), Error>
    where
        F: AsRef<Path>,
    {
        // using _p just to let the compiler know the correct type for open_option_filtered_todos()
        let mut _p = Some(|_t: &Todo| true);
        _p = None;
        self.open_option_filtered_todos(filepath, &_p)
    }

    /// Opens file at given filepath and process it by finding all its TODOs.
    /// Only TODOs that satisfy pred are added.
    pub fn open_filtered_todos<P, F>(&mut self, filepath: F, pred: &P) -> Result<(), Error>
    where
        P: Fn(&Todo) -> bool,
        F: AsRef<Path>,
    {
        self.open_option_filtered_todos(filepath, &Some(pred))
    }

    /// Opens file at given filepath and process it by finding all its TODOs.
    /// If pred is not None, only TODOs that satisfy pred are added.
    ///
    /// This method is useful for when you are not sure at compile time if a filter is necessary.
    pub fn open_option_filtered_todos<P, F>(
        &mut self,
        filepath: F,
        pred: &Option<P>,
    ) -> Result<(), Error>
    where
        P: Fn(&Todo) -> bool,
        F: AsRef<Path>,
    {
        let filepath = filepath.as_ref();
        let mut todo_file = TodoFile::new(filepath);

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

        let file_ext = match filepath.extension() {
            Some(ext) => ext.to_str().unwrap(),
            // lots of shell files have no extension
            None => "sh",
        };
        let parser_regexs = self.config.ext_to_regexs.get(file_ext, &self.config.tags);

        let file = File::open(filepath)?;
        let mut file_reader = BufReader::new(file);
        todo_file.set_todos(match pred {
            Some(p) => parse_content_with_filter(&mut file_reader, &parser_regexs, p)?,
            None => parse_content(&mut file_reader, &parser_regexs)?,
        });

        debug!(
            "found {} TODOs in `{}`",
            todo_file.len(),
            filepath.display()
        );

        self.todo_files.push(todo_file);
        Ok(())
    }

    /// Finds TODO comments in the given content
    pub fn find_todos(&mut self, content: &str, ext: &str) -> Result<(), Error> {
        let mut todo_file = TodoFile::new("");
        let mut content_buf = Cursor::new(content);
        let parser_regexs = self.config.ext_to_regexs.get(ext, &self.config.tags);

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
    pub fn write_todos(&self, out_buffer: &mut impl Write) -> Result<(), Error> {
        for todo_file in &self.todo_files {
            write_file_todos(out_buffer, &todo_file, &self.config.styles)?;
        }

        Ok(())
    }

    /// Writes TODOs to out_buffer.
    // MAYB: change self.todo_files to Hashmap for easier finding
    pub fn write_todos_from_file(
        &self,
        filepath: &Path,
        out_buffer: &mut impl Write,
    ) -> Result<(), Error> {
        for todo_file in &self.todo_files {
            if todo_file.filepath == filepath {
                write_file_todos(out_buffer, &todo_file, &self.config.styles)?;
                break;
            }
        }

        Ok(())
    }

    /// Returns an iterator that Iterates over tracked TODOs along with the
    pub fn iter(&self) -> impl Iterator<Item = PathedTodo> {
        self.todo_files.iter().flatten()
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

impl Serialize for TodoR {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.todo_files.len()))?;
        for ptodo in self.iter() {
            seq.serialize_element(&ptodo)?;
        }
        seq.end()
    }
}
