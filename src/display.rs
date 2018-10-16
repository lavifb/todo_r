// Module for displaying output

use parser::Todo;

use ansi_term::Style;
use ansi_term::Colour::{Green, Cyan, Fixed};

/// Print filename and a list of Todos to stdout
// TODO: add colors/color options
pub fn print_file_todos(filename: &str, todos: Vec<Todo>) {
	println!("{}", Style::new().underline().paint(filename));
	for todo in todos {
		// TODO: add option for no colors
		// TODO: format using something other than \t tabs
		println!("  {}{}\t{}\t{}", Fixed(8).paint("line "), Fixed(8).paint(todo.line.to_string()), Green.paint(todo.todo_type), Cyan.paint(todo.content));
	}
}

/// Print a list of Todos to stdout
#[allow(dead_code)]
pub fn print_todos(todos: Vec<Todo>) {
	for todo in todos {
		println!("{}", todo);
	}
}