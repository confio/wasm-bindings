use wasm_bindings::{setup_singlepass, setup_clif, run, setup_metered};

use wasmer_middleware_common::metering;

use ::criterion::Criterion;
use ::criterion::{black_box, criterion_group, criterion_main};


fn bench_run_singlepass(c: &mut Criterion) {
    let instance = setup_singlepass().unwrap();
    c.bench_function("wasm_hash_singlepass(100, 16, 43)", |b| b.iter(|| run(&instance, black_box(100), black_box(16), black_box( 43))));
}

fn bench_run_clif(c: &mut Criterion) {
    let instance = setup_clif().unwrap();
    c.bench_function("wasm_hash_clif(100, 16, 43)", |b| b.iter(|| run(&instance, black_box(100), black_box(16), black_box( 43))));
}

fn bench_run_metered(c: &mut Criterion) {
    let mut instance = setup_metered(100000000).unwrap();
    c.bench_function("wasm_hash_metered(100, 16, 43)", |b| b.iter(|| {
        run(&instance, black_box(100), black_box(16), black_box( 43));
        metering::set_points_used(&mut instance, 100);
    }));
}


criterion_group!(example, bench_run_singlepass, bench_run_clif, bench_run_metered);
criterion_main!(example);
