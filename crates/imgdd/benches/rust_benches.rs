use criterion::{black_box, criterion_group, criterion_main, Criterion};
use imgdd::*;
use std::path::PathBuf;

fn benchmark_hash(c: &mut Criterion) {
    let dir_path = PathBuf::from("../../imgs/test/single");

    c.bench_function("hash_function", |b| {
        b.iter(|| {
            let result = hash(
                black_box(dir_path.clone()),
                Some("nearest"),
                Some("dhash"),
                Some(false),
            );
            let _ = black_box(result).is_ok(); // Ignore the result
        });
    });
}

fn benchmark_dupes(c: &mut Criterion) {
    let dir_path = PathBuf::from("../../imgs/test");

    c.bench_function("dupes_function", |b| {
        b.iter(|| {
            let result = dupes(
                black_box(dir_path.clone()),
                Some("nearest"),
                Some("dhash"),
                false,
            );
            let _ = black_box(result).is_ok(); // Ignore the result
        });
    });
}

criterion_group!(
    rust_interface_benchmarks,
    benchmark_hash,
    benchmark_dupes
);

criterion_main!(rust_interface_benchmarks);
