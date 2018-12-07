// Benchmarking for todor

use criterion::{Criterion, criterion_group, criterion_main};
use std::path::Path;
use todo_r::TodoRBuilder;

fn bench_jquery(c: &mut Criterion) {
    c.bench_function("jquery", |b| b.iter(|| {
		let tags = vec!["TODO", "FIXME"];
		let mut builder = TodoRBuilder::new();
		builder.add_override_tags(tags);
    	let mut todor = builder.build().unwrap();
    	todor.open_todos(Path::new("benches/inputs/jquery-3.3.1.js")).unwrap();
    }));
}

criterion_group!(benches, bench_jquery);
criterion_main!(benches);