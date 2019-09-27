use wasm_bindings::{run, setup_clif, setup_metered, setup_singlepass};

#[cfg(feature = "llvm")]
use wasm_bindings::setup_llvm;

use wasmer_middleware_common::metering;

use ::criterion::Criterion;
use ::criterion::{black_box, criterion_group, criterion_main};

fn bench_run_singlepass(c: &mut Criterion) {
    let instance = setup_singlepass().unwrap();
    c.bench_function("wasm_hash_singlepass(100, 16, 43)", |b| {
        b.iter(|| run(&instance, black_box(100), black_box(16), black_box(43)))
    });
}

fn bench_setup_singlepass(c: &mut Criterion) {
    c.bench_function("wasm_setup_singlepass()", |b| {
        b.iter(|| { let _i = setup_singlepass().unwrap(); })
    });
}

fn bench_run_clif(c: &mut Criterion) {
    let instance = setup_clif().unwrap();
    c.bench_function("wasm_hash_clif(100, 16, 43)", |b| {
        b.iter(|| run(&instance, black_box(100), black_box(16), black_box(43)))
    });
}


fn bench_setup_clif(c: &mut Criterion) {
    c.bench_function("wasm_setup_clif()", |b| {
        b.iter(|| { let _i = setup_clif().unwrap(); })
    });
}


#[cfg(feature = "llvm")]
fn bench_run_llvm(c: &mut Criterion) {
    let instance = setup_llvm().unwrap();
    c.bench_function("wasm_hash_llvm(100, 16, 43)", |b| {
        b.iter(|| run(&instance, black_box(100), black_box(16), black_box(43)))
    });
}

#[cfg(feature = "llvm")]
fn bench_setup_llvm(c: &mut Criterion) {
    c.bench_function("wasm_setup_llvm()", |b| {
        b.iter(|| { let _i = setup_llvm().unwrap(); })
    });
}


fn bench_run_metered(c: &mut Criterion) {
    let mut instance = setup_metered(100000000).unwrap();
    c.bench_function("wasm_hash_metered(100, 16, 43)", |b| {
        b.iter(|| {
            run(&instance, black_box(100), black_box(16), black_box(43));
            metering::set_points_used(&mut instance, 100);
        })
    });
}

fn setup_config() -> Criterion {
    let base = Criterion::default();
    base.sample_size(10)
}

#[cfg(feature = "llvm")]
criterion_group!(
    hashing,
    bench_run_singlepass,
    bench_run_clif,
    bench_run_llvm,
    bench_run_metered
);

#[cfg(not(feature = "llvm"))]
criterion_group!(
    hashing,
    bench_run_singlepass,
    bench_run_clif,
    bench_run_metered
);

#[cfg(feature = "llvm")]
criterion_group!(
    name = setup;
    config = setup_config();
    targets = bench_setup_singlepass, bench_setup_clif, bench_setup_llvm
);

#[cfg(not(feature = "llvm"))]
criterion_group!(
    name = setup;
    config = setup_config();
    targets = bench_setup_singlepass, bench_setup_clif
);


criterion_main!(hashing, setup);
