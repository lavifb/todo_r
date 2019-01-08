// Binary for finding TODOs in specified files

mod clap_app;
mod logger;
mod select;
mod walk;

use clap::ArgMatches;
use config::FileFormat;
use dirs::home_dir;
use env_logger;
use failure::{format_err, Error};
use ignore::overrides::OverrideBuilder;
use log::*;
use std::env::current_dir;
use std::fs::File;
use std::path::Path;

use todo_r::printer::ReportFormat;
use todo_r::todo::Todo;
use todo_r::TodoRBuilder;

use self::clap_app::build_cli;
use self::logger::init_logger;
use self::select::run_delete;
use self::walk::build_walker;

/// Parses command line arguments and use TodoR to find TODO comments.
fn main() {
    let matches = build_cli().get_matches();

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

    // Search for global config file
    load_global_config(&mut builder)?;

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

    let curr_dir = current_dir()?;
    let mut ignore_builder = OverrideBuilder::new(&curr_dir);
    if let Some(ignore_paths_iter) = matches.values_of("IGNORE") {
        for ignore_path in ignore_paths_iter {
            ignore_builder.add(&format!("!{}", ignore_path))?;
        }
    }

    let pred = if let Some(users_iter) = matches.values_of("USER") {
        let users: Vec<&str> = users_iter.collect();
        Some(move |t: &Todo| users.iter().any(|u| t.tags_user(*u)))
    } else {
        None
    };

    let mut todor;
    match matches.values_of("FILE") {
        Some(files) => {
            let ignores = ignore_builder.build()?;
            todor = builder.build()?;
            debug!("todor parser built");
            for file in files {
                info!("looking at `{}`...", file);

                if !ignores.matched(file, false).is_ignore() {
                    todor
                        .open_option_filtered_todos(file, &pred)
                        .unwrap_or_else(|err| warn!("{}", err));
                }
            }
        }
        None => {
            info!("Looking for .git or .todor to use as workspace root...");
            let walk = build_walker(&mut builder, ignore_builder)?;
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
    } else if let Some(format) = matches.value_of("FORMAT") {
        let report_format = match format {
            "json" => ReportFormat::Json,
            "prettyjson" => ReportFormat::JsonPretty,
            "markdown" => ReportFormat::Markdown,
            "usermarkdown" => ReportFormat::UserMarkdown,
            "csv" => ReportFormat::Csv,
            "default" => ReportFormat::Default,
            _ => return Err(format_err!("invalid output format: {}.", format)),
        };

        todor.print_formatted_todos(&report_format)?;
    } else {
        todor.print_todos();
    }

    if matches.is_present("CHECK") && todor.num_todos() > 0 {
        return Ok(1);
    }

    Ok(0)
}

fn load_global_config(builder: &mut TodoRBuilder) -> Result<(), Error> {
    if let Some(mut global_conf) = home_dir() {
        if cfg!(windows) {
            global_conf.push(r"AppData\Roaming\lavifb\todor\todor.conf");
        } else {
            global_conf.push(".config/todor/todor.conf");
        }

        info!("searching for global config in '{}'", global_conf.display());
        if global_conf.exists() && global_conf.metadata().unwrap().len() > 2 {
            info!("adding global config file...");
            builder.add_config_file_with_format(global_conf, FileFormat::Hjson)?;
        }
    }

    Ok(())
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
