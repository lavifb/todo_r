use clap::Shell;
use std::env;
use std::fs;

include!("src/bin/todor/clap_app.rs");

fn main() {
    let outdir = match env::var_os("OUT_DIR") {
        Some(outdir) => outdir,
        None => {
            println!("Environment variable OUT_DIR not found.");
            std::process::exit(1);
        }
    };

    fs::create_dir_all(&outdir).unwrap();

    let mut app = build_cli();
    app.gen_completions("todor", Shell::Bash, &outdir);
    app.gen_completions("todor", Shell::Zsh, &outdir);
    app.gen_completions("todor", Shell::Fish, &outdir);
    app.gen_completions("todor", Shell::PowerShell, &outdir);
}
