.PHONY: fmt lint lint-fix check spell spell-fix quality setup-hooks test run build clean

# Run the application
run:
	cargo run

# Run tests
test:
	cargo test

# Format code (like Prettier)
fmt:
	cargo fmt
	taplo format

# Lint code (like ESLint)
lint:
	cargo clippy

# Fix linting issues automatically
lint-fix:
	cargo clippy --fix --allow-dirty --allow-staged

# Check code without building (faster)
check:
	cargo check

# Check spelling
spell:
	typos

# Fix spelling issues automatically
spell-fix:
	typos --write-changes

# Setup git hooks
setup-hooks:
	./.githooks/setup.sh

# Run all code quality checks
quality: fmt lint check spell

# Build release version
build:
	cargo build --release

# Clean build artifacts
clean:
	cargo clean

