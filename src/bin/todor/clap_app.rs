use clap::{clap_app, App, Arg};

pub fn build_cli() -> App<'static, 'static> {
    // TODO: rewrite to not use the clunky macro
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
        );

    clap_app!(todo_r =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: "Lavi Blumberg <lavifb@gmail.com>")
        (about: "Lists TODO comments in code.")
        (@arg FILE: ... "File to search for TODO items.")
        (@arg CONFIG: -c --("config") +takes_value "Takes configuration from file.")
        (@arg NOSTYLE: -s --("no-style") "Prints output with no ansi colors or styles.")
        (@arg TAGS: -t --("tag") +takes_value +multiple "TODO tags to search for.")
        (@arg OVERRIDE_TAGS: -T --("override-tags") +takes_value +multiple
            "Overrides default TODO tags to only search custom ones.")
        (@arg USER: -u --("user") +takes_value +multiple
            "Filter output to only feature provided users.")
        (@arg IGNORE: -i --("ignore") +takes_value +multiple "Paths to be ignored.")
        (@arg VERBOSE: -v --("verbose") "Provide verbose output.")
        (@arg CHECK: --("check") "Exits nicely only if no TODO tags are found.")
        (@arg FORMAT: -f --("format") conflicts_with[DELETE_MODE] +takes_value
            possible_values(&["json", "prettyjson", "markdown", "usermarkdown", "csv", "default"])
            "Outputs TODOs in specified formats.")
        (@arg DELETE_MODE: -d --("delete") conflicts_with[FORMAT] conflicts_with[EXT]
            "Interactive delete mode.")
        (@arg EXT: -e --("ext") conflicts_with[DELETE_MODE] +takes_value
            "Reads piped content as if it has the provided extantion.")
        (@subcommand init =>
            (about: "Creates .todor config file and defines todor workspace.")
            (author: "Lavi Blumberg <lavifb@gmail.com>")
        )
    )
}
