// Module for printing TODOs in various formats

use crate::todo::{Todo, TodoFile};
use failure::Error;
use fnv::FnvHashMap;
use serde_derive::Serialize;
use serde_json;
use std::fmt::Write as StringWrite;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Debug)]
struct PrintTodo<'a> {
    file: &'a Path,
    kind: &'a str,
    line: usize,
    text: &'a str,
    users: Vec<&'a str>,
}

impl PrintTodo<'_> {
    fn from_todo<'p, P: AsRef<Path> + ?Sized>(todo: &'p Todo, file: &'p P) -> PrintTodo<'p> {
        PrintTodo {
            file: file.as_ref(),
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

struct PrintTodoIter<'a, I>
where
    I: Iterator<Item = &'a Todo>,
{
    inner: I,
    file: &'a Path,
}

type TodoIter<'a> = std::slice::Iter<'a, Todo>;
impl<'a> From<&'a TodoFile> for PrintTodoIter<'a, TodoIter<'a>> {
    fn from(tf: &TodoFile) -> PrintTodoIter<'_, TodoIter<'_>> {
        PrintTodoIter {
            inner: tf.todos.iter(),
            file: &tf.filepath,
        }
    }
}

impl<'a, I> Iterator for PrintTodoIter<'a, I>
where
    I: Iterator<Item = &'a Todo>,
{
    type Item = PrintTodo<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|t| PrintTodo::from_todo(t, self.file))
    }
}

#[derive(Serialize, Debug)]
struct PrintTodos<'a> {
    ptodos: Vec<PrintTodo<'a>>,
}

impl PrintTodos<'_> {
    fn from_todo_files(todo_files: &[TodoFile]) -> PrintTodos {
        let mut ptodos = Vec::new();
        for tf in todo_files {
            ptodos.extend(PrintTodoIter::from(tf));
        }

        PrintTodos { ptodos }
    }

    #[allow(dead_code)]
    fn from_todo_file(todo_file: &TodoFile) -> PrintTodos {
        let ptodos: Vec<PrintTodo> = PrintTodoIter::from(todo_file).collect();

        PrintTodos { ptodos }
    }

    /// Writes String of TODOs serialized in the JSON format
    fn write_json(&self, out_buffer: &mut impl Write) -> Result<(), Error> {
        serde_json::to_writer(out_buffer, &self.ptodos)?;
        Ok(())
    }

    /// Writes String of TODOs serialized in a pretty JSON format
    fn write_json_pretty(&self, out_buffer: &mut impl Write) -> Result<(), Error> {
        serde_json::to_writer_pretty(out_buffer, &self.ptodos)?;
        Ok(())
    }

    /// Writes String of TODOs serialized in a markdown format
    fn write_markdown(&self, out_buffer: &mut impl Write) -> Result<(), Error> {
        let mut tag_tables: FnvHashMap<String, String> = FnvHashMap::default();

        for ptodo in self.ptodos.iter() {
            let tag = ptodo.kind.to_string();

            let table_string = tag_tables.entry(tag).or_insert_with(|| {
                format!(
                    "### {}s\n| Filename | line | {} |\n|:---|:---:|:---|\n",
                    ptodo.kind, ptodo.kind,
                )
            });

            writeln!(
                table_string,
                "| {} | {} | {} |",
                ptodo.file.display(),
                ptodo.line,
                ptodo.text
            )?;
        }

        for table_strings in tag_tables.values() {
            writeln!(out_buffer, "{}", table_strings)?;
        }

        Ok(())
    }
}

// MAYB: add more output formats
/// Enum holding the different supported output formats.
pub enum ReportFormat {
    Json,
    JsonPretty,
    Markdown,
}

/// Writes TODOs in `todo_files` to `out_buffer` in the format provided by `report_format`
pub(crate) fn report_todos(
    out_buffer: &mut impl Write,
    todo_files: &[TodoFile],
    report_format: &ReportFormat,
) -> Result<(), Error> {
    let formatted_write = match report_format {
        ReportFormat::Json => PrintTodos::write_json,
        ReportFormat::JsonPretty => PrintTodos::write_json_pretty,
        ReportFormat::Markdown => PrintTodos::write_markdown,
    };

    let ptodos = PrintTodos::from_todo_files(todo_files);
    formatted_write(&ptodos, out_buffer)?;

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::todo::Todo;
    use std::io::Cursor;

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
        let ptodo = PrintTodos::from_todo_file(&tf);

        let out_vec: Vec<u8> = Vec::new();
        let mut out_buf = Cursor::new(out_vec);
        ptodo.write_json(&mut out_buf).unwrap();

        assert_eq!(
            &String::from_utf8(out_buf.into_inner()).unwrap(),
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
        let ptodo = PrintTodos::from_todo_files(&tfs);

        let out_vec: Vec<u8> = Vec::new();
        let mut out_buf = Cursor::new(out_vec);
        ptodo.write_json(&mut out_buf).unwrap();

        assert_eq!(
            &String::from_utf8(out_buf.into_inner()).unwrap(),
            r#"[{"file":"test1.rs","kind":"TODO","line":2,"text":"item1","users":[]},{"file":"test2.rs","kind":"TODO","line":5,"text":"item2","users":[]}]"#,
        );
    }
}
