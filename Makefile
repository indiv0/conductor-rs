CARGO_FLAGS = --all-features --all-targets
CLIPPY_LINTS = -W clippy::all -W clippy::pedantic -W clippy::restriction -W clippy::nursery

all: check clippy build test doc
.PHONY: all bench build check clean clippy doc fmt test watch

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
	cargo watch -s make
