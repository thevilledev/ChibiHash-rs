# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [v0.3.1] - 2024-11-26

- Fixed `hashbrown` version

## [v0.3.0] - 2024-11-26

- Added `no-std` support
- Added `hashbrown` as a dependency for `no-std` compatibility
- Added `no-std` Crate category
- Fixed tests for well-known inputs

## [v0.2.2] - 2024-11-25

- Added `BuildHasher` implementation, supporting `HashMap` and `HashSet`

## [v0.2.1] - 2024-11-24

- Added benchmarks and dependencies for cross-language hashing, behind the `ffi` feature

## [v0.2.0] - 2024-11-20

- Removed custom error type `ChibiHashError`
- Added support for streaming hashing through `StreamingChibiHasher`

## [v0.1.2] - 2024-11-19

- Bit rotation the Rust way
- README improvements
- Added CI

## [v0.1.1] - 2024-11-19

- Makefile changes
- Linting and formatting
- Fixed `crates.io` categories

## [v0.1.0] - 2024-11-19

- Initial release
- Added benchmarks
- Released a crate

[Unreleased]: https://github.com/thevilledev/ChibiHash-rs/compare/v0.3.1...HEAD
[v0.3.1]: https://github.com/thevilledev/ChibiHash-rs/compare/v0.3.0...v0.3.1
[v0.3.0]: https://github.com/thevilledev/ChibiHash-rs/compare/v0.2.2...v0.3.0
[v0.2.2]: https://github.com/thevilledev/ChibiHash-rs/compare/v0.2.1...v0.2.2
[v0.2.1]: https://github.com/thevilledev/ChibiHash-rs/compare/v0.2.0...v0.2.1
[v0.2.0]: https://github.com/thevilledev/ChibiHash-rs/compare/v0.1.2...v0.2.0
[v0.1.2]: https://github.com/thevilledev/ChibiHash-rs/compare/v0.1.1...v0.1.2
[v0.1.1]: https://github.com/thevilledev/ChibiHash-rs/compare/v0.1.0...v0.1.1
[v0.1.0]: https://github.com/thevilledev/ChibiHash-rs/releases/tag/v0.1.0
