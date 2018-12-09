// Binary for finding TODOs in specified files

use ansi_term::Color::Red;
use clap::clap_app;
use clap::ArgMatches;
use dialoguer::Select;
use failure::{format_err, Error};
use ignore::overrides::OverrideBuilder;
use ignore::WalkBuilder;
use std::env::*;
use std::fs::File;
use std::path::{Path, PathBuf};
use log::*;
use env_logger;

use todo_r::{TodoR, TodoRBuilder};

/// Prints error message to stderr using a red identifier.
pub fn eprint_error(err: &Error) {
    match err {
        _ => eprintln!("{}: {}", Red.paint("[todor error]"), err.to_string()),
    };
}

/// Parses command line arguments and use TodoR to find TODO comments.
fn main() {
    env_logger::init();

    // TODO: add subcommand for just content so it can be piped
    let matches = clap_app!(todo_r =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: "Lavi Blumberg <lavifb@gmail.com>")
        (about: "Lists TODO comments in code")
        (@arg FILE: ... "File to search for TODO items.")
        (@arg CONFIG: -c --("config") +takes_value "Takes configuration from file.")
        (@arg NOSTYLE: -s --("no-style") "Prints output with no ansi colors or styles.")
        (@arg TAGS: -t --("tag") +takes_value +multiple "TODO tags to search for.")
        (@arg IGNORE: -i --("ignore") +takes_value +multiple "Paths to be ignored.")
        (@arg OVERRIDE_TAGS: -T --("override-tags") +takes_value +multiple
            "Overrides default TODO tags to only search custom ones.")
        (@arg VERBOSE: -v --("verbose") "Provide verbose output.")
        (@arg CHECK: --("check") "Exits nicely only if no TODO tags are found.")
        (@arg DELETE_MODE: -d --("delete") "Interactive delete mode.")
        (@subcommand init =>
            (about: "Creates example config file")
            (author: "Lavi Blumberg <lavifb@gmail.com>")
        )
    )
    .get_matches();

    let exit_code = if matches.subcommand_matches("init").is_some() {
        run_init()
    } else {
        match run(&matches) {
            Ok(code) => code,
            Err(err) => {
                eprint_error(&err);
                1
            }
        }
    };

    std::process::exit(exit_code);
}

fn run(matches: &ArgMatches) -> Result<i32, Error> {
    let mut builder = TodoRBuilder::new();
    // TODO: serach for default config file in ~/.config/todor

    if let Some(config_path) = matches.value_of("CONFIG") {
        builder.add_config_file(Path::new(config_path))?;
    };

    if let Some(tags_iter) = matches.values_of("TAGS") {
        builder.add_tags(tags_iter);
    }

    if let Some(tags_iter) = matches.values_of("OVERRIDE_TAGS") {
        builder.add_override_tags(tags_iter);
    }

    let verbose: bool = matches.is_present("VERBOSE");
    if verbose {
        builder.set_verbose(verbose);
    }

    if matches.is_present("NOSTYLE") {
        builder.set_no_style();
    }

    if let Some(ignore_paths_iter) = matches.values_of("IGNORE") {
        builder.add_override_ignore_paths(ignore_paths_iter)?;
    }

    let mut todor;
    match matches.values_of("FILE") {
        Some(files) => {
            todor = builder.build()?;
            for file in files {
                todor
                    .open_todos(Path::new(file))
                    .unwrap_or_else(|err| eprint_error(&err));
            }
        }
        None => {
            // Recurse down and try to find either .git or .todor as the root folder
            if verbose {
                println!("Looking for .git or .todor to use as workspace root...");
            }

            let mut curr_dir = current_dir()?;
            let mut ignore_builder = OverrideBuilder::new(&curr_dir);
            curr_dir.push(".todor");
            let mut relative_path = PathBuf::from(".");
            let mut walk_builder = WalkBuilder::new(&relative_path);
            let mut found_walker_root = false;

            for path in curr_dir.ancestors() {
                let ignore_path = relative_path
                    .strip_prefix(".")
                    .unwrap()
                    .with_file_name(path.file_name().ok_or_else(|| {
                        format_err!(
                            "No input files provided and no git repo or todor workspace found"
                        )
                    })?)
                    .to_string_lossy()
                    .into_owned();
                ignore_builder.add(&format!("!{}", &ignore_path)).unwrap();

                let todor_path = path.with_file_name(".todor");
                if todor_path.exists() {
                    found_walker_root = true;
                    if verbose {
                        println!("Found workspace root: '{}'", todor_path.display());
                        println!("Applying config file '{}'...", todor_path.display());
                    }

                    // check for empty file before adding
                    if todor_path.metadata().unwrap().len() > 2 {
                        builder.add_config_file(&todor_path)?;
                    }
                    break;
                }

                let git_path = path.with_file_name(".git");
                if git_path.exists() {
                    found_walker_root = true;
                    if verbose {
                        println!("Found workspace root: '{}'", git_path.display());
                    }
                    break;
                }

                relative_path.push("..");
                walk_builder.add(&relative_path);
            }

            if !found_walker_root {
                return Err(format_err!(
                    "No input files provided and no git repo or todor workspace found"
                ));
            }

            walk_builder
                .overrides(ignore_builder.build()?)
                .sort_by_file_name(std::ffi::OsStr::cmp)
                .add_custom_ignore_filename(".todorignore")
                .parents(false);
            let walk = walk_builder.build();
            todor = builder.build()?;

            for entry in walk {
                let dir_entry = entry?;
                let path = dir_entry.path().strip_prefix("./").unwrap();

                debug!("walking: {}", path.display());

                if path.is_file() {
                    todor
                        .open_todos(path)
                        .unwrap_or_else(|err| eprint_error(&err));
                }
            }
        }
    }

    if matches.is_present("DELETE_MODE") {
        loop {
            let file_selection = match select_file(&todor) {
                Some(file_selection) => file_selection,
                None => return Ok(0),
            };

            let filepath = Path::new(&file_selection);
            let selected_todo = select_todo(&todor, filepath)?;

            let todo_ind = match selected_todo {
                Some(todo_ind) => todo_ind,
                None => continue,
            };

            todor
                .remove_todo(filepath, todo_ind)
                .unwrap_or_else(|err| eprint_error(&err));
            println!("Comment removed");
        }
    } else {
        todor.print_todos();
    }

    if matches.is_present("CHECK") && todor.num_todos() > 0 {
        return Ok(1);
    }

    Ok(0)
}

fn run_init() -> i32 {
    let mut config_file = match File::create(Path::new(".todor")) {
        Ok(file) => file,
        Err(e) => {
            eprint_error(&e.into());
            return 1;
        }
    };

    match todo_r::write_example_config(&mut config_file) {
        Ok(_) => 0,
        Err(err) => {
            eprint_error(&err);
            1
        }
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
