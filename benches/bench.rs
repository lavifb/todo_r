#[macro_use]
extern crate criterion;
extern crate todo_r;

use criterion::Criterion;
use todo_r::TodoR;
use todo_r::errors::eprint_error;
// use assert_cmd::prelude::*;
// use std::process::Command;

// fn todor_cmd(input: &str) -> Command {
// 	let mut cmd = Command::cargo_bin("todor").unwrap();
// 	cmd.current_dir("benches/inputs");
// 	cmd.arg(input);
// 	cmd
// }

fn bench_jquery(c: &mut Criterion) {
    c.bench_function("jquery", |b| b.iter(|| {
		let todo_words = ["TODO", "FIXME"];
    	let mut todor = TodoR::new(&todo_words);
    	todor.open_todos("benches/inputs/jquery-3.3.1.js").unwrap_or_else(|err| eprint_error(&err));
    }));
}

criterion_group!(benches, bench_jquery);
criterion_main!(benches);