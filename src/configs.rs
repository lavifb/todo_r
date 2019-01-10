use crate::display::TodoRStyles;
use ansi_term::Color;
use ansi_term::Style;
use failure::Error;
use fnv::FnvHashMap;
use serde::Deserialize;

use crate::comments::{CommentType, CommentTypes};
use crate::errors::TodoRError::InvalidConfigFile;

/// Comments configuration as read from the config file
#[derive(Debug, Default, Clone, Deserialize)]
pub(crate) struct CommentsConfig {
    ext: Option<String>,
    exts: Option<Vec<String>>,
    types: Vec<CommentType>,
}

impl CommentsConfig {
    /// Consume the CommentsConfig type and return its parts
    pub fn break_apart(self) -> (Option<String>, Option<Vec<String>>, CommentTypes) {
        (self.ext, self.exts, self.types.into())
    }
}

/// Style as read from the config file
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum StyleConfig {
    Named(String),
    Fixed(u8),
}

impl StyleConfig {
    /// Converts StyleConfig into ansi_term::Style type
    pub fn into_style(self) -> Result<Style, Error> {
        let style = match self {
            StyleConfig::Named(s) => {
                let mut style_parts = s.rsplit('_');

                let color = style_parts.next().unwrap();
                let mut style_from_string = parse_style_color(color)?;

                for modifier in style_parts {
                    style_from_string = parse_style_modifier(style_from_string, modifier)?;
                }

                style_from_string
            }
            StyleConfig::Fixed(n) => Style::from(Color::Fixed(n)),
        };

        Ok(style)
    }
}

/// Parses color str to get a color Style
fn parse_style_color(color: &str) -> Result<Style, Error> {
    if let Ok(n) = color.parse::<u8>() {
        return Ok(Style::from(Color::Fixed(n)));
    }

    let colored_style = match color.to_uppercase().as_str() {
        "BLACK" => Style::from(Color::Black),
        "RED" => Style::from(Color::Red),
        "GREEN" => Style::from(Color::Green),
        "YELLOW" => Style::from(Color::Yellow),
        "BLUE" => Style::from(Color::Blue),
        "PURPLE" | "MAGENTA" => Style::from(Color::Purple),
        "CYAN" => Style::from(Color::Cyan),
        "WHITE" => Style::from(Color::White),
        "" => Style::new(),
        _ => {
            return Err(InvalidConfigFile {
                message: format!("'{}' is not a valid ANSI color.", color),
            }
            .into())
        }
    };

    Ok(colored_style)
}

/// Parses modifier str to modify a Style and return the result
fn parse_style_modifier(unmodified_style: Style, modifier: &str) -> Result<Style, Error> {
    let style = match modifier.to_uppercase().as_str() {
        "BOLD" | "B" => unmodified_style.bold(),
        "ITALIC" | "I" | "IT" => unmodified_style.italic(),
        "UNDERLINE" | "U" => unmodified_style.underline(),
        _ => {
            return Err(InvalidConfigFile {
                message: format!(
                    "'{}' is not a valid ANSI style modifier. Try using 'b', 'i', or 'u'",
                    modifier
                ),
            }
            .into())
        }
    };

    Ok(style)
}

/// Styles as read from the config file
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct StylesConfig {
    filepath: StyleConfig,
    tag: StyleConfig,
    content: StyleConfig,
    line_number: StyleConfig,
    user: StyleConfig,
    tags: FnvHashMap<String, StyleConfig>,
}

impl Default for StylesConfig {
    fn default() -> StylesConfig {
        StylesConfig {
            filepath: StyleConfig::Named("U_WHITE".to_string()),
            tag: StyleConfig::Named("GREEN".to_string()),
            content: StyleConfig::Named("CYAN".to_string()),
            line_number: StyleConfig::Fixed(8),
            user: StyleConfig::Fixed(8),
            tags: FnvHashMap::default(),
        }
    }
}

impl StylesConfig {
    /// Converts StyleConfig into TodoRStyles type
    pub fn into_todo_r_styles(mut self) -> Result<TodoRStyles, Error> {
        let mut styles = TodoRStyles::new(
            self.filepath.into_style()?,
            self.line_number.into_style()?,
            self.user.into_style()?,
            self.content.into_style()?,
            self.tag.into_style()?,
        );

        for (tag, style_conf) in self.tags.drain() {
            styles = styles.add_tag_style(&tag, style_conf.into_style()?);
        }

        Ok(styles)
    }
}

/// TodoR configuration settings as read from the config file
#[derive(Debug, Default, Clone, Deserialize)]
pub(crate) struct TodoRConfigFileSerial {
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
