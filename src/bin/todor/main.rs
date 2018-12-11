// Binary for finding TODOs in specified files

mod select;
mod clap_app;
mod logger;

use clap::ArgMatches;
use env_logger;
use failure::{format_err, Error};
use ignore::overrides::OverrideBuilder;
use ignore::WalkBuilder;
use log::*;
use std::env::*;
use std::fs::File;
use std::path::{self, Path, PathBuf};

use todo_r::TodoRBuilder;

use self::clap_app::get_cli_matches;
use self::select::run_delete;
use self::logger::init_logger;

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
            // TODO: pull out walk building into smaller function
            // Recurse down and try to find either .git or .todor as the root folder
            info!("Looking for .git or .todor to use as workspace root...");

            let mut curr_dir = current_dir()?;
            let mut ignore_builder = OverrideBuilder::new(&curr_dir);
            curr_dir.push(".todor");
            let mut relative_path = PathBuf::from(".");
            let mut walk_builder = WalkBuilder::new(&relative_path);
            let mut found_walker_root = false;

            for path in curr_dir.ancestors() {
                let ignore_path = relative_path.strip_prefix(".").unwrap().with_file_name(
                    path.file_name().ok_or_else(|| {
                        format_err!(
                            "No input files provided and no git repo or todor workspace found"
                        )
                    })?,
                );

                let ignore_path_str = ignore_path.to_str().ok_or_else(|| {
                    format_err!(
                        "Path `{}` contains invalid Unicode and cannot be processed",
                        ignore_path.to_string_lossy()
                    )
                })?;

                let ignore_string = if path::MAIN_SEPARATOR != '/' {
                    format!(
                        "!{}",
                        ignore_path_str.replace(&path::MAIN_SEPARATOR.to_string(), "/")
                    )
                } else {
                    format!("!{}", ignore_path_str)
                };

                debug!("adding {} in walker override", &ignore_string);
                ignore_builder.add(&ignore_string).unwrap();

                let todor_path = path.with_file_name(".todor");
                if todor_path.exists() {
                    found_walker_root = true;
                    info!("Found workspace root: '{}'", todor_path.display());
                    info!("Applying config file '{}'...", todor_path.display());

                    // check for empty file before adding
                    if todor_path.metadata().unwrap().len() > 2 {
                        builder.add_config_file(&todor_path)?;
                    }
                    break;
                }

                let git_path = path.with_file_name(".git");
                if git_path.exists() {
                    found_walker_root = true;
                    info!("Found workspace root: '{}'", git_path.display());
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
