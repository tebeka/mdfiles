.PHONY: test build fmt clippy clean

test: build fmt clippy
	cargo test --verbose

build:
	cargo build --release

fmt:
	cargo fmt -- --check

clippy:
	cargo clippy -- -D warnings

clean:
	cargo clean
