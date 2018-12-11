use dialoguer::Select;
use ansi_term::Color::Red;
use failure::Error;
use std::path::Path;
use log::warn;

use todo_r::TodoR;

pub fn run_delete(todor: &mut TodoR) -> Result<(), Error> {
    loop {
        let file_selection = match select_file(&todor) {
            Some(file_selection) => file_selection,
            None => return Ok(()),
        };

        let filepath = Path::new(&file_selection);
        let selected_todo = select_todo(&todor, filepath)?;

        let todo_ind = match selected_todo {
            Some(todo_ind) => todo_ind,
            None => continue,
        };

        todor
            .remove_todo(filepath, todo_ind)
            .unwrap_or_else(|err| warn!("{}", err));
        println!("Comment removed");
    }
}

fn select_file(todor: &TodoR) -> Option<String> {
    let option_quit = format!("{}", Red.paint("QUIT"));
    let mut tracked_files = todor.get_tracked_files();
    tracked_files.push(&option_quit);
    // IMPR: Cache tracked_files for when you go back

    let mut file_selector = Select::new();
    file_selector
        .with_prompt("Pick a file to delete comment")
        .items(&tracked_files)
        .default(0);

    let file_ind = file_selector.interact().unwrap();
    if file_ind + 1 == tracked_files.len() {
        return None;
    }

    Some(tracked_files[file_ind].to_string())
}

fn select_todo(todor: &TodoR, filepath: &Path) -> Result<Option<usize>, Error> {
    let mut todos_buf: Vec<u8> = Vec::new();
    todor.write_todos_from_file(filepath, &mut todos_buf)?;

    let todos_string = String::from_utf8_lossy(&todos_buf);
    let mut todos_lines = todos_string.lines();
    let styled_filename = todos_lines.next().unwrap();

    let option_back = format!("{}", Red.paint("BACK"));
    let mut todos_items: Vec<&str> = todos_lines.collect();
    todos_items.push(&option_back);

    let mut todo_selector = Select::new();
    todo_selector
        .with_prompt(styled_filename)
        .items(&todos_items)
        .default(0);

    let todo_ind = todo_selector.interact().unwrap();
    if todo_ind + 1 == todos_items.len() {
        return Ok(None);
    }

    Ok(Some(todo_ind))
}