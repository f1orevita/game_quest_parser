# Makefile requirements 

run:
	cargo run -- parse --file test_quest.txt

test:
	cargo test

fmt:
	cargo fmt

lint:
	cargo clippy -- -D warnings

check: fmt lint test
	@echo "All checks passed!"

help:
	cargo run -- --help