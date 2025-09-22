# Makefile for agg-rs
.PHONY: help build test clean coverage coverage-html coverage-open lint fmt

# Default target
help:
	@echo "Available targets:"
	@echo "  build          - Build the project"
	@echo "  test           - Run tests"
	@echo "  coverage       - Generate text coverage report"
	@echo "  coverage-html  - Generate HTML coverage report"
	@echo "  coverage-open  - Generate and open HTML coverage report"
	@echo "  lint           - Run clippy linter"
	@echo "  fmt            - Format code with rustfmt"
	@echo "  clean          - Clean build artifacts"

build:
	cargo build

test:
	cargo test

coverage:
	./scripts/coverage.sh --format text

coverage-html:
	./scripts/coverage.sh --format html

coverage-open:
	./scripts/coverage.sh --format html --open

lint:
	cargo clippy --all-targets --all-features -- -D warnings

fmt:
	cargo fmt

clean:
	cargo clean
	rm -rf target/coverage target/coverage-html
	rm -f coverage.info lcov.info