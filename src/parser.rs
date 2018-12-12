// Module for finding TODOs in files

use ansi_term::Style;
use log::trace;
use regex::Regex;
use std::fmt;
use std::io::BufRead;

use crate::comments::CommentTypes;
use crate::custom_tags::get_regex_for_comment;

/// A struct holding the TODO and all the needed meta-information for it.
#[derive(Debug, Clone)]
pub struct Todo {
    pub line: usize,
    tag: String,
    content: String,
    user: Option<String>,
    // TODO: add slices that represent all in-text users 
}

impl Todo {
    /// Create new TODO struct
    fn new(line: usize, tag_str: &str, content_str: &str, user_str: Option<&str>) -> Todo {
        Todo {
            line,
            tag: tag_str.to_uppercase(),
            content: content_str.to_string(),
            user: user_str.map(|u| format!("@{}", u)),
        }
    }

    /// Returns colored output string
    // TODO: style for tagged users 
    pub fn style_string(
        &self,
        line_style: &Style,
        todo_style: &Style,
        content_style: &Style,
    ) -> String {

        let content_out: String = match &self.user {
            Some(user) => format!("{}{}{} {}", line_style.prefix(), user, line_style.infix(*content_style), &self.content),
            None => content_style.paint(&self.content).to_string(),
        };

        format!(
            "  {}  {}  {}",
            // Columns align for up to 100,000 lines which should be fine
            line_style.paint(format!("line {:<5}", self.line)),
            todo_style.paint(format!("{:5}", &self.tag)),
            content_style.paint(content_out),
        )

        // Test(user): item
        // Test: item @me woo
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "line {}\t{}\t{}", self.line, self.tag, self.content,)
    }
}

/// Parses content and Creates a list of TODOs found in content
pub(crate) fn parse_content<B>(
    content_buf: &mut B,
    comment_types: &CommentTypes,
    tags: &[String],
) -> Result<Vec<Todo>, std::io::Error>
where
    B: BufRead,
{
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
                // TODO: stick todo_caps[2] at the front of content
                // TODO: store locations of users in content for painting in output
                let todo = Todo::new(
                    line_num + 1,
                    todo_caps[1].trim(),
                    todo_caps[3].trim(),
                    todo_caps.get(2).or(todo_caps.get(4)).map(|s| s.as_str().trim()),
                );
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

    // TODO: add tests for user
}
