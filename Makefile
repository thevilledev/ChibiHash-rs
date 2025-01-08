.PHONY: all test clean fmt fmt-check bench bench-cross-lang

all: clean fmt test bench bench-cross-lang

fmt:
	cargo fmt

fmt-check:
	cargo fmt --check

test: test-std test-no-std

test-std:
	@echo
	@echo "*** Testing with default features ***"
	@echo
	@cargo test

test-no-std:
	@echo
	@echo "*** Testing without default features ***"
	@echo
	@cargo test --no-default-features

clean:
	cargo clean

bench:
	RUSTFLAGS="-C opt-level=3 -C target-cpu=native" cargo bench --bench bench

bench-v2:
	RUSTFLAGS="-C opt-level=3 -C target-cpu=native" cargo bench --bench bench_v2

bench-cross-lang:
	RUSTFLAGS="-C opt-level=3 -C target-cpu=native" cargo bench --bench rust_vs_c --features ffi

lint:
	cargo clippy -- -D warnings
