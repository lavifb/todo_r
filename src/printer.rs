// Module for printing TODOs in various formats

use crate::todo::{Todo, TodoFile};
use failure::{format_err, Error};
use serde_derive::Serialize;
use serde_json;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Debug)]
struct PrintTodo<'a> {
    file: &'a str,
    kind: &'a str,
    line: usize,
    text: &'a str,
    users: Vec<&'a str>,
}

impl<'a> PrintTodo<'a> {
    fn from_todo<'p>(todo: &'p Todo, filepath: &'p Path) -> Result<PrintTodo<'p>, Error> {
        let file = filepath.to_str().ok_or_else(|| {
            format_err!(
                "error converting filepath `{}` to unicode",
                filepath.display()
            )
        })?;

        Ok(PrintTodo {
            file,
            kind: &todo.tag,
            line: todo.line,
            text: &todo.content,
            users: todo.users().iter().map(|u| &u[1..]).collect(),
        })
    }

    /// Returns String of TODO serialized in the JSON format
    fn to_json(&self) -> Result<String, Error> {
        Ok(serde_json::to_string(self)?)
    }

    /// Returns String of TODO serialized in a pretty JSON format
    fn to_json_pretty(&self) -> Result<String, Error> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}

#[derive(Serialize, Debug)]
struct PrintTodos<'a> {
    ptodos: Vec<PrintTodo<'a>>,
}

// TODO: convert Vec<TodoFile> into PrintTodos
// TODO: impl to_json(), to_json_pretty(), ... for PrintTodos

pub enum ReportFormat {
    Json,
    JsonPretty,
}

/// Writes TODOs in `todo_files` to `out_buffer` in the format provided by `report_format`
pub fn report_todos(
    out_buffer: &mut Write,
    todo_files: &[TodoFile],
    report_format: &ReportFormat,
) -> Result<(), Error> {
    let report = match report_format {
        ReportFormat::Json => PrintTodo::to_json,
        ReportFormat::JsonPretty => PrintTodo::to_json_pretty,
    };

    for tf in todo_files {
        let filepath = &tf.filepath;
        for todo in &tf.todos {
            let ptodo = PrintTodo::from_todo(&todo, &filepath)?;
            write!(out_buffer, "{}", report(&ptodo)?)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::todo::Todo;

    #[test]
    fn json_todo() {
        let todo = Todo::new(2, "TODO", "item");
        let test_path = Path::new("tests/test.rs");
        let ptodo = PrintTodo::from_todo(&todo, &test_path).unwrap();

        assert_eq!(
            ptodo.to_json().unwrap(),
            r#"{"file":"tests/test.rs","kind":"TODO","line":2,"text":"item","users":[]}"#,
        );
    }

    #[test]
    fn json_todo_pretty() {
        let todo = Todo::new(2, "TODO", "item");
        let test_path = Path::new("tests/test.rs");
        let ptodo = PrintTodo::from_todo(&todo, &test_path).unwrap();

        assert_eq!(
            ptodo.to_json_pretty().unwrap(),
            r#"{
  "file": "tests/test.rs",
  "kind": "TODO",
  "line": 2,
  "text": "item",
  "users": []
}"#,
        );
    }

    #[test]
    fn json_todo_users() {
        let todo = Todo::new(2, "TODO", "@user1 item @user2");
        let test_path = Path::new("tests/test.rs");
        let ptodo = PrintTodo::from_todo(&todo, &test_path).unwrap();

        assert_eq!(
            ptodo.to_json().unwrap(),
            r#"{"file":"tests/test.rs","kind":"TODO","line":2,"text":"@user1 item @user2","users":["user1","user2"]}"#,
        );
    }
}
