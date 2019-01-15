// Module for displaying output

use ansi_term::Style;
use failure::Error;
use log::debug;
use std::io::Write;

use crate::maps::FallbackHashMap;
use crate::todo::TodoFile;

/// Struct for holding ansi color printing options
#[derive(Debug, Clone)]
pub struct TodoRStyles {
    pub filepath_style: Style,
    pub line_number_style: Style,
    pub user_style: Style,
    pub content_style: Style,
    tag_styles: FallbackHashMap<String, Style>,
}

impl TodoRStyles {
    /// Creates new StyleConfig
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
            tag_styles: FallbackHashMap::new(default_tag_style),
        }
    }

    /// Creates new StyleConfig with plaintext printing (no colors).
    pub fn no_style() -> TodoRStyles {
        TodoRStyles {
            filepath_style: Style::new(),
            line_number_style: Style::new(),
            user_style: Style::new(),
            content_style: Style::new(),
            tag_styles: FallbackHashMap::new(Style::new()),
        }
    }

    /// Adds style for printing given tag
    pub fn add_tag_style(mut self, tag: &str, style: Style) -> Self {
        self.tag_styles.insert(tag.to_uppercase(), style);
        self
    }

    /// Returns tag style for given tag.
    pub fn tag_style(&self, tag: &str) -> &Style {
        self.tag_styles.get(tag)
    }
}

/// Writes file path and a list of Todos to out_buffer.
///
/// If no there are no `Todo`s that satisfy `pred` in `todo_file`, nothing is printed.
pub fn write_file_todos(
    out_buffer: &mut impl Write,
    todo_file: &TodoFile,
    styles: &TodoRStyles,
) -> Result<(), Error> {
    let mut todos = todo_file.todos.iter().peekable();
    if todos.peek().is_some() {
        writeln!(
            out_buffer,
            "{}",
            styles
                .filepath_style
                .paint(todo_file.filepath.to_string_lossy())
        )?;

        for todo in todos {
            todo.write_style_string(out_buffer, styles)?;
        }
    } else {
        debug!(
            "No TODOs found in `{}`",
            todo_file.filepath.to_string_lossy()
        )
    }

    Ok(())
}
