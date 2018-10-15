// Module for displaying output

use parser::Todo;

/// Print filename and a list of Todos to stdout
// TODO: add colors/color options
pub fn print_file_todos(filename: &str, todos: Vec<Todo>) {
	println!("{}", filename);
	for todo in todos {
		println!("  {}", todo);
	}
}

/// Print a list of Todos to stdout
#[allow(dead_code)]
pub fn print_todos(todos: Vec<Todo>) {
	for todo in todos {
		println!("{}", todo);
	}
}