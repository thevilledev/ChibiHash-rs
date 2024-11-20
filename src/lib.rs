//! ChibiHash: A small, fast 64-bit hash function implementation in Rust
//!
//! This crate provides a fast, non-cryptographic 64-bit hash function implementation
//! based on the [ChibiHash algorithm](https://github.com/N-R-K/ChibiHash).
//!
//! # Examples
//!
//! Basic usage:
//! ```rust
//! use chibihash::{chibi_hash64, ChibiHasher};
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

use std::hash::{Hash, Hasher};

pub fn chibi_hash64(key: &[u8], seed: u64) -> u64 {
    const P1: u64 = 0x2B7E151628AED2A5;
    const P2: u64 = 0x9E3793492EEDC3F7;
    const P3: u64 = 0x3243F6A8885A308D;

    let mut h = [P1, P2, P3, seed];
    let len = key.len();
    let mut k = key;

    // Process 32-byte chunks
    while k.len() >= 32 {
        for i in 0..4 {
            let lane = load_u64_le(&k[i * 8..]);
            h[i] ^= lane;
            h[i] = h[i].wrapping_mul(P1);
            h[(i + 1) & 3] ^= lane.rotate_left(40);
        }
        k = &k[32..];
    }

    // Add length mix
    h[0] = h[0].wrapping_add((len as u64).rotate_right(32));

    // Handle single byte if present
    if k.len() & 1 != 0 {
        h[0] ^= k[0] as u64;
        k = &k[1..];
    }
    h[0] = h[0].wrapping_mul(P2);
    h[0] ^= h[0] >> 31;

    // Process remaining 8-byte chunks
    let mut i = 1;
    while k.len() >= 8 {
        h[i] ^= load_u64_le(k);
        h[i] = h[i].wrapping_mul(P2);
        h[i] ^= h[i] >> 31;
        k = &k[8..];
        i += 1;
    }

    // Process remaining 2-byte chunks
    i = 0;
    while k.len() >= 2 {
        h[i] ^= u64::from(k[0]) | (u64::from(k[1]) << 8);
        h[i] = h[i].wrapping_mul(P3);
        h[i] ^= h[i] >> 31;
        k = &k[2..];
        i += 1;
    }

    // Final mixing
    let mut x = seed;
    x ^= h[0].wrapping_mul((h[2] >> 32) | 1);
    x ^= h[1].wrapping_mul((h[3] >> 32) | 1);
    x ^= h[2].wrapping_mul((h[0] >> 32) | 1);
    x ^= h[3].wrapping_mul((h[1] >> 32) | 1);

    // moremur mixing
    x ^= x >> 27;
    x = x.wrapping_mul(0x3C79AC492BA7B653);
    x ^= x >> 33;
    x = x.wrapping_mul(0x1C69B3F74AC4AE35);
    x ^= x >> 27;

    x
}

#[inline(always)]
fn load_u64_le(bytes: &[u8]) -> u64 {
    u64::from_le_bytes(bytes[..8].try_into().unwrap())
}

/// Configuration for the hash function
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_hashing() {
        let data = b"Hello, World!";
        let hash1 = chibi_hash64(data, 0);
        let hash2 = chibi_hash64(data, 0);
        assert_eq!(hash1, hash2, "Same input should produce same hash");

        let hash3 = chibi_hash64(data, 1);
        assert_ne!(
            hash1, hash3,
            "Different seeds should produce different hashes"
        );
    }

    #[test]
    // Tested against a Github comment from the original ChibiHash author
    // See https://github.com/N-R-K/ChibiHash/issues/1#issuecomment-2486086163
    fn test_real_world_examples() {
        assert_eq!(chibi_hash64(b"", 0), 0x9EA80F3B18E26CFB);
        assert_eq!(chibi_hash64(b"", 55555), 0x2EED9399FC4AC7E5);
        assert_eq!(chibi_hash64(b"hi", 0), 0xAF98F3924F5C80D6);
        assert_eq!(chibi_hash64(b"123", 0), 0x893A5CCA05B0A883);
        assert_eq!(chibi_hash64(b"abcdefgh", 0), 0x8F922660063E3E75);
        assert_eq!(chibi_hash64(b"Hello, world!", 0), 0x5AF920D8C0EBFE9F);
        assert_eq!(
            chibi_hash64(b"qwertyuiopasdfghjklzxcvbnm123456", 0),
            0x2EF296DB634F6551
        );
        assert_eq!(
            chibi_hash64(b"qwertyuiopasdfghjklzxcvbnm123456789", 0),
            0x0F56CF3735FFA943
        );
    }

    #[test]
    fn test_hasher_trait() {
        let mut hasher1 = ChibiHasher::new(0);
        let mut hasher2 = ChibiHasher::new(0);

        "Hello, World!".hash(&mut hasher1);
        "Hello, World!".hash(&mut hasher2);

        assert_eq!(
            hasher1.finish(),
            hasher2.finish(),
            "Same input should produce same hash"
        );

        let mut hasher3 = ChibiHasher::new(1);
        "Hello, World!".hash(&mut hasher3);
        assert_ne!(
            hasher1.finish(),
            hasher3.finish(),
            "Different seeds should produce different hashes"
        );
    }
}
