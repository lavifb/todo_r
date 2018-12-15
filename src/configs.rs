use crate::display::TodoRStyles;
use ansi_term::Color;
use ansi_term::Style;
use log::debug;
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

// TODO: use Try_into
impl Into<Style> for StyleConfig {
    fn into(self) -> Style {
        match self {
            StyleConfig::Named(s) => match s.to_uppercase().as_str() {
                "BLACK" => Style::from(Color::Black),
                "RED" => Style::from(Color::Red),
                "GREEN" => Style::from(Color::Green),
                "YELLOW" => Style::from(Color::Yellow),
                "BLUE" => Style::from(Color::Blue),
                "PURPLE" => Style::from(Color::Purple),
                "CYAN" => Style::from(Color::Cyan),
                "WHITE" => Style::from(Color::White),
                _ => {
                    debug!("invalid color choice");
                    Style::from(Color::White)
                }
            },
            StyleConfig::Fixed(n) => Style::from(Color::Fixed(n)),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct StylesConfig {
    tag: StyleConfig,
    content: StyleConfig,
    line_number: StyleConfig,
    user: StyleConfig,
}

impl Default for StylesConfig {
    fn default() -> StylesConfig {
        StylesConfig {
            tag: StyleConfig::Named("GREEN".to_string()),
            content: StyleConfig::Named("CYAN".to_string()),
            line_number: StyleConfig::Fixed(8),
            user: StyleConfig::Fixed(8),
        }
    }
}

impl Into<TodoRStyles> for StylesConfig {
    fn into(self) -> TodoRStyles {
        TodoRStyles::new(
            Style::new().underline(),
            self.line_number.into(),
            self.user.into(),
            self.content.into(),
            self.tag.into(),
        )
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
