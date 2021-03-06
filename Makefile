CLIPPY_LINTS = -W clippy::all -W clippy::pedantic -W clippy::restriction -W clippy::nursery

all: check clippy build test doc
.PHONY: all bench build check clean clippy doc fmt test watch

bench:
	cargo bench

build:
	cargo build --lib --examples --tests --benches

check:
	cargo check

clean:
	cargo clean

clippy:
	cargo clippy -- $(CLIPPY_LINTS)

doc:
	cargo doc

fmt:
	cargo fmt

test:
	cargo test

watch:
	cargo watch -s make
