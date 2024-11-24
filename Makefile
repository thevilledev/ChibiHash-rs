.PHONY: all test clean fmt bench bench-cross-lang

all: clean fmt test bench bench-cross-lang

fmt:
	cargo fmt

test:
	cargo test

clean:
	cargo clean

bench:
	RUSTFLAGS="-C opt-level=3 -C target-cpu=native" cargo bench --bench bench

bench-cross-lang:
	RUSTFLAGS="-C opt-level=3 -C target-cpu=native" cargo bench --bench rust_vs_c --features ffi

lint:
	cargo clippy -- -D warnings
