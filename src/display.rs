// Module for displaying output

use ansi_term::Style;
use failure::Error;
use log::debug;
use std::io::{self, Write};

use crate::todo::{Todo, TodoFile};

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

pub fn write_filtered_file_todos<P>(
    out_buffer: &mut Write,
    todo_file: &TodoFile,
    styles: &StyleConfig,
    pred: &P,
) -> Result<(), Error>
where
    P: Fn(&Todo) -> bool,
{
    let mut tmp: Vec<u8> = Vec::new();
    for todo in &todo_file.todos {
        if pred(todo) {
            writeln!(
                tmp,
                "{}",
                todo.style_string(
                    &styles.line_number_style,
                    &styles.tag_style,
                    &styles.content_style
                )
            )?;
        }
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
