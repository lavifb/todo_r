// Module for displaying output

use parser::Todo;

use ansi_term::Style;
use std::io::{self, Write};

/// Struct for holding ansi color printing options
pub struct StyleConfig {
	filename_style: Style,
	line_number_style: Style,
	todo_type_style: Style,
	content_style: Style,
}

impl StyleConfig {
	/// Creates new StyleConfig with plaintext printing (no colors).
	pub fn no_style() -> StyleConfig {
		StyleConfig {
			filename_style: Style::new(),
			line_number_style: Style::new(),
			todo_type_style: Style::new(),
			content_style: Style::new(),
		}
	}
}

use ansi_term::Color::{Green, Cyan, Fixed};
impl Default for StyleConfig {
	/// Creates new StyleConfig with the default color printing style.
	fn default() -> StyleConfig {
		StyleConfig {
			filename_style: Style::new().underline(),
			line_number_style: Style::from(Fixed(8)),
			todo_type_style: Style::from(Green),
			content_style: Style::from(Cyan),
		}
	}
}

/// Print filename and a list of Todos to stdout
// TODO: add struct that stores file and its TODOs
// MAYB: have different colors for different TODOs
pub fn print_file_todos(filename: &str, todos: &[Todo], styles: &StyleConfig) {
	if todos.is_empty() {
		return
	}

	// lock stdout to print faster
	let stdout = io::stdout();
	let lock = stdout.lock();
	let mut out_buffer = io::BufWriter::new(lock);
	writeln!(out_buffer, "{}", styles.filename_style.paint(filename));
	for todo in todos {
		writeln!(out_buffer, "{}", 
			todo.style_string(
				&styles.line_number_style, 
				&styles.todo_type_style, 
				&styles.content_style
			)
		);
	}
}

/// Print a list of Todos to stdout
#[allow(dead_code)]
pub fn print_todos(todos: &[Todo]) {
	for todo in todos {
		println!("{}", todo);
	}
}