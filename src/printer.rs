// Module for printing TODOs in various formats

use crate::todo::Todo;
use serde;
use std::path::Path;

struct PrintTodo<'a> {
    file: &'a str,
    kind: &'a str,
    line: usize,
    text: &'a str,
    users: Vec<&'a str>,
}

impl<'a> PrintTodo<'a> {
    fn new<'p>(todo: &'p Todo, filepath: &'p Path) -> PrintTodo<'p> {
        PrintTodo {
            // TODO: deal with error
            file: filepath.to_str().unwrap(),
            kind: &todo.tag,
            line: todo.line,
            text: &todo.content,
            users: todo.users(),
        }
    }
}
