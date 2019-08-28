// #[cfg(test)]
use ::criterion::Criterion;
use ::criterion::{black_box, criterion_group, criterion_main};

use crate::{setup, run};

fn bench_run(c: &mut Criterion) {
    let instance = setup().unwrap();
    c.bench_function("hash(16, 43)", |b| b.iter(|| run(&instance, black_box(16), black_box( 43))));
}

criterion_group!(example, bench_run);
criterion_main!(example);
