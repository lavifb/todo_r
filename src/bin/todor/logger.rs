use std::io::Write;
use env_logger::fmt::Formatter;
use log::Record;

pub fn init_logger(verbose: bool) {
    let log_env = if verbose {
        env_logger::Env::default().default_filter_or("info")
    } else {
        env_logger::Env::default().default_filter_or("error")
    };

    env_logger::Builder::from_env(log_env)
        .format(todor_fmt)
        .target(env_logger::Target::Stderr)
        .init();
}

fn todor_fmt(buf: &mut Formatter, record: &Record) -> std::io::Result<()> {
    let mut style = buf.style();

    match record.level() {
        log::Level::Error => style.set_color(env_logger::fmt::Color::Red),
        log::Level::Warn => style.set_color(env_logger::fmt::Color::Yellow),
        _ => style.set_color(env_logger::fmt::Color::White),
    };

    let log_prefix = match record.module_path() {
        Some(mod_path) => format!("[{} {}]", mod_path, record.level()),
        None => format!("[{}]", record.level()),
    };

    writeln!(buf, "{}: {}", style.value(log_prefix), record.args())
}