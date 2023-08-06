use criterion::{criterion_group, criterion_main, Bencher, Criterion};
use image::io::Reader as ImageReader;
use showie::{to_ascii, trim};

fn display_benchmark(c: &mut Criterion) {
    let img = ImageReader::open("tests/very_big.png")
        .unwrap()
        .decode()
        .unwrap();

    c.bench_function("display benchmark", |b: &mut Bencher| {
        b.iter(|| {
            let trimmed = trim(&img);

            to_ascii(&trimmed);
        })
    });
} 

criterion_group!(benches, display_benchmark);
criterion_main!(benches);
