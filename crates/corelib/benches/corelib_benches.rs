use criterion::{criterion_group, criterion_main, Criterion, black_box};
extern crate corelib;

use corelib::dedupe::{open_image, collect_hashes, sort_hashes, find_duplicates};
use corelib::hashing::ImageHash;
use corelib::normalize::proc as normalize;
use std::path::PathBuf;

/// Benchmark for opening an image file
fn open_image_bench(c: &mut Criterion) {
    let path = PathBuf::from("../../imgs/test/single/file000898199107.jpg");

    c.bench_function("open_image", |b| {
        b.iter(|| {
            let _ = open_image(black_box(&path)).expect("Failed to open image");
        });
    });
}

/// Benchmark for `normalize`
fn benchmark_normalize(c: &mut Criterion) {
    let img_path = PathBuf::from("../../imgs/test/single/file000898199107.jpg");
    let image = open_image(&img_path).expect("Failed to open image");

    c.bench_function("normalize", |b| {
        b.iter(|| {
            normalize(black_box(&image), black_box(image::imageops::FilterType::Triangle))
                .expect("Failed to normalize image");
        });
    });
}

/// Benchmark for `dhash`
fn benchmark_dhash(c: &mut Criterion) {
    let img_path = PathBuf::from("../../imgs/test/single/file000898199107.jpg");

    // Unwrap the image and normalize it outside the benchmark iteration
    let image = open_image(&img_path).expect("Failed to open image");
    let normalized_image = normalize(&image, image::imageops::FilterType::Triangle)
        .expect("Failed to normalize image");

    c.bench_function("dhash", |b| {
        b.iter(|| {
            // Compute the dhash for the normalized image
            ImageHash::dhash(black_box(&normalized_image)).expect("Failed to compute dhash");
        });
    });
}

/// Benchmark for `collect_hashes`
fn benchmark_collect_hashes(c: &mut Criterion) {
    let dir_path = PathBuf::from("../../imgs/test/single");

    c.bench_function("collect_hashes", |b| {
        b.iter(|| {
            let _ = collect_hashes(
                black_box(&dir_path),
                black_box(image::imageops::FilterType::Triangle),
                black_box("dhash"),
            )
            .expect("Failed to collect hashes");
        });
    });
}

/// Benchmark for `sort_hashes`
fn benchmark_sort_hashes(c: &mut Criterion) {
    let dir_path = PathBuf::from("../../imgs/test");
    let mut hash_paths = collect_hashes(
        &dir_path,
        image::imageops::FilterType::Triangle,
        "dhash",
    )
    .expect("Failed to collect hashes");

    c.bench_function("sort_hashes", |b| {
        b.iter(|| {
            sort_hashes(black_box(&mut hash_paths));
        });
    });
}

/// Benchmark for `find_duplicates`
fn benchmark_find_duplicates(c: &mut Criterion) {
    let dir_path = PathBuf::from("../../imgs/test");
    let mut hash_paths = collect_hashes(
        &dir_path,
        image::imageops::FilterType::Triangle,
        "dhash",
    )
    .expect("Failed to collect hashes");
    // let mut sorted_hashes = hash_paths.clone();
    sort_hashes(&mut hash_paths);

    c.bench_function("find_duplicates", |b| {
        b.iter(|| {
            let _ = find_duplicates(black_box(&hash_paths), false)
                .expect("Failed to find duplicates");
        });
    });
}

criterion_group! {
    name = group1;
    config = Criterion::default().sample_size(60);
    targets = open_image_bench, benchmark_normalize
}

criterion_group! {
    name = group2;
    config = Criterion::default().sample_size(40);
    targets = benchmark_collect_hashes, benchmark_sort_hashes
}

criterion_group!(
    group3,
    benchmark_dhash,
    benchmark_find_duplicates
);

criterion_main!(group1, group2, group3);
