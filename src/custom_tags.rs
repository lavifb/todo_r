// Module for creating regexs for custom tags

use regex::Regex;
use std::borrow::Borrow;

use crate::comments::CommentType;

/// Returns Regex that matches TODO comment.
///
/// Optionally, you can specify a user tag in one of two ways:
/// - using perenthesis after the TODO tag such as `// TODO(user): content`.
/// - using @ in the content like this `// TODO: tag @user in this`
///
///
/// The capture groups in the Regex are:
/// 1. TODO tag
/// 2. Optional user tag
/// 3. Content
/// 4. Optional in text user tag
///
pub(crate) fn get_regex_for_comment<S>(
    custom_tags: &[S],
    comment_type: &CommentType,
) -> Result<Regex, regex::Error>
where
    S: Borrow<str>,
{
    let tags_string = custom_tags.join("|");

    // use something like ^\s*\/\/\s*(TODO)\s?(?:\((\S+)\))?[:\s]?\s+((?:.*?@(\S+))?.*?)\s*$
    Regex::new(&format!(
        r"(?i)^\s*{}\s*({})\s?{}[:\s]?\s+{}\s*{}", // whitespace and optional colon
        comment_type.prefix(),                     // comment prefix token
        tags_string,                               // custom tags
        r"(?:\(@?(\S+)\))?",                       // optional user tag in ()`s
        r"((?:.*?@(\S+))?.*?)",                    // content with optional user subcapture
        comment_type.suffix(),                     // comment prefix token
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::comments::CommentType;

    fn test_regex(content: &str, exp_result: Option<&str>, comment_type: &CommentType) {
        let re = get_regex_for_comment(&["TODO", "FIXME"], comment_type).unwrap();
        let todo_content = re.captures(content);
        match todo_content {
            Some(todo_content) => {
                assert_eq!(exp_result, Some(todo_content[3].trim()));
                assert_eq!(None, todo_content.get(2).or(todo_content.get(4)));
            }
            None => assert_eq!(exp_result, None),
        };
    }

    fn test_user_regex(
        content: &str,
        exp_content: Option<&str>,
        exp_user: Option<&str>,
        comment_type: &CommentType,
    ) {
        let re = get_regex_for_comment(&["TODO", "FIXME"], comment_type).unwrap();
        let todo_content = re.captures(content);
        match todo_content {
            Some(todo_content) => {
                assert_eq!(exp_content, Some(todo_content[3].trim()));
                assert_eq!(
                    exp_user,
                    todo_content
                        .get(2)
                        .or(todo_content.get(4))
                        .map(|s| s.as_str())
                );
            }
            None => {
                assert_eq!(exp_content, None);
                assert_eq!(exp_user, None);
            }
        };
    }

    #[test]
    fn regex_whitespace() {
        test_regex(
            "\t\t\t\t  //  TODO:  item \t",
            Some("item"),
            &CommentType::new_single("//"),
        );
    }

    #[test]
    fn regex_todo_in_comment() {
        test_regex(
            "//  TODO:  item // TODO: item \t",
            Some("item // TODO: item"),
            &CommentType::new_single("//"),
        );
    }

    #[test]
    fn regex_optional_colon() {
        test_regex(
            "//  TODO  item // TODO: item \t",
            Some("item // TODO: item"),
            &CommentType::new_single("//"),
        );
    }

    #[test]
    fn regex_case_insensitive() {
        test_regex(
            "// tODo: case ",
            Some("case"),
            &CommentType::new_single("//"),
        );
    }

    #[test]
    fn regex_fixme() {
        test_regex(
            "\t\t\t\t  //  fixMe:  item for fix \t",
            Some("item for fix"),
            &CommentType::new_single("//"),
        );
    }

    #[test]
    fn regex_todop() {
        test_regex("// todop: nope ", None, &CommentType::new_single("//"));
    }

    #[test]
    fn regex_todf() {
        test_regex("// todf: nope ", None, &CommentType::new_single("//"));
    }

    #[test]
    fn regex_todofixme() {
        test_regex("// todofixme : nope ", None, &CommentType::new_single("//"));
    }

    #[test]
    fn regex_py_comment() {
        test_regex(
            "# todo: item \t ",
            Some("item"),
            &CommentType::new_single("#"),
        );
    }

    #[test]
    fn regex_percent_comment() {
        test_regex(
            "% todo: item \t ",
            Some("item"),
            &CommentType::new_single("%"),
        );
    }

    #[test]
    fn regex_ddash_comment() {
        test_regex(
            "-- todo: item \t ",
            Some("item"),
            &CommentType::new_single("--"),
        );
    }

    #[test]
    fn regex_slashstar_comment() {
        test_regex(
            "/* todo: item \t */ \t ",
            Some("item"),
            &CommentType::new_block("/*", "*/"),
        );
    }

    #[test]
    fn regex_slashstar_comment_double_prefix() {
        test_regex(
            "/* todo: item /* todo: decoy*/\t ",
            Some("item /* todo: decoy"),
            &CommentType::new_block("/*", "*/"),
        );
    }

    #[test]
    fn regex_slashstar_comment_double_suffix() {
        test_regex(
            "/* todo: item */ \t other stuff */ ",
            Some("item"),
            &CommentType::new_block("/*", "*/"),
        );
    }

    #[test]
    fn regex_comment_not_on_separate_line() {
        test_regex(
            "do_things(); // todo: item",
            None,
            &CommentType::new_single("//"),
        );
    }

    #[test]
    fn regex_block_todo_before_function() {
        test_regex(
            "/* todo: item */ do_things();",
            Some("item"),
            &CommentType::new_block("/*", "*/"),
        );
    }

    #[test]
    fn regex_basic_user() {
        test_user_regex(
            "// TODO(userA): item",
            Some("item"),
            Some("userA"),
            &CommentType::new_single("//"),
        );
    }

    #[test]
    fn regex_basic_user2() {
        test_user_regex(
            "// TODO: item @userA",
            Some("item @userA"),
            Some("userA"),
            &CommentType::new_single("//"),
        );
    }

    #[test]
    fn regex_basic_user3() {
        test_user_regex(
            "// TODO: @userA   item",
            Some("@userA   item"),
            Some("userA"),
            &CommentType::new_single("//"),
        );
    }

    #[test]
    fn regex_basic_user4() {
        test_user_regex(
            "// TODO: item @userA item2",
            Some("item @userA item2"),
            Some("userA"),
            &CommentType::new_single("//"),
        );
    }

    #[test]
    fn regex_tricky_user() {
        test_user_regex(
            "@ TODO: item @userA item2",
            Some("item @userA item2"),
            Some("userA"),
            &CommentType::new_single("@"),
        );
    }

    #[test]
    fn regex_user_twice() {
        test_user_regex(
            "// TODO(user1): item @user2 item2",
            Some("item @user2 item2"),
            Some("user1"),
            &CommentType::new_single("//"),
        );
    }

    #[test]
    fn regex_user_twice2() {
        test_user_regex(
            "// TODO: item @user1 item2 @user2",
            Some("item @user1 item2 @user2"),
            Some("user1"),
            &CommentType::new_single("//"),
        );
    }

    #[test]
    fn regex_at_in_user() {
        test_user_regex(
            "// TODO: item @user@web.com ",
            Some("item @user@web.com"),
            Some("user@web.com"),
            &CommentType::new_single("//"),
        );
    }

    #[test]
    fn regex_user_block() {
        test_user_regex(
            "/* TODO: item @user */",
            Some("item @user"),
            Some("user"),
            &CommentType::new_block("/*", "*/"),
        );
    }

    #[test]
    fn regex_user_block2() {
        test_user_regex(
            "/* TODO(user): item */",
            Some("item"),
            Some("user"),
            &CommentType::new_block("/*", "*/"),
        );
    }
}
