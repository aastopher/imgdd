use criterion::{criterion_group, criterion_main, Criterion, black_box};

use imgddcore::dedupe::{open_image, collect_hashes, sort_hashes, find_duplicates};
use imgddcore::hashing::ImageHash;
use imgddcore::normalize::proc as normalize;
use std::path::PathBuf;

// WARNING! 
// dd.hash function benchmarks will be inaccurate because; this metric relies heavily on system calls.
// Since they cannot be consistently instrumented, those calls are not included in the final measure.
// To resolve this we must use hosted codspeed macro runners which require a pro plan. 
// For now I will just leave this warning here.

fn open_image_bench(c: &mut Criterion) {
    let path = PathBuf::from("../../imgs/test/single/file000898199107.jpg");

    c.bench_function("open_image", |b| {
        b.iter(|| {
            let _ = open_image(black_box(&path)).expect("Failed to open image");
        });
    });
}

fn benchmark_normalize(c: &mut Criterion) {
    let img_path = PathBuf::from("../../imgs/test/single/file000898199107.jpg");
    let image = open_image(&img_path).expect("Failed to open image");

    c.bench_function("normalize", |b| {
        b.iter(|| {
            normalize(black_box(&image), black_box(image::imageops::FilterType::Triangle), black_box(9), black_box(8))
                .expect("Failed to normalize image");
        });
    });
}

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

fn benchmark_find_duplicates(c: &mut Criterion) {
    let dir_path = PathBuf::from("../../imgs/test");
    let mut hash_paths = collect_hashes(
        &dir_path,
        image::imageops::FilterType::Triangle,
        "dhash",
    )
    .expect("Failed to collect hashes");
    sort_hashes(&mut hash_paths);

    c.bench_function("find_duplicates", |b| {
        b.iter(|| {
            let _ = find_duplicates(black_box(&hash_paths), false)
                .expect("Failed to find duplicates");
        });
    });
}

// Hash algorithms
fn benchmark_ahash(c: &mut Criterion) {
    let img_path = PathBuf::from("../../imgs/test/single/file000898199107.jpg");

    // Unwrap the image and normalize it outside the benchmark iteration
    let image = open_image(&img_path).expect("Failed to open image");
    let normalized_image = normalize(&image, image::imageops::FilterType::Triangle, 8, 8)
        .expect("Failed to normalize image");

    c.bench_function("ahash", |b| {
        b.iter(|| {
            // Compute ahash for the normalized image
            ImageHash::ahash(black_box(&normalized_image)).expect("Failed to compute ahash");
        });
    });
}

fn benchmark_mhash(c: &mut Criterion) {
    let img_path = PathBuf::from("../../imgs/test/single/file000898199107.jpg");

    // Unwrap the image and normalize it outside the benchmark iteration
    let image = open_image(&img_path).expect("Failed to open image");
    let normalized_image = normalize(&image, image::imageops::FilterType::Triangle, 8, 8)
        .expect("Failed to normalize image");

    c.bench_function("mhash", |b| {
        b.iter(|| {
            // Compute mhash for the normalized image
            ImageHash::mhash(black_box(&normalized_image)).expect("Failed to compute mhash");
        });
    });
}

fn benchmark_dhash(c: &mut Criterion) {
    let img_path = PathBuf::from("../../imgs/test/single/file000898199107.jpg");

    // Unwrap the image and normalize it outside the benchmark iteration
    let image = open_image(&img_path).expect("Failed to open image");
    let normalized_image = normalize(&image, image::imageops::FilterType::Triangle, 9, 8)
        .expect("Failed to normalize image");

    c.bench_function("dhash", |b| {
        b.iter(|| {
            // Compute dhash for the normalized image
            ImageHash::dhash(black_box(&normalized_image)).expect("Failed to compute dhash");
        });
    });
}

fn benchmark_phash(c: &mut Criterion) {
    let img_path = PathBuf::from("../../imgs/test/single/file000898199107.jpg");

    // Unwrap the image and normalize it outside the benchmark iteration
    let image = open_image(&img_path).expect("Failed to open image");
    let normalized_image = normalize(&image, image::imageops::FilterType::Triangle, 32, 32)
        .expect("Failed to normalize image");

    c.bench_function("phash", |b| {
        b.iter(|| {
            // Compute pHash for the normalized image
            ImageHash::phash(black_box(&normalized_image)).expect("Failed to compute phash");
        });
    });
}

fn benchmark_whash(c: &mut Criterion) {
    let img_path = PathBuf::from("../../imgs/test/single/file000898199107.jpg");

    // Unwrap the image and normalize it outside the benchmark iteration
    let image = open_image(&img_path).expect("Failed to open image");
    let normalized_image = normalize(&image, image::imageops::FilterType::Triangle, 8, 8)
        .expect("Failed to normalize image");

    c.bench_function("whash", |b| {
        b.iter(|| {
            // Compute wHash for the normalized image
            ImageHash::whash(black_box(&normalized_image)).expect("Failed to compute whash");
        });
    });
}


criterion_group! {
    name = group1;
    config = Criterion::default().sample_size(40);
    targets = open_image_bench, benchmark_normalize
}

criterion_group! {
    name = group2;
    config = Criterion::default().sample_size(30);
    targets = benchmark_collect_hashes, benchmark_sort_hashes
}

criterion_group!(
    group3,
    benchmark_ahash,
    benchmark_mhash,
    benchmark_dhash,
    benchmark_phash,
    benchmark_whash,
    benchmark_find_duplicates
);

criterion_main!(group1, group2, group3);
