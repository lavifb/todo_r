// Module for displaying output

use ansi_term::Style;
use failure::Error;
use fnv::FnvHashMap;
use log::debug;
use std::io::Write;

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

/// Writes file path and a list of Todos to out_buffer.
/// Predicate `pred` is used to determine if a `Todo` should be printed.
///
/// If no there are no `Todo`s that satisfy `pred` in `todo_file`, nothing is printed.
pub fn write_file_todos<P>(
    out_buffer: &mut Write,
    todo_file: &TodoFile,
    styles: &TodoRStyles,
    pred: Box<P>,
) -> Result<(), Error>
where
    P: Fn(&&Todo) -> bool,
{
    let mut todos = todo_file.todos.iter().filter(*pred).peekable();
    if todos.peek().is_some() {
        writeln!(
            out_buffer,
            "{}",
            styles
                .filepath_style
                .paint(todo_file.filepath.to_string_lossy())
        )?;

        for todo in todos {
            writeln!(out_buffer, "{}", todo.style_string(styles))?;
        }
    } else {
        debug!(
            "No filtered TODOs found in `{}`",
            todo_file.filepath.to_string_lossy()
        )
    }

    Ok(())
}
