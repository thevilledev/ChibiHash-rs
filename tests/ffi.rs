#![cfg(feature = "ffi")]

use std::ffi::c_void;

extern "C" {
    fn chibihash64_v1(key: *const c_void, len: isize, seed: u64) -> u64;
}

#[test]
fn c_v1_matches_rust_v1() {
    const SEEDS: [u64; 4] = [0, 1, 55_555, u64::MAX];

    for len in 0..=256 {
        let input: Vec<u8> = (0..len)
            .map(|index| ((index * 37 + len * 11) & 0xff) as u8)
            .collect();

        for seed in SEEDS {
            let rust_hash = chibihash::v1::chibi_hash64(&input, seed);
            let c_hash =
                unsafe { chibihash64_v1(input.as_ptr().cast(), input.len() as isize, seed) };

            assert_eq!(
                c_hash, rust_hash,
                "C and Rust v1 hashes differ for length {len} and seed {seed}"
            );
        }
    }
}
