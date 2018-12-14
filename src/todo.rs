// Module for holding Todo types.

use ansi_term::Style;
use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;
use std::fmt;
use std::path::{Path, PathBuf};

lazy_static! {
    static ref USER_REGEX: Regex = Regex::new(r"(@\S+)").unwrap();
}

/// A struct holding the TODO and all the needed meta-information for it.
#[derive(Debug, Clone)]
pub struct Todo<'a> {
    pub line: usize,
    pub tag: String,
    pub content: String,
    // TODO: add slices that represent all in-text users
    users: Option<Vec<&'a str>>,
}

impl<'a> Todo<'a> {
    /// Create new TODO struct
    pub fn new<'b, 'c>(line: usize, tag_str: &str, content: impl Into<Cow<'c, str>>) -> Todo<'b> {
        Todo {
            line,
            tag: tag_str.to_uppercase(),
            content: content.into().into_owned(),
            users: None,
        }
    }

    /// Returns colored output string
    pub fn style_string(
        &self,
        line_style: &Style,
        todo_style: &Style,
        content_style: &Style,
    ) -> String {
        // TODO: style for tagged users
        let user_style = line_style;

        // Paint users using user_style by wrapping users with infix ansi-strings
        let cs_to_us = content_style.infix(*user_style);
        let us_to_cs = user_style.infix(*content_style);
        let paint_users = |c: &regex::Captures| format!("{}{}{}", cs_to_us, &c[1], us_to_cs);
        let content_out = USER_REGEX.replace_all(&self.content, paint_users);

        format!(
            "  {}  {}  {}",
            // Columns align for up to 100,000 lines which should be fine
            line_style.paint(format!("line {:<5}", self.line)),
            todo_style.paint(format!("{:5}", &self.tag)),
            content_style.paint(content_out),
        )
    }

    #[allow(dead_code)]
    /// Returns all is tagged in the Todo.
    pub fn users(&'a self) -> Vec<&'a str> {
        USER_REGEX
            .find_iter(&self.content)
            .map(|s| s.as_str())
            .collect()

        // let out = if self.users.is_some() {
        //     self.users.unwrap()
        // } else {
        //     let new_out = USER_REGEX.find_iter(&self.content).map(|s| s.as_str()).collect();
        //     self.users = Some(new_out);
        //     new_out
        // };

        // out
    }

    #[allow(dead_code)]
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

impl<'a> fmt::Display for Todo<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {}\t{}\t{}", self.line, self.tag, self.content,)
    }
}

/// A struct holding the a list of TODOs associated with a file.
#[derive(Debug, Clone)]
pub struct TodoFile<'a> {
    pub filepath: PathBuf,
    pub todos: Vec<Todo<'a>>,
}

impl<'a> TodoFile<'a> {
    pub fn new<'b, P>(filepath: P) -> TodoFile<'b>
    where
        P: Into<Cow<'a, Path>>,
    {
        TodoFile {
            filepath: filepath.into().into_owned(),
            // do not allocate because it will be replaced
            todos: Vec::with_capacity(0),
        }
    }

    pub fn set_todos(&mut self, todos: Vec<Todo<'a>>) {
        self.todos = todos;
    }

    pub fn is_empty(&self) -> bool {
        self.todos.is_empty()
    }

    pub fn len(&self) -> usize {
        self.todos.len()
    }
}