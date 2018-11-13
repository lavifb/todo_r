// Module for deleting TODO comments from files

use failure::Error;
use std::fs::{File, rename};
use std::io::{BufReader, BufRead, BufWriter, Write};

// use parser::Todo;
use display::TodoFile;

pub fn remove_todo_by_index(todo_file: &mut TodoFile, ind: usize) -> Result<(), Error> {
	assert!(ind < todo_file.todos.len());

	let old_file = File::open(&todo_file.filepath)?;
	let temp_filepath = todo_file.filepath.with_extension("tmp");
	let temp_file = File::create(&temp_filepath)?;
	
	let file_reader = BufReader::new(old_file);
	let mut file_writer = BufWriter::new(temp_file);

	let todo_line = todo_file.todos.remove(ind).line;
	
	let old_lines = file_reader.lines();

	// iterate skipping the line with the TODO
	for line in old_lines.enumerate().filter(|&(i, _)| i != todo_line-1).map(|(_, l)| l) {
		file_writer.write(&line?.into_bytes())?;
	}

	// replace old file with temp file
	rename(temp_filepath, &todo_file.filepath)?;

	Ok(())
}