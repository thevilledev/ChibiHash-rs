use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use std::ffi::c_void;

// FFI declaration for the C implementation
extern "C" {
    fn chibihash64(key: *const c_void, len: isize, seed: u64) -> u64;
}

fn bench_cross_language(c: &mut Criterion) {
    let mut group = c.benchmark_group("rust_vs_c");
    
    // Test different input patterns
    let test_cases = vec![
        ("zeros", vec![0u8; 1024]),
        ("ones", vec![1u8; 1024]),
        ("alternating", (0..1024).map(|i| (i % 2) as u8).collect()),
        ("incremental", (0..1024).map(|i| (i % 256) as u8).collect()),
        ("random", {
            let mut v = vec![0u8; 1024];
            for i in 0..1024 {
                v[i] = ((i * 7 + 13) % 256) as u8;
            }
            v
        }),
    ];

    // Test different sizes to see where performance characteristics differ
    let sizes = [8, 16, 32, 64, 128, 256, 512, 1024];

    for (pattern_name, pattern) in test_cases {
        for size in sizes.iter() {
            let input = &pattern[..*size];
            
            // Benchmark Rust implementation
            group.bench_with_input(
                BenchmarkId::new(format!("rust_{}", pattern_name), size),
                &input,
                |b, input| {
                    b.iter(|| {
                        black_box(chibihash::chibi_hash64(
                            black_box(input),
                            black_box(0)
                        ))
                    })
                },
            );

            // Benchmark C implementation
            group.bench_with_input(
                BenchmarkId::new(format!("c_{}", pattern_name), size),
                &input,
                |b, input| {
                    b.iter(|| unsafe {
                        black_box(chibihash64(
                            black_box(input.as_ptr() as *const c_void),
                            black_box(input.len() as isize),
                            black_box(0)
                        ))
                    })
                },
            );
        }
    }

    // Test alignment sensitivity
    let aligned_data = vec![0u8; 1024];
    let mut unaligned_data = vec![0u8; 1025];
    unaligned_data.remove(0); // Create unaligned slice

    for size in [32, 64, 128, 256].iter() {
        group.bench_with_input(
            BenchmarkId::new("rust_aligned", size),
            &aligned_data[..*size],
            |b, input| {
                b.iter(|| black_box(chibihash::chibi_hash64(black_box(input), 0)))
            },
        );

        group.bench_with_input(
            BenchmarkId::new("rust_unaligned", size),
            &unaligned_data[..*size],
            |b, input| {
                b.iter(|| black_box(chibihash::chibi_hash64(black_box(input), 0)))
            },
        );

        // Same for C implementation
        group.bench_with_input(
            BenchmarkId::new("c_aligned", size),
            &aligned_data[..*size],
            |b, input| unsafe {
                b.iter(|| {
                    black_box(chibihash64(
                        black_box(input.as_ptr() as *const c_void),
                        black_box(input.len() as isize),
                        0
                    ))
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("c_unaligned", size),
            &unaligned_data[..*size],
            |b, input| unsafe {
                b.iter(|| {
                    black_box(chibihash64(
                        black_box(input.as_ptr() as *const c_void),
                        black_box(input.len() as isize),
                        0
                    ))
                })
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_cross_language);
criterion_main!(benches);