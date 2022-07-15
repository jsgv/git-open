.PHONY: test build install

test:
	cargo fmt --all -- --check
	cargo clippy -- -D warnings
	cargo test

build: test
	cargo clean
	cargo build --release

install: test
	cargo clean
	cargo install --path .

