use criterion::{black_box, criterion_group, criterion_main, Criterion};
use imgdd::*;
use std::path::PathBuf;

fn benchmark_select_filter_type(c: &mut Criterion) {
    c.bench_function("select_filter_type", |b| {
        b.iter(|| {
            black_box(select_filter_type(Some("nearest")));
        });
    });
}

fn benchmark_select_algo(c: &mut Criterion) {
    c.bench_function("select_algo", |b| {
        b.iter(|| {
            black_box(select_algo(Some("dhash")));
        });
    });
}

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
    benchmark_select_filter_type,
    benchmark_select_algo,
    benchmark_hash,
    benchmark_dupes
);

criterion_main!(rust_interface_benchmarks);
