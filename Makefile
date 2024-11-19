.PHONY: all test clean fmt bench

all: clean fmt test bench

fmt:
	cargo fmt

test:
	cargo test

clean:
	cargo clean

bench:
	cargo bench