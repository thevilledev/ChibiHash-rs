//! ChibiHash: A small, fast 64-bit hash function implementation in Rust
//!
//! This crate provides a fast, non-cryptographic 64-bit hash function implementation
//! based on the [ChibiHash algorithm](https://github.com/N-R-K/ChibiHash).
//!
//! # Examples
//!
//! Basic usage:
//! ```rust
//! use chibihash::{chibi_hash64, ChibiHasher, StreamingChibiHasher};
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
//!
//! // Streaming hashing
//! let mut hasher1 = StreamingChibiHasher::new(0);
//! hasher1.update(b"Hello, ");
//! hasher1.update(b"World!");
//! println!("{:016x}", hasher1.finalize());
//! ```

use std::hash::{Hash, Hasher};

const P1: u64 = 0x2B7E151628AED2A5;
const P2: u64 = 0x9E3793492EEDC3F7;
const P3: u64 = 0x3243F6A8885A308D;

pub fn chibi_hash64(key: &[u8], seed: u64) -> u64 {
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

/// Streaming ChibiHasher that processes data incrementally
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StreamingChibiHasher {
    h: [u64; 4], // keep 8-byte aligned fields together
    total_len: u64,
    seed: u64,
    buf: [u8; 32], // larger arrays later
    buf_len: usize,
}

impl StreamingChibiHasher {
    #[inline(always)]
    pub const fn new(seed: u64) -> Self {
        Self {
            h: [P1, P2, P3, seed],
            buf: [0; 32],
            buf_len: 0,
            total_len: 0,
            seed,
        }
    }

    pub fn update(&mut self, input: &[u8]) {
        let mut p = input;
        let mut l = p.len();

        // If there's data in buf, try to fill it up
        if self.buf_len > 0 {
            while l > 0 && self.buf_len < 32 {
                self.buf[self.buf_len] = p[0];
                self.buf_len += 1;
                p = &p[1..];
                l -= 1;
            }

            // Flush if filled
            if self.buf_len == 32 {
                for i in 0..4 {
                    let lane = load_u64_le(&self.buf[i * 8..]);
                    self.h[i] ^= lane;
                    self.h[i] = self.h[i].wrapping_mul(P1);
                    self.h[(i + 1) & 3] ^= lane.rotate_left(40);
                }
                self.buf_len = 0;
            }
        }

        // Process stripes, no copy
        while l >= 32 {
            for i in 0..4 {
                let lane = load_u64_le(&p[i * 8..]);
                self.h[i] ^= lane;
                self.h[i] = self.h[i].wrapping_mul(P1);
                self.h[(i + 1) & 3] ^= lane.rotate_left(40);
            }
            p = &p[32..];
            l -= 32;
        }

        // Tail end of the input goes to the buffer
        while l > 0 {
            self.buf[self.buf_len] = p[0];
            self.buf_len += 1;
            p = &p[1..];
            l -= 1;
        }

        self.total_len += input.len() as u64;
    }

    pub fn finalize(&self) -> u64 {
        let mut h = self.h;
        let mut p = &self.buf[..self.buf_len];
        let mut l = self.buf_len;

        h[0] = h[0].wrapping_add(self.total_len.rotate_right(32));

        if l & 1 != 0 {
            h[0] ^= p[0] as u64;
            p = &p[1..];
            l -= 1;
        }
        h[0] = h[0].wrapping_mul(P2);
        h[0] ^= h[0] >> 31;

        let mut i = 1;
        while l >= 8 && i < 4 { // bounds check
            h[i] ^= load_u64_le(p);
            h[i] = h[i].wrapping_mul(P2);
            h[i] ^= h[i] >> 31;
            p = &p[8..];
            l -= 8;
            i += 1;
        }

        i = 0;
        while l >= 2 {
            h[i] ^= u64::from(p[0]) | (u64::from(p[1]) << 8);
            h[i] = h[i].wrapping_mul(P3);
            h[i] ^= h[i] >> 31;
            p = &p[2..];
            l -= 2;
            i += 1;
        }

        let mut x = self.seed;
        x ^= h[0].wrapping_mul((h[2] >> 32) | 1);
        x ^= h[1].wrapping_mul((h[3] >> 32) | 1);
        x ^= h[2].wrapping_mul((h[0] >> 32) | 1);
        x ^= h[3].wrapping_mul((h[1] >> 32) | 1);

        x ^= x >> 27;
        x = x.wrapping_mul(0x3C79AC492BA7B653);
        x ^= x >> 33;
        x = x.wrapping_mul(0x1C69B3F74AC4AE35);
        x ^= x >> 27;

        x
    }
}

impl Hasher for StreamingChibiHasher {
    fn finish(&self) -> u64 {
        self.finalize()
    }

    fn write(&mut self, bytes: &[u8]) {
        self.update(bytes);
    }
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
}