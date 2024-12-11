use criterion::{criterion_group, criterion_main, Criterion};
use imgdd::image_hash::ImageHash;
use image::{ open };

fn dhash_benchmark(c: &mut Criterion) {
    let img_path = "./imgs/ukbench00120_hflip.jpg";
    let image = open(img_path).expect("Failed to open image");

    c.bench_function("dhash", |b| {
        b.iter(|| {
            ImageHash::dhash(&image, 8).expect("Failed to compute dhash");
        })
    });
}

fn hamming_distance_benchmark(c: &mut Criterion) {
    let img_path1 = "./imgs/ukbench00120_hflip.jpg";
    let img_path2 = "./imgs/ukbench00120.jpg";
    let image1 = open(img_path1).expect("Failed to open image 1");
    let image2 = open(img_path2).expect("Failed to open image 2");

    let hash1 = ImageHash::dhash(&image1, 8).expect("Failed to compute dhash for image 1");
    let hash2 = ImageHash::dhash(&image2, 8).expect("Failed to compute dhash for image 2");

    c.bench_function("hamming_distance", |b| {
        b.iter(|| {
            hash1.hamming_distance(&hash2);
        })
    });
}

criterion_group!(benches, dhash_benchmark, hamming_distance_benchmark);
criterion_main!(benches);
