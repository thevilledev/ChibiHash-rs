//! ChibiHash: A small, fast 64-bit hash function implementation in Rust
//!
//! This crate provides a fast, non-cryptographic 64-bit hash function implementation
//! based on the [ChibiHash algorithm](https://github.com/N-R-K/ChibiHash).
//!
//! This is version `v2` of the algorithm. Notes from the original author:
//!
//! - Faster performance on short string (42 cycles/hash vs 34 cycles/hash).
//!   The tail end handling has been reworked entirely with some inspiration
//!   from wyhash's short input reading.
//! - Better seeding. v1 seed only affected 64 bits of the initial state.
//!   v2 seed affects the full 256 bits. This allows it to pass smhasher3's
//!   SeedBlockLen and SeedBlockOffset tests.
//! - Slightly better mixing in bulk handling.
//! - Passes all 252 tests in smhasher3 (commit 34093a3), v1 failed 3.
//!
//! # Examples
//!
//! Basic usage:
//! ```rust
//! use chibihash::v2::{chibi_hash64, ChibiHasher};
//! use std::hash::Hasher;
//!
//! // Direct hashing
//! let key = b"Hello, World!";
//! let seed = 1337;
//! let hash = chibi_hash64(key, seed);
//! println!("Hash of '{}' is: {:016x}", String::from_utf8_lossy(key), hash);
//!
//! // Using the Hasher trait
//! let mut hasher = ChibiHasher::new(seed);
//! hasher.write(key);
//! println!("{:016x}", hasher.finish());
//! ```

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use core::hash::{BuildHasher, Hash, Hasher};
#[cfg(feature = "std")]
use std::hash::{BuildHasher, Hash, Hasher};

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[cfg(not(feature = "std"))]
use core::convert::TryInto;
#[cfg(feature = "std")]
use std::convert::TryInto;

const K: u64 = 0x2B7E151628AED2A7; // digits of e

pub fn chibi_hash64(key: &[u8], seed: u64) -> u64 {
    let seed2 = seed
        .wrapping_sub(K)
        .rotate_left(15)
        .wrapping_add(seed.wrapping_sub(K).rotate_left(47));

    let mut h = [
        seed,
        seed.wrapping_add(K),
        seed2,
        seed2.wrapping_add(K.wrapping_mul(K) ^ K),
    ];

    let mut p = key;
    let mut l = key.len();

    // Process 32-byte chunks
    while l >= 32 {
        for i in 0..4 {
            let stripe = load_u64_le(&p[i * 8..]);
            h[i] = stripe.wrapping_add(h[i]).wrapping_mul(K);
            h[(i + 1) & 3] = h[(i + 1) & 3].wrapping_add(stripe.rotate_left(27));
        }
        p = &p[32..];
        l -= 32;
    }

    // Process 8-byte chunks
    while l >= 8 {
        h[0] ^= load_u32_le(&p[0..]);
        h[0] = h[0].wrapping_mul(K);
        h[1] ^= load_u32_le(&p[4..]);
        h[1] = h[1].wrapping_mul(K);
        p = &p[8..];
        l -= 8;
    }

    // Handle remaining bytes
    if l >= 4 {
        h[2] ^= load_u32_le(&p[0..]);
        h[3] ^= load_u32_le(&p[l - 4..]);
    } else if l > 0 {
        h[2] ^= u64::from(p[0]);
        h[3] ^= u64::from(p[l / 2]) | (u64::from(p[l - 1]) << 8);
    }

    h[0] = h[0].wrapping_add((h[2].wrapping_mul(K)).rotate_left(31) ^ (h[2] >> 31));
    h[1] = h[1].wrapping_add((h[3].wrapping_mul(K)).rotate_left(31) ^ (h[3] >> 31));
    h[0] = h[0].wrapping_mul(K);
    h[0] ^= h[0] >> 31;
    h[1] = h[1].wrapping_add(h[0]);

    let mut x = (key.len() as u64).wrapping_mul(K);
    x ^= x.rotate_left(29);
    x = x.wrapping_add(seed);
    x ^= h[1];

    x ^= x.rotate_left(15) ^ x.rotate_left(42);
    x = x.wrapping_mul(K);
    x ^= x.rotate_left(13) ^ x.rotate_left(31);

    x
}

#[inline(always)]
fn load_u64_le(bytes: &[u8]) -> u64 {
    u64::from_le_bytes(bytes[..8].try_into().unwrap())
}

/// Configuration for the hash function
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct ChibiHasher {
    seed: u64,
    buffer: Vec<u8>,
}

impl ChibiHasher {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            buffer: Vec::new(),
        }
    }

    pub fn hash(&self, input: &[u8]) -> u64 {
        chibi_hash64(input, self.seed)
    }
}

impl Hasher for ChibiHasher {
    fn finish(&self) -> u64 {
        // Hash the accumulated bytes with our chibi_hash64 function
        chibi_hash64(&self.buffer, self.seed)
    }

    fn write(&mut self, bytes: &[u8]) {
        // Append the new bytes to our buffer
        self.buffer.extend_from_slice(bytes);
    }
}

impl BuildHasher for ChibiHasher {
    type Hasher = ChibiHasher;

    fn build_hasher(&self) -> Self::Hasher {
        ChibiHasher::new(self.seed)
    }
}

#[inline(always)]
fn load_u32_le(bytes: &[u8]) -> u64 {
    u32::from_le_bytes(bytes[..4].try_into().unwrap()) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    // Keep only internal implementation tests here
    #[test]
    fn test_load_u64_le() {
        let bytes = [1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(load_u64_le(&bytes), 0x0807060504030201);
    }

    #[test]
    #[cfg(not(feature = "std"))]
    fn test_no_std() {
        let key = b"abcdefgh";
        let hash = chibi_hash64(key, 0);
        assert_eq!(hash, 0xA2E39BE0A0689B32);
    }

    #[test]
    fn test_known_hashes() {
        let test_cases = [
            ("", 55555, 0x58AEE94CA9FB5092),
            ("", 0, 0xD4F69E3ECCF128FC),
            ("hi", 0, 0x92C85CA994367DAC),
            ("123", 0, 0x788A224711FF6E25),
            ("abcdefgh", 0, 0xA2E39BE0A0689B32),
            ("Hello, world!", 0, 0xABF8EB3100B2FEC7),
            ("qwertyuiopasdfghjklzxcvbnm123456", 0, 0x90FC5DB7F56967FA),
            ("qwertyuiopasdfghjklzxcvbnm123456789", 0, 0x6DCDCE02882A4975),
        ];

        for (input, seed, expected) in test_cases {
            assert_eq!(chibi_hash64(input.as_bytes(), seed), expected);
        }
    }
}
