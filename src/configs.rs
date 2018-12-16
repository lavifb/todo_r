use crate::display::TodoRStyles;
use ansi_term::Color;
use ansi_term::Style;
use failure::{format_err, Error};
use serde::Deserialize;

use crate::comments::CommentType;
use crate::comments::CommentTypes;

#[derive(Debug, Default, Clone, Deserialize)]
pub(crate) struct CommentsConfig {
    #[serde(default)]
    pub ext: String,
    #[serde(default)]
    pub exts: Vec<String>,
    pub(self) types: Vec<CommentType>,
}

impl CommentsConfig {
    pub fn break_apart(self) -> (String, Vec<String>, CommentTypes) {
        (self.ext, self.exts, self.types.into())
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum StyleConfig {
    Named(String),
    Fixed(u8),
}

impl StyleConfig {
    pub fn into_style(self) -> Result<Style, Error> {
        let style = match self {
            StyleConfig::Named(s) => {
                let mut style_parts = s.rsplit("_");

                let color = style_parts.next().unwrap();
                let mut style_from_string = match color.to_uppercase().as_str() {
                    "BLACK" => Style::from(Color::Black),
                    "RED" => Style::from(Color::Red),
                    "GREEN" => Style::from(Color::Green),
                    "YELLOW" => Style::from(Color::Yellow),
                    "BLUE" => Style::from(Color::Blue),
                    "PURPLE" => Style::from(Color::Purple),
                    "CYAN" => Style::from(Color::Cyan),
                    "WHITE" => Style::from(Color::White),
                    _ => return Err(format_err!("'{}' is not a valid ANSI color.", color)),
                };

                for modifier in style_parts {
                    match modifier.to_uppercase().as_str() {
                        "BOLD" | "B" => {
                            style_from_string = style_from_string.bold();
                        }
                        "ITALIC" | "I" | "IT" => {
                            style_from_string = style_from_string.italic();
                        }
                        "UNDERLINE" | "U" => {
                            style_from_string = style_from_string.underline();
                        }
                        _ => {
                            return Err(format_err!(
                            "'{}' is not a valid ANSI style modifier. Try using 'b', 'i', or 'u'",
                            modifier
                        ))
                        }
                    }
                }

                style_from_string
            }
            StyleConfig::Fixed(n) => Style::from(Color::Fixed(n)),
        };

        Ok(style)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct StylesConfig {
    filepath: StyleConfig,
    tag: StyleConfig,
    content: StyleConfig,
    line_number: StyleConfig,
    user: StyleConfig,
}

impl Default for StylesConfig {
    fn default() -> StylesConfig {
        StylesConfig {
            filepath: StyleConfig::Named("U_WHITE".to_string()),
            tag: StyleConfig::Named("GREEN".to_string()),
            content: StyleConfig::Named("CYAN".to_string()),
            line_number: StyleConfig::Fixed(8),
            user: StyleConfig::Fixed(8),
        }
    }
}

impl StylesConfig {
    pub fn into_todo_r_styles(self) -> Result<TodoRStyles, Error> {
        let styles = TodoRStyles::new(
            self.filepath.into_style()?,
            self.line_number.into_style()?,
            self.user.into_style()?,
            self.content.into_style()?,
            self.tag.into_style()?,
        );

        Ok(styles)
    }
}

#[derive(Debug, Default, Clone, Deserialize)]
pub(crate) struct TodorConfigFileSerial {
    #[serde(default)]
    pub verbose: bool,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub ignore: Vec<String>,
    #[serde(default)]
    pub default_ext: String,
    #[serde(default)]
    pub default_comments: Vec<CommentsConfig>,
    #[serde(default)]
    pub comments: Vec<CommentsConfig>,
    #[serde(default)]
    pub styles: StylesConfig,
}
