use wasm_bindings::{setup, run};

#[macro_use]
use ::criterion::Criterion;
use ::criterion::{black_box, criterion_group, criterion_main};


fn bench_run(c: &mut Criterion) {
    let instance = setup().unwrap();
    c.bench_function("wasm_hash(100, 16, 43)", |b| b.iter(|| run(&instance, black_box(100), black_box(16), black_box( 43))));
}

criterion_group!(example, bench_run);
criterion_main!(example);
