// Benchmark the `v2` version of the algorithm

use chibihash::v2::{chibi_hash64, StreamingChibiHasher};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

fn bench_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("v2_input_sizes");

    // Benchmark different input sizes
    for size in [8, 16, 32, 64, 128, 256, 512, 1024].iter() {
        let input = vec![0u8; *size];
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, input| {
            b.iter(|| chibi_hash64(black_box(input), black_box(0)))
        });
    }

    group.finish();
}

pub fn bench_small_inputs(c: &mut Criterion) {
    let mut group = c.benchmark_group("v2_small_inputs");

    let text = b"Hello, World!";
    group.bench_function("hello_world", |b| {
        b.iter(|| chibi_hash64(black_box(text), black_box(0)))
    });

    group.finish();
}

pub fn bench_seeds(c: &mut Criterion) {
    let mut group = c.benchmark_group("v2_different_seeds");
    let input = b"Hello, World!";

    for seed in [0, 1, 42, u64::MAX].iter() {
        group.bench_with_input(BenchmarkId::new("seed", seed), seed, |b, &seed| {
            b.iter(|| chibi_hash64(black_box(input), black_box(seed)))
        });
    }

    group.finish();
}

pub fn bench_streaming(c: &mut Criterion) {
    let mut group = c.benchmark_group("v2_streaming");

    // Compare single-shot vs streaming for different sizes
    for size in [8, 16, 32, 64, 128, 256, 512, 1024].iter() {
        let input = vec![0u8; *size];

        // Single-shot benchmark
        group.bench_with_input(BenchmarkId::new("single_shot", size), &input, |b, input| {
            b.iter(|| chibi_hash64(black_box(input), black_box(0)))
        });

        // Streaming benchmark - single update
        group.bench_with_input(
            BenchmarkId::new("streaming_single", size),
            &input,
            |b, input| {
                b.iter(|| {
                    let mut hasher = StreamingChibiHasher::new(0);
                    hasher.update(black_box(input));
                    hasher.finalize()
                })
            },
        );

        // Streaming benchmark - split updates
        if *size >= 16 {
            group.bench_with_input(
                BenchmarkId::new("streaming_split", size),
                &input,
                |b, input| {
                    let mid = input.len() / 2;
                    let (first, second) = input.split_at(mid);
                    b.iter(|| {
                        let mut hasher = StreamingChibiHasher::new(0);
                        hasher.update(black_box(first));
                        hasher.update(black_box(second));
                        hasher.finalize()
                    })
                },
            );
        }
    }

    group.finish();
}

pub fn bench_streaming_small_chunks(c: &mut Criterion) {
    let mut group = c.benchmark_group("v2_streaming_chunks");

    // Test different chunk sizes for a 1KB input
    let input = vec![0u8; 1024];

    for chunk_size in [1, 4, 8, 16, 32, 64, 128].iter() {
        group.bench_with_input(
            BenchmarkId::new("chunk_size", chunk_size),
            chunk_size,
            |b, &chunk_size| {
                b.iter(|| {
                    let mut hasher = StreamingChibiHasher::new(0);
                    for chunk in input.chunks(chunk_size) {
                        hasher.update(black_box(chunk));
                    }
                    hasher.finalize()
                })
            },
        );
    }

    group.finish();
}

pub fn bench_streaming_realistic(c: &mut Criterion) {
    let mut group = c.benchmark_group("v2_streaming_realistic");

    // Simulate hashing a file in chunks
    let large_input = vec![0u8; 1024 * 1024]; // 1MB
    group.bench_function("file_chunks_32k", |b| {
        b.iter(|| {
            let mut hasher = StreamingChibiHasher::new(0);
            for chunk in large_input.chunks(32 * 1024) {
                // 32KB chunks
                hasher.update(black_box(chunk));
            }
            hasher.finalize()
        })
    });

    // Simulate incremental string building
    let parts = ["Hello", ", ", "World", "!"];
    group.bench_function("incremental_string", |b| {
        b.iter(|| {
            let mut hasher = StreamingChibiHasher::new(0);
            for part in parts.iter() {
                hasher.update(black_box(part.as_bytes()));
            }
            hasher.finalize()
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_sizes,
    bench_small_inputs,
    bench_seeds,
    bench_streaming,
    bench_streaming_small_chunks,
    bench_streaming_realistic
);

criterion_main!(benches);
