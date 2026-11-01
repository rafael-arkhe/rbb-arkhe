.PHONY: all build test lint clean wasm

all: build test

build:
	cargo build --workspace

test:
	cargo test --workspace

lint:
	cargo clippy --workspace --all-targets -- -D warnings

wasm:
	./scripts/build-wasm.sh

clean:
	cargo clean
