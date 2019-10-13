// Module for finding TODOs in files

use log::trace;
use regex::Regex;
use std::borrow::Cow;
use std::io::BufRead;

use crate::comments::CommentTypes;
use crate::custom_tags::get_regex_for_comment;
use crate::todo::Todo;

/// Builds Regexs for use with parse_content.
pub fn build_parser_regexs(comment_types: &CommentTypes, tags: &[String]) -> Vec<Regex> {
    comment_types
        .iter()
        .map(|c| get_regex_for_comment(tags, c).unwrap())
        .collect()
}

/// Parses content and creates a list of TODOs found in content
pub fn parse_content<B>(content_buf: &mut B, regexs: &[Regex]) -> Result<Vec<Todo>, std::io::Error>
where
    B: BufRead,
{
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

/// Parses content and creates a list of TODOs found in content. Only adds TODOs that satisfy pred.
pub fn parse_content_with_filter<P>(
    content_buf: &mut impl BufRead,
    regexs: &[Regex],
    pred: P,
) -> Result<Vec<Todo>, std::io::Error>
where
    P: Fn(&Todo) -> bool,
{
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
                if pred(&todo) {
                    todos.push(todo);
                }
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
        let todos = parse_content(
            &mut content_buf,
            &build_parser_regexs(&comment_types, &["TODO".to_string()]),
        )
        .unwrap();

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
        let todos = parse_content(
            &mut content_buf,
            &build_parser_regexs(&comment_types, &["TODO".to_string()]),
        )
        .unwrap();

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
