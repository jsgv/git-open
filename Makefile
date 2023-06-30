.PHONY: test
test:
	cargo clean
	cargo fmt --all -- --check
	cargo clippy -- -D warnings
	cargo test

.PHONY: build
build: test
	cargo clean
	cargo build --release

.PHONY: install
install: test
	cargo clean
	cargo install --path .

