use clap::{App, Arg};

#[cfg(windows)]
macro_rules! global_config_path {
    () => {
        r"~\AppData\Roaming\lavifb\todor\todor.conf"
    };
}
#[cfg(unix)]
macro_rules! global_config_path {
    () => {
        r"$XDG_CONFIG_HOME/todor/todor.conf or ~/.config/todor/todor.conf"
    };
}

pub fn build_cli() -> App<'static, 'static> {
    App::new("Todo_r")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Lavi Blumberg <lavifb@gmail.com>")
        .about("Lists TODO comments in code.")
        .arg(
            Arg::with_name("FILE")
                .multiple(true)
                .help("Sets todor to only search in provided files."),
        )
        .arg(
            Arg::with_name("CONFIG")
                .short("c")
                .long("config")
                .takes_value(true)
                .help("Takes config from file.")
                .long_help(concat!(
                    "Takes configuration from file. This file should be in a JSON format and \
                    allows todor to be customized by adding new comment types for extensions and \
                    custom colors. An example file called .todor can be created by using the \
                    `todor init` command. \
                    \n\n\
                    You can also set a global config file at `",
                    global_config_path!(),
                    "`.")
                )
        )
        .arg(
            Arg::with_name("NOSTYLE")
                .short("s")
                .long("no-style")
                .help("Prints output with no ANSI colors or styles."),
        )
        .arg(
            Arg::with_name("TAGS")
                .short("t")
                .long("tag")
                .takes_value(true)
                .multiple(true)
                .help("Additional TODO tags to search for.")
                .long_help(
                    "Adds additional tags to search for over the ones provided by default and any \
                    config files. \nFor example, to add MAYB and NOW tags to your search, use \n\n\
                    \t> todor -t mayb now\n\n\
                    to find them. This will also find tags defined in any config files."
                ),
        )
        .arg(
            Arg::with_name("OVERRIDE_TAGS")
                .short("T")
                .long("override-tag")
                .takes_value(true)
                .multiple(true)
                .help("Overrides default TODO tags to only search custom ones.")
                .long_help(
                    "Works the same as `-t` except tags in default and config are not searched for.\
                    Thus, only tags explicitly passed after this flag are considered."
                ),
        )
        .arg(
            Arg::with_name("USER")
                .short("u")
                .long("user")
                .takes_value(true)
                .multiple(true)
                .help("Filter TODOs to only feature ones that are tagged with users.")
                .long_help(
                    "Only searches for TODOs that include provided users.\n\
                    For example, to only print TODOs with user1 and user2, use \n\n\
                    \t> todor -u user1 user2\n\n"
                ),
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
                .help("Outputs in specified format.")
                .long_help(
                    "Outputs in specified format. The following formats are supported:\n\n\
                    json: compacted JSON\n\
                    prettyjson: nicely formatted JSON\n\
                    markdown: Markdown tables with a table for each tag type\n\
                    usermarkdown: Markdown tables for each user\n\
                    csv: Comma separated values table\n\
                    default: regular output with no ANSI colors for "
                ),
        )
        .arg(
            Arg::with_name("DELETE_MODE")
                .short("d")
                .long("delete")
                .conflicts_with("FORMAT")
                .help("Interactive delete mode.")
                .long_help(
                    "Runs todor and lets you delete TODO comments interactively. First you select \
                    which file to delete from and then pick which comment to delete."
                ),
        )
        .arg(
            Arg::with_name("EXT")
                .short("e")
                .long("ext")
                .takes_value(true)
                .conflicts_with("DELETE_MODE")
                .help("Reads piped content as if it has the provided extention.")
                .long_help(
                    "Reads piped content as if it has the provided extention. For example, \n\n\
                    \t> cat test.rs | todor -e rs\n\n\
                    will take the piped output from cat and read using the .rs comment styles."
                ),
        )
        .subcommand(
            App::new("init")
                .about("Creates .todor config file and defines a todor workspace.")
                .author("Lavi Blumberg <lavifb@gmail.com>"),
        )
}
