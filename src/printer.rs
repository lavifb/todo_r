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
    #[allow(dead_code)]
    fn from_todo_with_path<'p>(todo: &'p Todo, filepath: &'p Path) -> Result<PrintTodo<'p>, Error> {
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

    fn from_todo<'p>(todo: &'p Todo, file: &'p str) -> PrintTodo<'p> {
        PrintTodo {
            file,
            kind: &todo.tag,
            line: todo.line,
            text: &todo.content,
            users: todo.users().iter().map(|u| &u[1..]).collect(),
        }
    }

    /// Returns String of TODO serialized in the JSON format
    #[allow(dead_code)]
    fn to_json(&self) -> Result<String, Error> {
        Ok(serde_json::to_string(self)?)
    }

    /// Returns String of TODO serialized in a pretty JSON format
    #[allow(dead_code)]
    fn to_json_pretty(&self) -> Result<String, Error> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}

struct PrintTodoIter<'a> {
    inner: std::slice::Iter<'a, Todo>,
    file: &'a str,
    pred: fn(&&Todo) -> bool,
}

impl<'a> PrintTodoIter<'a> {
    fn try_from(tf: &TodoFile) -> Result<PrintTodoIter, Error> {
        let file = tf.filepath.to_str().ok_or_else(|| {
            format_err!(
                "error converting filepath `{}` to unicode",
                tf.filepath.display()
            )
        })?;

        let pred = |_t: &&Todo| true;

        Ok(PrintTodoIter {
            inner: tf.todos.iter(),
            file,
            pred,
        })
    }

    #[allow(dead_code)]
    fn try_from_with_filter(
        tf: &TodoFile,
        pred: fn(&&Todo) -> bool,
    ) -> Result<PrintTodoIter, Error> {
        let file = tf.filepath.to_str().ok_or_else(|| {
            format_err!(
                "error converting filepath `{}` to unicode",
                tf.filepath.display()
            )
        })?;

        Ok(PrintTodoIter {
            inner: tf.todos.iter(),
            file,
            pred,
        })
    }
}

impl<'a> Iterator for PrintTodoIter<'a> {
    type Item = PrintTodo<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .filter(self.pred)
            .map(|t| PrintTodo::from_todo(t, self.file))
    }
}

#[derive(Serialize, Debug)]
struct PrintTodos<'a> {
    ptodos: Vec<PrintTodo<'a>>,
}

impl<'a> PrintTodos<'a> {
    fn from_todo_files(todo_files: &[TodoFile]) -> Result<PrintTodos, Error> {
        let mut ptodos = Vec::new();
        for tf in todo_files {
            ptodos.extend(PrintTodoIter::try_from(tf)?);
        }

        Ok(PrintTodos { ptodos })
    }

    #[allow(dead_code)]
    fn from_todo_file(todo_file: &TodoFile) -> Result<PrintTodos, Error> {
        let ptodos: Vec<PrintTodo> = PrintTodoIter::try_from(todo_file)?.collect();

        Ok(PrintTodos { ptodos })
    }

    /// Returns String of TODOs serialized in the JSON format
    fn to_json(&self) -> Result<String, Error> {
        Ok(serde_json::to_string(&self.ptodos)?)
    }

    /// Returns String of TODOs serialized in a pretty JSON format
    fn to_json_pretty(&self) -> Result<String, Error> {
        Ok(serde_json::to_string_pretty(&self.ptodos)?)
    }
}

// TODO: add more output formats
pub enum ReportFormat {
    Json,
    JsonPretty,
}

/// Writes TODOs in `todo_files` to `out_buffer` in the format provided by `report_format`
pub(crate) fn report_todos(
    out_buffer: &mut Write,
    todo_files: &[TodoFile],
    report_format: &ReportFormat,
) -> Result<(), Error> {
    let report = match report_format {
        ReportFormat::Json => PrintTodos::to_json,
        ReportFormat::JsonPretty => PrintTodos::to_json_pretty,
    };

    let ptodos = PrintTodos::from_todo_files(todo_files)?;
    write!(out_buffer, "{}", report(&ptodos)?)?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::todo::Todo;

    #[test]
    fn json_todo() {
        let todo = Todo::new(2, "TODO", "item");
        let test_path = "tests/test.rs";
        let ptodo = PrintTodo::from_todo(&todo, test_path);

        assert_eq!(
            ptodo.to_json().unwrap(),
            r#"{"file":"tests/test.rs","kind":"TODO","line":2,"text":"item","users":[]}"#,
        );
    }

    #[test]
    fn json_todo_pretty() {
        let todo = Todo::new(2, "TODO", "item");
        let test_path = "tests/test.rs";
        let ptodo = PrintTodo::from_todo(&todo, test_path);

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
        let test_path = "tests/test.rs";
        let ptodo = PrintTodo::from_todo(&todo, test_path);

        assert_eq!(
            ptodo.to_json().unwrap(),
            r#"{"file":"tests/test.rs","kind":"TODO","line":2,"text":"@user1 item @user2","users":["user1","user2"]}"#,
        );
    }

    #[test]
    fn json_todos() {
        let mut tf = TodoFile::new(Path::new("tests/test.rs"));
        tf.todos.push(Todo::new(2, "TODO", "item1"));
        tf.todos.push(Todo::new(5, "TODO", "item2"));
        let ptodo = PrintTodos::from_todo_file(&tf).unwrap();

        assert_eq!(
            ptodo.to_json().unwrap(),
            r#"[{"file":"tests/test.rs","kind":"TODO","line":2,"text":"item1","users":[]},{"file":"tests/test.rs","kind":"TODO","line":5,"text":"item2","users":[]}]"#,
        );
    }

    #[test]
    fn json_todos2() {
        let mut tf1 = TodoFile::new(Path::new("test1.rs"));
        tf1.todos.push(Todo::new(2, "TODO", "item1"));
        let mut tf2 = TodoFile::new(Path::new("test2.rs"));
        tf2.todos.push(Todo::new(5, "TODO", "item2"));

        let tfs = [tf1, tf2];
        let ptodo = PrintTodos::from_todo_files(&tfs).unwrap();

        assert_eq!(
            ptodo.to_json().unwrap(),
            r#"[{"file":"test1.rs","kind":"TODO","line":2,"text":"item1","users":[]},{"file":"test2.rs","kind":"TODO","line":5,"text":"item2","users":[]}]"#,
        );
    }
}
