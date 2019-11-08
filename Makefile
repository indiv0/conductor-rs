CARGO_FLAGS = --all-features --all-targets
CLIPPY_LINTS = -W clippy::all -W clippy::pedantic -W clippy::nursery
export RUST_LOG = conductor=trace,mockito=trace

all: check clippy build test doc
.PHONY: all bench build check clean clippy doc example run test watch

bench:
	cargo bench $(CARGO_FLAGS)

build:
	cargo build $(CARGO_FLAGS)

check:
	cargo check $(CARGO_FLAGS)

clean:
	cargo clean

clippy:
	cargo clippy $(CARGO_FLAGS) -- $(CLIPPY_LINTS)

doc:
	cargo doc --all-features --no-deps

fmt:
	cargo fmt

run:
	cargo run --example task_def

test:
	cargo test --doc
	cargo test $(CARGO_FLAGS)

watch:
	cargo watch -s "make all run"
