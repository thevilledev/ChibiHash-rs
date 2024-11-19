.PHONY: all test clean fmt bench

all: test

test:
	cargo test

clean:
	cargo clean

bench:
	cargo bench