.PHONY: all test clean

install-dev:
	rustup component add clippy
	cargo install cargo-watch

clean:
	cargo clean

lint:
	cargo clippy

fix:
	cargo clippy --fix
	cargo fmt

build:
	cargo build

test:
	cargo test

bench:
	CRITERION_DEBUG=1 cargo bench
