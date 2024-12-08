    use criterion::{criterion_group, criterion_main, Criterion};
    use imgdd::image_hash::ImageHash;
    use image::open;
    
    fn dhash_benchmark(c: &mut Criterion) {
        // Load a sample image for benchmarking
        let img_path = "./imgs/ukbench00120_hflip.jpg";
        let image = open(img_path).expect("Failed to open image");
    
        c.bench_function("dhash", |b| {
            b.iter(|| {
                ImageHash::dhash(&image, 8).expect("Failed to compute dhash");
            })
        });
    }
    
    criterion_group!(benches, dhash_benchmark);
    criterion_main!(benches);