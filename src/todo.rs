// Module for holding Todo types.

use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;
use std::fmt;
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

        format!(
            "  {}  {}  {}",
            // Columns align for up to 100,000 lines which should be fine
            styles
                .line_number_style
                .paint(format!("line {:<5}", self.line)),
            styles
                .tag_style(&self.tag)
                .paint(format!("{:5}", &self.tag)),
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
    pub fn new<'p>(filepath: impl Into<Cow<'p, Path>>) -> TodoFile {
        TodoFile {
            filepath: filepath.into().into_owned(),
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
}
