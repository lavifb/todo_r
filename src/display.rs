// Module for displaying output

use crate::parser::Todo;

use ansi_term::Style;
use failure::Error;
use std::borrow::Cow;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Struct for holding ansi color printing options
#[derive(Debug, Clone)]
pub struct StyleConfig {
    filepath_style: Style,
    line_number_style: Style,
    tag_style: Style,
    content_style: Style,
}

impl StyleConfig {
    /// Creates new StyleConfig with plaintext printing (no colors).
    pub fn no_style() -> StyleConfig {
        StyleConfig {
            filepath_style: Style::new(),
            line_number_style: Style::new(),
            tag_style: Style::new(),
            content_style: Style::new(),
        }
    }
}

use ansi_term::Color::{Cyan, Fixed, Green};
impl Default for StyleConfig {
    /// Creates new StyleConfig with the default color printing style.
    fn default() -> StyleConfig {
        StyleConfig {
            filepath_style: Style::new().underline(),
            line_number_style: Style::from(Fixed(8)),
            tag_style: Style::from(Green),
            content_style: Style::from(Cyan),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TodoFile<'a> {
    pub filepath: PathBuf,
    pub todos: Vec<Todo<'a>>,
}

impl<'a> TodoFile<'a> {
    pub fn new<'b, P>(filepath: P) -> TodoFile<'b>
    where
        P: Into<Cow<'a, Path>>,
    {
        TodoFile {
            filepath: filepath.into().into_owned(),
            // do not allocate because it will be replaced
            todos: Vec::with_capacity(0),
        }
    }

    pub fn set_todos(&mut self, todos: Vec<Todo<'a>>) {
        self.todos = todos;
    }

    pub fn is_empty(&self) -> bool {
        self.todos.is_empty()
    }

    pub fn len(&self) -> usize {
        self.todos.len()
    }
}

// TODO: other printing options: json, xml, etc.

#[allow(dead_code)]
/// Prints file path and a list of Todos to stdout
pub fn print_file_todos(todo_file: &TodoFile, styles: &StyleConfig, verbose: bool) {
    if todo_file.todos.is_empty() && !verbose {
        return;
    }

    // lock stdout to print faster
    let stdout = io::stdout();
    let lock = stdout.lock();
    let mut out_buffer = io::BufWriter::new(lock);
    write_file_todos(&mut out_buffer, todo_file, styles).unwrap();
}

/// Writes file path and a list of Todos to out_buffer
// MAYB: have different colors for different TODOs
pub fn write_file_todos(
    out_buffer: &mut Write,
    todo_file: &TodoFile,
    styles: &StyleConfig,
) -> Result<(), Error> {
    writeln!(
        out_buffer,
        "{}",
        styles
            .filepath_style
            .paint(todo_file.filepath.to_string_lossy())
    )?;
    for todo in &todo_file.todos {
        writeln!(
            out_buffer,
            "{}",
            todo.style_string(
                &styles.line_number_style,
                &styles.tag_style,
                &styles.content_style
            )
        )?;
    }

    Ok(())
}

// TODO: make write_user_file_todos() that only ouputs user tagged TODOs