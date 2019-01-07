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
    Markdown,
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

    /// Writes TODOs in TodoR serialized in a markdown format
    fn write_markdown(&self, out_buffer: &mut impl Write) -> Result<(), Error> {
        let mut tag_tables: FnvHashMap<String, String> = FnvHashMap::default();

        for ptodo in self.iter() {
            let todo = ptodo.todo;
            let table_string = tag_tables.entry(todo.tag.clone()).or_insert_with(|| {
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
            ReportFormat::Default => TodoR::write_todos,
        };

        formatted_write(self, out_buffer)
    }
}
