// Binary for finding TODOs in specified files

mod clap_app;
mod logger;
mod select;
mod walk;

use clap::ArgMatches;
use env_logger;
use failure::Error;
use log::*;
use std::fs::File;
use std::path::Path;

use todo_r::todo::Todo;
use todo_r::TodoRBuilder;

use self::clap_app::get_cli_matches;
use self::logger::init_logger;
use self::select::run_delete;
use self::walk::build_walker;

/// Parses command line arguments and use TodoR to find TODO comments.
fn main() {
    let matches = get_cli_matches();

    let verbose: bool = matches.is_present("VERBOSE");
    // Set up log output
    init_logger(verbose);

    // Run program
    let exit_code = if matches.subcommand_matches("init").is_some() {
        run_init()
    } else {
        match run(&matches) {
            Ok(code) => code,
            Err(err) => {
                error!("{}", err);
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

    if matches.is_present("NOSTYLE") {
        builder.set_no_style();
    }

    if let Some(ignore_paths_iter) = matches.values_of("IGNORE") {
        builder.add_override_ignore_paths(ignore_paths_iter)?;
    }

    let mut pred = None;
    if let Some(users_iter) = matches.values_of("USER") {
        // let users: Vec<String> = users_iter.map(|s| s.to_string()).collect();
        let users: Vec<&str> = users_iter.collect();
        pred = Some(move |t: &Todo| users.iter().any(|u| t.tags_user(*u)));
    }

    let mut todor;
    match matches.values_of("FILE") {
        Some(files) => {
            todor = builder.build()?;
            debug!("todor parser built");
            for file in files {
                info!("looking at `{}`...", file);
                todor
                    .open_todos(Path::new(file))
                    .unwrap_or_else(|err| warn!("{}", err));
            }
        }
        None => {
            info!("Looking for .git or .todor to use as workspace root...");
            let walk = build_walker(&mut builder)?;
            todor = builder.build()?;
            debug!("todor parser built");

            for entry in walk {
                let dir_entry = entry?;
                let path = dir_entry.path().strip_prefix(".").unwrap();

                debug!("found {} in walk", path.display());

                if path.is_file() {
                    info!("looking at `{}`...", path.display());
                    todor
                        .open_todos(path)
                        .unwrap_or_else(|err| warn!("{}", err));
                }
            }
        }
    }

    if matches.is_present("DELETE_MODE") {
        run_delete(&mut todor)?;
    } else {
        if let Some(p) = pred {
            todor.print_filtered_todos(&p);
        } else {
            todor.print_todos();
        }
    }

    if matches.is_present("CHECK") && todor.num_todos() > 0 {
        return Ok(1);
    }

    Ok(0)
}

fn run_init() -> i32 {
    let mut config_file = match File::create(Path::new(".todor")) {
        Ok(file) => file,
        Err(err) => {
            error!("{}", err);
            return 1;
        }
    };

    match todo_r::write_example_config(&mut config_file) {
        Ok(_) => 0,
        Err(err) => {
            error!("{}", err);
            1
        }
    }
}
