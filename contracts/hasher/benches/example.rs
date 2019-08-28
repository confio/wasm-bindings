use hasher::hash_num;

#[macro_use]
use ::criterion::Criterion;
use ::criterion::{black_box, criterion_group, criterion_main};

// run 100 sha256 hashes... and then sum
// this is high enough that the wasm call overhead should be minimal
fn bench_run(c: &mut Criterion) {
    c.bench_function("hash(100, 16, 43)", |b| {
        b.iter(|| hash_num(black_box(100), black_box(16), black_box(43)))
    });
}

criterion_group!(example, bench_run);
criterion_main!(example);
