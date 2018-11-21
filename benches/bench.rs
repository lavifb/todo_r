#[macro_use]
extern crate criterion;
extern crate todo_r;

use std::path::Path;
use criterion::Criterion;
use todo_r::TodoR;
use todo_r::errors::eprint_error;

fn bench_jquery(c: &mut Criterion) {
    c.bench_function("jquery", |b| b.iter(|| {
		let todo_words = vec!["TODO", "FIXME"];
    	let mut todor = TodoR::with_todo_words(&todo_words);
    	todor.open_todos(Path::new("benches/inputs/jquery-3.3.1.js")).unwrap_or_else(|err| eprint_error(&err));
    }));
}

criterion_group!(benches, bench_jquery);
criterion_main!(benches);