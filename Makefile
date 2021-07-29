build:
	cargo clean
	cargo build --release

install:
	cargo clean
	cargo install --path .

test:
	cargo test
