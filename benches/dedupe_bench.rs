use criterion::{criterion_group, criterion_main, Criterion, black_box};
use imgdd::dedupe::{collect_hashes, find_duplicates, find_duplicates_with_threshold};
use imgdd::image_hash::ImageHash;
use image::open;
use std::path::PathBuf;

fn reduce_samples() -> Criterion {
    Criterion::default().sample_size(10)
}

fn benchmark_collect_hashes(c: &mut Criterion) {
    let path = PathBuf::from("./imgs/bench");

    c.bench_function("collect_hashes", |b| {
        b.iter(|| {
            let _ = collect_hashes(black_box(&path)).expect("Failed to collect hashes");
        });
    });
}

fn benchmark_find_duplicates(c: &mut Criterion) {
    let path = PathBuf::from("./imgs/bench");
    let hashes_with_paths = collect_hashes(&path).expect("Failed to collect hashes");

    c.bench_function("find_duplicates", |b| {
        b.iter(|| {
            let _ = find_duplicates(black_box(&hashes_with_paths));
        });
    });

    let threshold = 12;

    c.bench_function("find_duplicates_with_threshold", |b| {
        b.iter(|| {
            let _ = find_duplicates_with_threshold(black_box(&hashes_with_paths), threshold);
        });
    });
}

fn benchmark_dhash(c: &mut Criterion) {
    let img_path = "./imgs/test/apple_pie/21063.jpg";
    let image = open(img_path).expect("Failed to open image");

    c.bench_function("dhash", |b| {
        b.iter(|| {
            ImageHash::dhash(&image).expect("Failed to compute dhash");
        })
    });
}

fn benchmark_hamming_distance(c: &mut Criterion) {
    let img_path1 = "./imgs/test/baklava/21435.jpg"; 
    let img_path2 = "./imgs/test/baklava/25496.jpg"; 
    let image1 = open(img_path1).expect("Failed to open ./imgs/test/baklava/21435.jpg");
    let image2 = open(img_path2).expect("Failed to open ./imgs/test/baklava/25496.jpg");

    let hash1 = ImageHash::dhash(&image1).expect("Failed to compute dhash for ./imgs/test/baklava/21435.jpg");
    let hash2 = ImageHash::dhash(&image2).expect("Failed to compute dhash for ./imgs/test/baklava/25496.jpg");

    c.bench_function("hamming_distance", |b| {
        b.iter(|| {
            hash1.hamming_distance(&hash2);
        })
    });
}

criterion_group! {
    name = reduced_benches;
    config = reduce_samples();
    targets = benchmark_collect_hashes
}

criterion_group!(
    other_benches,
    benchmark_find_duplicates,
    benchmark_dhash,
    benchmark_hamming_distance
);

criterion_main!(reduced_benches, other_benches);
