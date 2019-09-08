.PHONEY: test build run

test:
	cargo fmt
	cargo test

build:
	cargo build --release

run: build
	cargo run