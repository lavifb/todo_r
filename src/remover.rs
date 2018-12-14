// Module for deleting TODO comments from files

use failure::Error;
use log::debug;
use std::fs::{rename, File};
use std::io::{BufRead, BufReader, BufWriter, Write};

use crate::errors::TodoRError;
use crate::todo::TodoFile;

pub fn remove_todo_by_index(todo_file: &mut TodoFile, ind: usize) -> Result<(), Error> {
    assert!(ind < todo_file.todos.len());

    let old_file = File::open(&todo_file.filepath)?;
    let temp_filepath = todo_file.filepath.with_extension("tmp");
    let temp_file = File::create(&temp_filepath)?;

    let mut file_reader = BufReader::new(old_file);
    let mut file_writer = BufWriter::new(temp_file);

    let todo_line = todo_file.todos.remove(ind).line;

    debug!(
        "removing content in `{}` on line {}",
        todo_file.filepath.display(),
        todo_line
    );
    copy_except_line(&mut file_reader, &mut file_writer, todo_line)?;

    for todo in &mut todo_file.todos[ind..] {
        todo.line -= 1;
    }

    // replace old file with temp file
    rename(temp_filepath, &todo_file.filepath)?;

    Ok(())
}

pub fn remove_todo_by_line(todo_file: &mut TodoFile, line: usize) -> Result<(), Error> {
    let del_index: usize;
    // Kind of annoyed there is no retain_mut() to make this easier...
    {
        let mut todos = todo_file
            .todos
            .iter_mut()
            .enumerate()
            .skip_while(|(_, t)| t.line < line);

        let (i, todo_to_check) = match todos.next() {
            Some((i, todo_to_check)) => (i, todo_to_check),
            None => return Err(TodoRError::TodoNotFound { line }.into()),
        };

        // line not found in TODOs
        if todo_to_check.line > line {
            return Err(TodoRError::TodoNotFound { line }.into());
        }

        del_index = i;

        let old_file = File::open(&todo_file.filepath)?;
        let temp_filepath = todo_file.filepath.with_extension("tmp");
        let temp_file = File::create(&temp_filepath)?;

        let mut file_reader = BufReader::new(old_file);
        let mut file_writer = BufWriter::new(temp_file);

        debug!(
            "removing content in `{}` on line {}",
            todo_file.filepath.display(),
            line
        );
        copy_except_line(&mut file_reader, &mut file_writer, line)?;

        // replace old file with temp file
        rename(temp_filepath, &todo_file.filepath)?;

        for (_, todo) in todos {
            todo.line -= 1;
        }
    }

    todo_file.todos.remove(del_index);

    Ok(())
}

fn copy_except_line<B, W>(orig: &mut B, copy: &mut W, line_number: usize) -> Result<(), Error>
where
    B: BufRead,
    W: Write,
{
    let orig_lines = orig.lines();
    let mut line_skip_iter = orig_lines
        .enumerate()
        .filter(|&(i, _)| i != line_number - 1)
        .map(|(_, l)| l);

    // First line needs no '\n' char
    {
        let first_line = match line_skip_iter.next() {
            Some(first_line) => first_line?,
            None => return Ok(()), // Input is empty
        };

        copy.write_all(&first_line.into_bytes())?;
    }

    // iterate skipping the line with the TODO
    for line in line_skip_iter {
        let l: String = line?;
        copy.write_all(b"\n")?;
        copy.write_all(&l.into_bytes())?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn assert_copy(orig_text: &str, expected_out_text: &str, todo_line: usize) {
        let mut out_buf: Cursor<Vec<u8>> = Cursor::new(Vec::new());
        let mut in_buf = Cursor::new(orig_text);

        copy_except_line(&mut in_buf, &mut out_buf, todo_line).unwrap();

        let out_bytes = out_buf.into_inner();
        assert_eq!(
            expected_out_text.to_string(),
            String::from_utf8(out_bytes).unwrap()
        );
    }

    #[test]
    fn test_remove_line3() {
        let todo_line = 3;
        let orig_text = "code.run()
// regular comment
// item
// item2
other.stuff()
// another comment";

        let expected_out_text = "code.run()
// regular comment
// item2
other.stuff()
// another comment";

        assert_copy(orig_text, expected_out_text, todo_line);
    }

    #[test]
    fn test_remove_line1() {
        let todo_line = 1;
        let orig_text = "code.run()
// regular comment
// item
// item2
other.stuff()
// another comment";

        let expected_out_text = "// regular comment
// item
// item2
other.stuff()
// another comment";

        assert_copy(orig_text, expected_out_text, todo_line);
    }

    #[test]
    fn test_remove_line_last() {
        let todo_line = 6;
        let orig_text = "code.run()
// regular comment
// item
// item2
other.stuff()
// another comment";

        let expected_out_text = "code.run()
// regular comment
// item
// item2
other.stuff()";

        assert_copy(orig_text, expected_out_text, todo_line);
    }

    #[test]
    fn test_remove_line_out_of_bounds() {
        let todo_line = 8;
        let orig_text = "code.run()
// regular comment
// item
// item2
other.stuff()
// another comment";

        let expected_out_text = "code.run()
// regular comment
// item
// item2
other.stuff()
// another comment";

        assert_copy(orig_text, expected_out_text, todo_line);
    }
}
