// Module for holding Todo types.

use failure::Error;
use lazy_static::lazy_static;
use regex::Regex;
use serde::ser::{Serialize, SerializeSeq, SerializeStruct, Serializer};
use serde_derive::Serialize;
use std::borrow::Cow;
use std::fmt;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::display::TodoRStyles;

lazy_static! {
    static ref USER_REGEX: Regex = Regex::new(r"(@\S+)").unwrap();
}

/// A struct holding the TODO and all the needed meta-information for it.
#[derive(Debug, Clone)]
pub struct Todo {
    pub line: usize,
    pub tag: String,
    pub content: String,
}

impl Todo {
    /// Create new TODO struct
    pub fn new<'c>(line: usize, tag_str: &str, content: impl Into<Cow<'c, str>>) -> Todo {
        Todo {
            line,
            tag: tag_str.to_uppercase(),
            content: content.into().into_owned(),
        }
    }

    /// Returns ANSI colored output string
    pub fn style_string(&self, styles: &TodoRStyles) -> String {
        // Paint users using user_style by wrapping users with infix ansi-strings
        let cs_to_us = styles.content_style.infix(styles.user_style);
        let us_to_cs = styles.user_style.infix(styles.content_style);
        let paint_users = |c: &regex::Captures| format!("{}{}{}", cs_to_us, &c[1], us_to_cs);
        let content_out = USER_REGEX.replace_all(&self.content, paint_users);

        let tag_width = &self.tag.len().min(5);
        format!(
            "  {}  {}{}  {}",
            // Columns align for up to 100,000 lines which should be fine
            styles
                .line_number_style
                .paint(format!("line {:<5}", self.line)),
            styles
                .tag_style(&self.tag)
                .paint(format!("{:w$}", &self.tag, w = tag_width)),
            format!("{:w$}", "", w = 5 - tag_width),
            styles.content_style.paint(content_out),
        )
    }

    /// Returns all is tagged in the Todo.
    pub fn users(&self) -> Vec<&str> {
        USER_REGEX
            .find_iter(&self.content)
            .map(|s| s.as_str())
            .collect()
    }

    /// Returns true if user is tagged in the Todo.
    pub fn tags_user(&self, user: &str) -> bool {
        for u in self.users() {
            if &u[1..] == user {
                return true;
            }
        }

        false
    }

    /// Returns String of TODO serialized in the JSON format
    pub fn to_json(&self) -> Result<String, Error> {
        Ok(serde_json::to_string(self)?)
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {}\t{}\t{}", self.line, self.tag, self.content,)
    }
}

/// A struct holding the a list of TODOs associated with a file.
#[derive(Debug, Clone)]
pub struct TodoFile {
    pub filepath: PathBuf,
    pub todos: Vec<Todo>,
}

impl TodoFile {
    pub fn new<P: AsRef<Path>>(filepath: P) -> TodoFile {
        TodoFile {
            filepath: filepath.as_ref().to_owned(),
            // do not allocate because it will be replaced
            todos: Vec::with_capacity(0),
        }
    }

    pub fn set_todos(&mut self, todos: Vec<Todo>) {
        self.todos = todos;
    }

    pub fn is_empty(&self) -> bool {
        self.todos.is_empty()
    }

    pub fn len(&self) -> usize {
        self.todos.len()
    }

    /// Writes TODOs in a file serialized in the JSON format
    pub fn write_json(&self, out_buffer: &mut impl Write) -> Result<(), Error> {
        serde_json::to_writer(out_buffer, &self)?;
        Ok(())
    }
}

impl Serialize for Todo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Todo", 4)?;
        state.serialize_field("line", &self.line)?;
        state.serialize_field("tag", &self.tag)?;
        state.serialize_field("text", &self.content)?;
        state.serialize_field("users", &self.users())?;
        state.end()
    }
}

/// Helper struct for printing filename along with other TODO information.
#[derive(Serialize)]
pub struct PathedTodo<'a> {
    pub(crate) file: &'a Path,
    #[serde(flatten)]
    pub(crate) todo: &'a Todo,
}

impl PathedTodo<'_> {
    fn new<'a>(todo: &'a Todo, file: &'a Path) -> PathedTodo<'a> {
        PathedTodo { file, todo }
    }
}

pub struct TodoFileIter<'a, I>
where
    I: Iterator<Item = &'a Todo>,
{
    inner: I,
    file: &'a Path,
}

type TodoIter<'a> = std::slice::Iter<'a, Todo>;
impl<'a> From<&'a TodoFile> for TodoFileIter<'a, TodoIter<'a>> {
    fn from(tf: &TodoFile) -> TodoFileIter<'_, TodoIter<'_>> {
        TodoFileIter {
            inner: tf.todos.iter(),
            file: &tf.filepath,
        }
    }
}

impl<'a, I> Iterator for TodoFileIter<'a, I>
where
    I: Iterator<Item = &'a Todo>,
{
    type Item = PathedTodo<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|t| PathedTodo::new(t, self.file))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl Serialize for TodoFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for todo in &self.todos {
            let ptodo = PathedTodo {
                file: &self.filepath,
                todo,
            };
            seq.serialize_element(&ptodo)?;
        }
        seq.end()
    }
}

impl<'a> IntoIterator for &'a TodoFile {
    type Item = PathedTodo<'a>;
    type IntoIter = TodoFileIter<'a, TodoIter<'a>>;

    fn into_iter(self) -> TodoFileIter<'a, TodoIter<'a>> {
        self.into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;
    use std::io::Cursor;

    #[test]
    fn json_todo() {
        let todo = Todo::new(2, "TODO", "item");

        assert_eq!(
            todo.to_json().unwrap(),
            r#"{"line":2,"tag":"TODO","text":"item","users":[]}"#,
        );
    }

    #[test]
    fn json_todos() {
        let mut tf = TodoFile::new(Path::new("tests/test.rs"));
        tf.todos.push(Todo::new(2, "TODO", "item1"));
        tf.todos.push(Todo::new(5, "TODO", "item2 @u1"));

        let out_vec: Vec<u8> = Vec::new();
        let mut out_buf = Cursor::new(out_vec);
        tf.write_json(&mut out_buf).unwrap();

        assert_eq!(
            &String::from_utf8(out_buf.into_inner()).unwrap(),
            r#"[{"file":"tests/test.rs","line":2,"tag":"TODO","text":"item1","users":[]},{"file":"tests/test.rs","line":5,"tag":"TODO","text":"item2 @u1","users":["@u1"]}]"#,
        );
    }
}
