use hasher::{hash_num};

#[macro_use]
use ::criterion::Criterion;
use ::criterion::{black_box, criterion_group, criterion_main};


fn bench_run(c: &mut Criterion) {
    c.bench_function("hash(16, 43)", |b| b.iter(|| hash_num(black_box(16), black_box( 43))));
}

criterion_group!(example, bench_run);
criterion_main!(example);
