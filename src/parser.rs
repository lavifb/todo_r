// Module for finding TODOs in files

use ansi_term::Style;
use lazy_static::lazy_static;
use log::trace;
use regex::Regex;
use std::borrow::Cow;
use std::fmt;
use std::io::BufRead;

use crate::comments::CommentTypes;
use crate::custom_tags::get_regex_for_comment;

lazy_static! {
    static ref USER_REGEX: Regex = Regex::new(r"(@\S+)").unwrap();
}

/// A struct holding the TODO and all the needed meta-information for it.
#[derive(Debug, Clone)]
pub struct Todo<'a> {
    pub line: usize,
    tag: String,
    content: String,
    // TODO: add slices that represent all in-text users
    users: Option<Vec<&'a str>>,
}

impl<'a> Todo<'a> {
    /// Create new TODO struct
    fn new<'b, 'c>(line: usize, tag_str: &str, content: impl Into<Cow<'c, str>>) -> Todo<'b> {
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
            println!("{}", u);
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

/// Parses content and Creates a list of TODOs found in content
pub(crate) fn parse_content<'a, B>(
    content_buf: &mut B,
    comment_types: &CommentTypes,
    tags: &[String],
) -> Result<Vec<Todo<'a>>, std::io::Error>
where
    B: BufRead,
{
    // TODO: cache regexs
    let regexs: Vec<Regex> = comment_types
        .iter()
        .map(|c| get_regex_for_comment(tags, c).unwrap())
        .collect();

    trace!("capturing content against {} regexs", regexs.len());

    let mut todos = Vec::new();
    for (line_num, line_result) in content_buf.lines().enumerate() {
        let line = line_result?;

        for re in regexs.iter() {
            if let Some(todo_caps) = re.captures(&line) {
                let content: Cow<str> = match todo_caps.get(2) {
                    Some(user) => Cow::Owned(format!(
                        "@{} {}",
                        user.as_str(),
                        todo_caps.get(3).unwrap().as_str()
                    )),
                    None => Cow::Borrowed(&todo_caps[3]),
                };

                let todo = Todo::new(line_num + 1, &todo_caps[1], content);
                todos.push(todo);
            };
        }
    }

    Ok(todos)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    use crate::comments::CommentTypes;

    fn test_content(content: &str, exp_result: Option<&str>, file_ext: &str) {
        let comment_types = match file_ext {
            "rs" => CommentTypes::new().add_single("//").add_block("/*", "*/"),
            "c" => CommentTypes::new().add_single("//").add_block("/*", "*/"),
            "py" => CommentTypes::new()
                .add_single("#")
                .add_block("\"\"\"", "\"\"\""),
            _ => CommentTypes::new().add_single("//").add_block("/*", "*/"),
        };

        let mut content_buf = Cursor::new(content);
        let todos = parse_content(&mut content_buf, &comment_types, &["TODO".to_string()]).unwrap();

        if todos.is_empty() {
            assert_eq!(exp_result, None);
        } else {
            assert_eq!(exp_result, Some(todos[0].content.as_str()));
        }
    }

    fn test_users(content: &str, exp_content: Option<&str>, exp_users: &[&str], file_ext: &str) {
        let comment_types = match file_ext {
            "rs" => CommentTypes::new().add_single("//").add_block("/*", "*/"),
            "c" => CommentTypes::new().add_single("//").add_block("/*", "*/"),
            "py" => CommentTypes::new()
                .add_single("#")
                .add_block("\"\"\"", "\"\"\""),
            _ => CommentTypes::new().add_single("//").add_block("/*", "*/"),
        };

        let mut content_buf = Cursor::new(content);
        let todos = parse_content(&mut content_buf, &comment_types, &["TODO".to_string()]).unwrap();

        if todos.is_empty() {
            assert_eq!(exp_content, None);
        } else {
            assert_eq!(exp_content, Some(todos[0].content.as_str()));
            assert_eq!(exp_users.len(), todos[0].users().len());
            for user in exp_users {
                assert!(todos[0].tags_user(user));
            }
        }
    }

    #[test]
    fn find_todos_block_and_line1() {
        test_content("/* // todo: item */", None, "rs");
    }

    #[test]
    fn find_todos_block_and_line2() {
        test_content("/* todo: // item */", Some("// item"), "rs");
    }

    #[test]
    fn find_todos_block_and_line3() {
        test_content(" // /* todo: item */", None, "rs");
    }

    #[test]
    fn find_todos_block_and_line4() {
        test_content(" //   todo:  /* item */", Some("/* item */"), "rs");
    }

    #[test]
    fn find_todos_py_in_c_file() {
        test_content("# todo: item \t ", None, "c");
    }

    #[test]
    fn find_todos_c_comment_in_py_comment() {
        test_content("# todo: \\ todo: item \t ", Some("\\ todo: item"), "py");
    }

    #[test]
    fn find_todos_c_comment_in_py_comment_in_c_file() {
        test_content("# todo: \\ todo: item \t ", None, "c");
    }

    #[test]
    fn find_user() {
        test_users("// todo(u): item  \t ", Some("@u item"), &["u"], "c");
    }

    #[test]
    fn find_users() {
        test_users(
            "// todo(u): @u1 item @u2 \t ",
            Some("@u @u1 item @u2"),
            &["u", "u1", "u2"],
            "c",
        );
    }

}
