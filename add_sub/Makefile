# Makefile for Rust Project
check:
	cargo check

build:
	cargo build

format:
	cargo fmt

lint:
	cargo clippy

test:
	cargo test

release:
	cargo build --release

clean:
	cargo clean

run:
	cargo run -- 100 99

all: check build format lint test release run
