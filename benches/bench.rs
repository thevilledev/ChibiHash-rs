use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use chibihash::chibi_hash64;

fn bench_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("input_sizes");

    // Benchmark different input sizes
    for size in [8, 16, 32, 64, 128, 256, 512, 1024].iter() {
        let input = vec![0u8; *size];
        group.bench_with_input(
            BenchmarkId::from_parameter(size), 
            &input,
            |b, input| {
                b.iter(|| chibi_hash64(black_box(input), black_box(0)))
            }
        );
    }

    group.finish();
}
pub fn bench_small_inputs(c: &mut Criterion) {
    let mut group = c.benchmark_group("small_inputs");

    let text = b"Hello, World!";
    group.bench_function("hello_world", |b| {
        b.iter(|| chibi_hash64(black_box(text), black_box(0)))
    });

    group.finish();
}

pub fn bench_seeds(c: &mut Criterion) {
    let mut group = c.benchmark_group("different_seeds");
    let input = b"Hello, World!";
    
    for seed in [0, 1, 42, u64::MAX].iter() {
        group.bench_with_input(
            BenchmarkId::new("seed", seed),
            seed,
            |b, &seed| {
                b.iter(|| chibi_hash64(black_box(input), black_box(seed)))
            }
        );
    }

    group.finish();
}

criterion_group!(benches, bench_sizes, bench_small_inputs, bench_seeds);
criterion_main!(benches);