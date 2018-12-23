// Module for displaying output

use ansi_term::Color::{Cyan, Fixed, Green};
use ansi_term::Style;
use failure::Error;
use fnv::FnvHashMap;
use log::debug;
use std::io::{self, Write};

use crate::todo::{Todo, TodoFile};

/// Struct for holding ansi color printing options
#[derive(Debug, Clone)]
pub struct TodoRStyles {
    pub filepath_style: Style,
    pub line_number_style: Style,
    pub user_style: Style,
    pub content_style: Style,
    default_tag_style: Style,
    tag_styles: FnvHashMap<String, Style>,
}

impl Default for TodoRStyles {
    /// Creates new StyleConfig with the default color printing style.
    fn default() -> TodoRStyles {
        TodoRStyles::new(
            Style::new().underline(),
            Style::from(Fixed(8)),
            Style::from(Fixed(8)),
            Style::from(Cyan),
            Style::from(Green),
        )
    }
}

impl TodoRStyles {
    pub fn new(
        filepath_style: Style,
        line_number_style: Style,
        user_style: Style,
        content_style: Style,
        default_tag_style: Style,
    ) -> TodoRStyles {
        TodoRStyles {
            filepath_style,
            line_number_style,
            user_style,
            content_style,
            default_tag_style,
            tag_styles: FnvHashMap::default(),
        }
    }

    /// Creates new StyleConfig with plaintext printing (no colors).
    pub fn no_style() -> TodoRStyles {
        TodoRStyles {
            filepath_style: Style::new(),
            line_number_style: Style::new(),
            user_style: Style::new(),
            content_style: Style::new(),
            default_tag_style: Style::new(),
            tag_styles: FnvHashMap::default(),
        }
    }

    /// Adds style for printing given tag
    pub fn add_tag_style(mut self, tag: &str, style: Style) -> Self {
        self.tag_styles.insert(tag.to_uppercase(), style);
        self
    }

    /// Returns tag style for given tag.
    pub fn tag_style(&self, tag: &str) -> &Style {
        self.tag_styles.get(tag).unwrap_or(&self.default_tag_style)
    }
}

// TODO: other printing options: json, xml, etc.

#[allow(dead_code)]
/// Prints file path and a list of Todos to stdout
pub fn print_file_todos(todo_file: &TodoFile, styles: &TodoRStyles, verbose: bool) {
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
pub fn write_file_todos(
    out_buffer: &mut Write,
    todo_file: &TodoFile,
    styles: &TodoRStyles,
) -> Result<(), Error> {
    writeln!(
        out_buffer,
        "{}",
        styles
            .filepath_style
            .paint(todo_file.filepath.to_string_lossy())
    )?;
    for todo in &todo_file.todos {
        writeln!(out_buffer, "{}", todo.style_string(styles),)?;
    }

    Ok(())
}

pub fn write_filtered_file_todos<P>(
    out_buffer: &mut Write,
    todo_file: &TodoFile,
    styles: &TodoRStyles,
    pred: &P,
) -> Result<(), Error>
where
    P: Fn(&&Todo) -> bool,
{
    let mut tmp: Vec<u8> = Vec::new();
    for todo in todo_file.todos.iter().filter(pred) {
        writeln!(tmp, "{}", todo.style_string(styles))?;
    }

    if !tmp.is_empty() {
        writeln!(
            out_buffer,
            "{}",
            styles
                .filepath_style
                .paint(todo_file.filepath.to_string_lossy())
        )?;

        out_buffer.write_all(&tmp)?;
    } else {
        debug!(
            "No filtered TODOs found in `{}`",
            todo_file.filepath.to_string_lossy()
        )
    }

    Ok(())
}
