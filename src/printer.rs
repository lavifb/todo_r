// Module for printing TODOs in various formats

use crate::TodoR;
use failure::Error;
use fnv::FnvHashMap;
use serde_json;
use std::fmt::Write as StringWrite;
use std::io::{self, Write};

// MAYB: add more output formats
/// Enum holding the different supported output formats.
pub enum ReportFormat {
    Json,
    JsonPretty,
    // TODO: add CSV
    Markdown,
    UserMarkdown,
    Default,
}

impl TodoR {
    /// Writes TODOs in TodoR serialized in the JSON format
    fn write_json(&self, out_buffer: &mut impl Write) -> Result<(), Error> {
        serde_json::to_writer(out_buffer, &self)?;
        Ok(())
    }

    /// Writes TODOs in TodoR serialized in a pretty JSON format
    fn write_pretty_json(&self, out_buffer: &mut impl Write) -> Result<(), Error> {
        serde_json::to_writer_pretty(out_buffer, &self)?;
        Ok(())
    }

    /// Writes TODOs in TodoR serialized in a markdown table format.
    /// Tables are organized by TODO tag type.
    fn write_markdown(&self, out_buffer: &mut impl Write) -> Result<(), Error> {
        let mut tag_tables: FnvHashMap<&str, String> = FnvHashMap::default();

        for ptodo in self.iter() {
            let todo = ptodo.todo;
            let table_string = tag_tables.entry(&todo.tag).or_insert_with(|| {
                format!(
                    "### {}s\n| Filename | line | {} |\n|:---|:---:|:---|\n",
                    todo.tag, todo.tag,
                )
            });

            writeln!(
                table_string,
                "| {} | {} | {} |",
                ptodo.file.display(),
                todo.line,
                todo.content,
            )?;
        }

        for table_strings in tag_tables.values() {
            writeln!(out_buffer, "{}", table_strings)?;
        }

        Ok(())
    }

    /// Writes TODOs in TodoR serialized in a markdown table format.
    /// Tables are organized by TODO user.
    fn write_user_markdown(&self, out_buffer: &mut impl Write) -> Result<(), Error> {
        let mut user_tables: FnvHashMap<&str, String> = FnvHashMap::default();
        let mut untagged_todos_string = "".to_string();

        for ptodo in self.iter() {
            let todo = ptodo.todo;
            let users = todo.users();

            if users.is_empty() {
                writeln!(
                    untagged_todos_string,
                    "| {} | {} | {} | {} |",
                    ptodo.file.display(),
                    todo.line,
                    todo.tag,
                    todo.content,
                )?;
            } else {
                for user in users {
                    let table_string = user_tables.entry(user).or_insert_with(|| {
                        format!(
                            "### {}\n| Filename | line | type | content |\n|:---|:---:|:---:|:---|\n",
                            user,
                        )
                    });

                    writeln!(
                        table_string,
                        "| {} | {} | {} | {} |",
                        ptodo.file.display(),
                        todo.line,
                        todo.tag,
                        todo.content,
                    )?;
                }
            }
        }

        for table_strings in user_tables.values() {
            writeln!(out_buffer, "{}", table_strings)?;
        }

        if !untagged_todos_string.is_empty() {
            writeln!(
                out_buffer,
                "### Untagged\n| Filename | line | type | content |\n|:---|:---:|:---:|:---|\n{}",
                untagged_todos_string,
            )?;
        }

        Ok(())
    }

    /// Prints formatted TODOs to stdout.
    pub fn print_formatted_todos(&self, format: &ReportFormat) -> Result<(), Error> {
        // lock stdout to print faster
        let stdout = io::stdout();
        let lock = stdout.lock();
        let mut out_buffer = io::BufWriter::new(lock);

        self.write_formatted_todos(&mut out_buffer, format)
    }

    /// Writes formatted TODOs to out_buffer in the format provided by `report_format`
    pub fn write_formatted_todos(
        &self,
        out_buffer: &mut impl Write,
        out_format: &ReportFormat,
    ) -> Result<(), Error> {
        let formatted_write = match out_format {
            ReportFormat::Json => TodoR::write_json,
            ReportFormat::JsonPretty => TodoR::write_pretty_json,
            ReportFormat::Markdown => TodoR::write_markdown,
            ReportFormat::UserMarkdown => TodoR::write_user_markdown,
            // TODO: make default print have no colors
            ReportFormat::Default => TodoR::write_todos,
        };

        formatted_write(self, out_buffer)
    }
}
