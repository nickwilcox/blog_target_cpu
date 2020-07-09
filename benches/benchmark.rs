use blog_target_cpu::*;
use criterion::{criterion_group, criterion_main, Criterion};
use rand::distributions::Standard;
use rand::prelude::*;
use std::default::Default;

fn prepare_data(width: usize, height: usize) -> (Vec<u8>, Vec<RGBA32>, Vec<RGBA32>) {
    let samples = width * height;
    let palette = {
        let mut p = Vec::new();
        let mut i = 0u8;
        p.resize_with(256, || {
            let x = u32::from_le_bytes([i, i, i, 255]);
            i = i.wrapping_add(1);
            x
        });
        p
    };
    let rng = StdRng::seed_from_u64(0xdead_beef);
    let src: Vec<_> = rng.sample_iter(Standard).take(samples).collect();
    let dst = vec![0; samples];
    (src, palette, dst)
}

fn index_to_rgba_benchmarks(c: &mut Criterion) {
    let (width, height) = (256, 256);
    let (input, palette, mut output) = prepare_data(width, height);
    c.bench_function("indexed_to_rgba32", move |b| {
        b.iter(|| indexed_to_rgba32(&input, &palette, &mut output))
    });
    let (input, palette, mut output) = prepare_data(width, height);
    c.bench_function("indexed_to_rgba32 manual", move |b| {
        b.iter(|| unsafe { indexed_to_rgba32_avx2(&input, &palette, &mut output) })
    });
}

criterion_group!(benches, index_to_rgba_benchmarks);
criterion_main!(benches);
