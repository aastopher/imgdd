use criterion::{criterion_group, criterion_main, Criterion, black_box};
extern crate imgdd;

use imgdd::dedupe::{collect_hashes, open_image, find_duplicates};
use imgdd::hashing::ImageHash;
use imgdd::normalize::proc as normalize;
use std::path::PathBuf;

/// Benchmark for opening an image file
fn open_image_bench(c: &mut Criterion) {
    let path = PathBuf::from("./imgs/test/apple_pie/21063.jpg");

    c.bench_function("open_image", |b| {
        b.iter(|| {
            let _ = open_image(black_box(&path)).expect("Failed to open image");
        });
    });
}

/// Benchmark for `dhash`
fn benchmark_dhash(c: &mut Criterion) {
    let img_path = PathBuf::from("./imgs/test/apple_pie/21063.jpg");

    // Unwrap the image and normalize it outside the benchmark iteration
    let image = open_image(&img_path).expect("Failed to open image");
    let normalized_image = normalize(&image, image::imageops::FilterType::Nearest)
        .expect("Failed to normalize image");

    c.bench_function("dhash", |b| {
        b.iter(|| {
            // Compute the dhash for the normalized image
            ImageHash::dhash(black_box(&normalized_image)).expect("Failed to compute dhash");
        });
    });
}

/// Benchmark for `normalize`
fn benchmark_normalize(c: &mut Criterion) {
    let img_path = PathBuf::from("./imgs/test/apple_pie/21063.jpg");
    let image = open_image(&img_path).expect("Failed to open image");

    c.bench_function("normalize", |b| {
        b.iter(|| {
            normalize(black_box(&image), black_box(image::imageops::FilterType::Nearest))
                .expect("Failed to normalize image");
        });
    });
}

// /// Benchmark for `collect_hashes`
// fn benchmark_collect_hashes(c: &mut Criterion) {
//     let dir_path = PathBuf::from("./imgs/test");

//     c.bench_function("collect_hashes", |b| {
//         b.iter(|| {
//             let _ = collect_hashes(
//                 black_box(&dir_path),
//                 black_box(image::imageops::FilterType::Nearest),
//                 black_box("dhash"),
//             )
//             .expect("Failed to collect hashes");
//         });
//     });
// }

// /// Benchmark for `find_duplicates`
// fn benchmark_find_duplicates(c: &mut Criterion) {
//     let dir_path = PathBuf::from("./imgs/test");
//     let hashes_with_paths = collect_hashes(
//         &dir_path,
//         image::imageops::FilterType::Nearest,
//         "dhash",
//     )
//     .expect("Failed to collect hashes");

//     c.bench_function("find_duplicates", |b| {
//         b.iter(|| {
//             let _ = find_duplicates(black_box(&hashes_with_paths), false)
//                 .expect("Failed to find duplicates");
//         });
//     });
// }

criterion_group!(
    benchmarks,
    open_image_bench,
    benchmark_dhash,
    benchmark_normalize
);
criterion_main!(benchmarks);
