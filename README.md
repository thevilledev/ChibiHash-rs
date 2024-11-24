# ChibiHash-rs

[<img alt="crates.io" src="https://img.shields.io/crates/v/chibihash.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/chibihash)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-chibihash-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/chibihash)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/thevilledev/chibihash-rs/test.yml?branch=main&style=for-the-badge" height="20">](https://github.com/thevilledev/chibihash-rs/actions?query=branch%3Amain)

Rust port of [N-R-K/ChibiHash](https://github.com/N-R-K/ChibiHash). See the article [ChibiHash: A small, fast 64-bit hash function](https://nrk.neocities.org/articles/chibihash) for more information.

All credit for the algorithm goes to [N-R-K](https://github.com/N-R-K).

## Features

- 64-bit hash function
- Deterministic
- Fast
- No dependencies
- Three ways to use ChibiHash:
  1. **Direct Hashing**: One-shot hashing using `chibi_hash64()`
  2. **Simple Hasher**: Basic implementation using `ChibiHasher` (implements `std::hash::Hasher`)
  3. **Streaming Hasher**: Memory-efficient streaming with `StreamingChibiHasher` (implements `std::hash::Hasher`)
## Example

```rust
use chibihash::{chibi_hash64, ChibiHasher, StreamingChibiHasher};
use std::hash::Hasher;

fn main() {
    // Method 1: Direct hashing
    let hash = chibi_hash64(b"yellow world", 42);
    println!("Direct hash: {:016x}", hash);

    // Method 2: Using Hasher trait
    let mut hasher = ChibiHasher::new(42);
    hasher.write(b"yellow world");
    println!("Hasher trait: {:016x}", hasher.finish());

    // Method 3: Streaming hashing
    let mut hasher = StreamingChibiHasher::new(0);
    hasher.update(b"yellow ");
    hasher.update(b"world");
    println!("Streaming: {:016x}", hasher.finalize());
}
```

## Tests

Run `cargo test` to see the tests.

## Benchmarks

Run `cargo bench` to see the benchmarks. See `target/criterion/report/index.html` for the HTML report.

The repository also contains a benchmark comparing the Rust implementation to the C implementation. Run `cargo bench --features ffi` to see the benchmark. The C version can be found from the `csrc` directory.

Based on limited testing, the pure Rust implementation is faster than the C version when run through the FFI interface. Possibly due to the overhead of the FFI interface itself.

## When not to use ChibiHash

Copy-paste from the original repository. Same applies here.

>Here are some reasons to avoid using this:
>
>* For cryptographic purposes.
>* For protecting against [collision attacks](https://en.wikipedia.org/wiki/Collision_attack) (SipHash is the recommended one for this purpose).
>* When you need very strong probability against collisions: ChibiHash does very
>  minimal amount of mixing compared to other hashes (e.g xxhash64). And so
>  chances of collision should in theory be higher.

## License

MIT. The original C version is under the Unlicense.