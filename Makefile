.PHONY: fmt lint lint-fix check spell spell-fix quality setup-hooks test run build clean install-deps setup

# Install all development dependencies
install-deps:
	@echo "📦 Installing development dependencies..."
	@echo "Installing taplo-cli (TOML formatter)..."
	@cargo install taplo-cli --quiet
	@echo "Installing typos-cli (spell checker)..."
	@cargo install typos-cli --quiet
	@echo "✅ All development dependencies installed!"

# Complete setup for new developers (install deps + setup hooks)
setup: install-deps setup-hooks
	@echo ""
	@echo "🎉 Setup complete! You're ready to develop."
	@echo ""
	@echo "Available commands:"
	@echo "  make run       - Run the application"
	@echo "  make test      - Run tests"
	@echo "  make quality   - Run all quality checks"
	@echo "  make fmt       - Format code"

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

