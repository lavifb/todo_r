use clap::{clap_app, ArgMatches};

pub fn get_cli_matches<'a>() -> ArgMatches<'a> {
    // TODO: add subcommand for just content so it can be piped
    clap_app!(todo_r =>
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
    .get_matches()
}
