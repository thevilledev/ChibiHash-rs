# ChibiHash-rs

[<img alt="crates.io" src="https://img.shields.io/crates/v/chibihash.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/chibihash)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-chibihash-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/chibihash)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/thevilledev/chibihash-rs/test.yml?branch=main&style=for-the-badge" height="20">](https://github.com/thevilledev/chibihash-rs/actions?query=branch%3Amain)

Rust port of [N-R-K/ChibiHash](https://github.com/N-R-K/ChibiHash). See the article [ChibiHash: A small, fast 64-bit hash function](https://nrk.neocities.org/articles/chibihash) for more information.

See the original repository for more information, especially for when not to use ChibiHash.

All credit for the algorithm goes to [N-R-K](https://github.com/N-R-K).

## Features

- 64-bit hash function
- Deterministic
- Fast
- No dependencies
- Two alternative ways to use the algorithm:
  - Direct hashing via the `chibi_hash64()` function
  - Hasher implementation for use with Rust's `std::hash::Hasher` trait

## Example

```rust
use chibihash::{chibi_hash64, ChibiHasher};
use std::hash::Hasher;

fn main() {
    // Method 1: Direct hashing
    let hash = chibi_hash64(b"yellow world", 42).expect("Failed to hash");
    println!("Direct hash: {:016x}", hash);

    // Method 2: Using Hasher trait
    let mut hasher = ChibiHasher::new(42);
    hasher.write(b"yellow world");
    println!("Hasher trait: {:016x}", hasher.finish());
}
```

## Tests

Run `cargo test` to see the tests.

## Benchmarks

Run `cargo bench` to see the benchmarks. See `target/criterion/report/index.html` for the HTML report.

## License

MIT