use clap::{App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    App::new("Todo_r")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Lavi Blumberg <lavifb@gmail.com>")
        .about("Lists TODO comments in code.")
        .arg(
            Arg::with_name("FILE")
                .multiple(true)
                .help("File to search for TODO items."),
        )
        .arg(
            Arg::with_name("CONFIG")
                .short("c")
                .long("config")
                .takes_value(true)
                .help("Takes configuration from file."),
        )
        .arg(
            Arg::with_name("NOSTYLE")
                .short("s")
                .long("no-style")
                .help("Prints output with no ansi colors or styles."),
        )
        .arg(
            Arg::with_name("TAGS")
                .short("t")
                .long("tag")
                .takes_value(true)
                .multiple(true)
                .help("Additional TODO tags to search for."),
        )
        .arg(
            Arg::with_name("OVERRIDE_TAGS")
                .short("T")
                .long("override-tag")
                .takes_value(true)
                .multiple(true)
                .help("Overrides default TODO tags to only search custom ones."),
        )
        .arg(
            Arg::with_name("USER")
                .short("u")
                .long("user")
                .takes_value(true)
                .multiple(true)
                .help("Filter TODOs to only feature ones that are tagged with users."),
        )
        .arg(
            Arg::with_name("IGNORE")
                .short("i")
                .long("ignore")
                .takes_value(true)
                .multiple(true)
                .help("Files to be ignored."),
        )
        .arg(
            Arg::with_name("VERBOSE")
                .short("v")
                .long("verbose")
                .help("Provide verbose output."),
        )
        .arg(
            Arg::with_name("CHECK")
                .long("check")
                .help("Exits nicely only if no TODO comments are found."),
        )
        .arg(
            Arg::with_name("FORMAT")
                .short("f")
                .long("format")
                .takes_value(true)
                .possible_values(&[
                    "json",
                    "prettyjson",
                    "markdown",
                    "usermarkdown",
                    "csv",
                    "default",
                ])
                .help("Outputs in specified format."),
        )
        .arg(
            Arg::with_name("DELETE_MODE")
                .short("d")
                .long("delete")
                .conflicts_with("FORMAT")
                .help("Interactive delete mode."),
        )
        .arg(
            Arg::with_name("EXT")
                .short("e")
                .long("ext")
                .takes_value(true)
                .conflicts_with("DELETE_MODE")
                .help("Reads piped content as if it has the provided extention."),
        )
        .subcommand(
            App::new("init")
                .about("Creates .todor config file and defines a todor workspace.")
                .author("Lavi Blumberg <lavifb@gmail.com>"),
        )
}
