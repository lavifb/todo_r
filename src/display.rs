// Module for displaying output

use parser::Todo;

use ansi_term::Style;
use ansi_term::Color::{Green, Cyan, Fixed};

/// Print filename and a list of Todos to stdout
// TODO: add colors/color options
// TODO: add struct that stores file and its TODOs
pub fn print_file_todos(filename: &str, todos: &[Todo]) {
	println!("{}", Style::new().underline().paint(filename));
	for todo in todos {
		// TODO: add option for no colors
		// TODO: print entire buffer at once instead of one at a time
		// TODO: store colors in config struct
		println!("{}", todo.color_string(&Fixed(8), &Green, &Cyan)); // Color output gray, green, and cyan
	}
}

/// Print a list of Todos to stdout
#[allow(dead_code)]
pub fn print_todos(todos: &[Todo]) {
	for todo in todos {
		println!("{}", todo);
	}
}