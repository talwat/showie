use criterion::{criterion_group, criterion_main, Bencher, Criterion};
use image::io::Reader as ImageReader;
use showie::{render, Trim};

fn display_benchmark(c: &mut Criterion) {
    let img = ImageReader::open("tests/very_big.png")
        .unwrap()
        .decode()
        .unwrap()
        .trim();

    c.bench_function("display benchmark", move |b: &mut Bencher| {
        b.iter(|| {
            render(&img);
        })
    });
}

criterion_group!(benches, display_benchmark);
criterion_main!(benches);
