use criterion::{criterion_group, criterion_main, Criterion, black_box};
use imgdd::dedupe;
use std::path::PathBuf;

fn collect_duplicates_benchmark(c: &mut Criterion) {
    let path = PathBuf::from("./imgs");
    let hamming_threshold = 12;

    c.bench_function("collect_duplicates", |b| {
        b.iter(|| {
            let _ = dedupe::collect_duplicates(black_box(&path), black_box(hamming_threshold))
                .expect("Failed to collect duplicates");
        });
    });

}

criterion_group!(benches, collect_duplicates_benchmark);
criterion_main!(benches);
