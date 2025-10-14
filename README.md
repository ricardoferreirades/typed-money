# Typed Money

A type-safe money library for Rust that prevents currency mixing bugs at compile time.

## 🎯 Features

- ✅ **Type-safe monetary representation** - Compile-time currency safety
- ✅ **Explicit conversions** - No implicit currency conversions
- ✅ **Decimal precision** - Deterministic arithmetic using `rust_decimal`
- ✅ **Zero unsafe code** - 100% safe Rust with `#![forbid(unsafe_code)]`
- ✅ **O(1) performance** - Constant-time operations via compile-time types
- ✅ **No-std support** - Works in embedded and WebAssembly environments
- ✅ **Extensible** - Add custom currencies without modifying the library

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
typed-money = "0.1.0"
```

## 🚀 Quick Start

```rust
use typed_money::{Amount, USD, EUR, Rate};

fn main() {
    // Create amounts with type-safe currencies
    let price = Amount::<USD>::from_major(10);  // $10.00
    
    // Arithmetic with same currency
    let total = price + Amount::<USD>::from_major(5);  // $15.00
    
    // Explicit currency conversion
    let rate = Rate::<USD, EUR>::new(0.85);
    let euro_price = price.convert(&rate);  // €8.50
    
    // Compile error! Cannot mix currencies
    // let invalid = price + euro_price;  // ❌ Won't compile
}
```

## 🛠️ Development

### Prerequisites

Install required tools:

```bash
# Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Development tools
cargo install taplo-cli typos-cli
```

### Setup

Clone and setup the project:

```bash
git clone https://github.com/ricardoferreirades/typed-money.git
cd typed-money

# Setup git hooks for quality enforcement
make setup-hooks
```

### Available Commands

```bash
make run          # Run the application
make test         # Run tests
make fmt          # Format code (cargo fmt + taplo)
make lint         # Run linter (clippy)
make lint-fix     # Auto-fix linting issues
make check        # Type check without building
make spell        # Check spelling
make spell-fix    # Auto-fix spelling issues
make quality      # Run all quality checks
make build        # Build release version
make clean        # Clean build artifacts
```

### Code Quality

This project enforces strict quality standards:

- **Zero warnings policy** - All code must compile without warnings
- **Automated quality checks** - Pre-push hooks prevent bad code from being pushed
- **Conventional commits** - Standardized commit messages
- **100% safe Rust** - No unsafe code allowed
- **Spell checking** - Catch typos in code and documentation

The pre-push hook runs automatically and checks:
- ✅ Code formatting (rustfmt + taplo)
- ✅ Linting (clippy with strict warnings)
- ✅ Spell checking (typos)
- ✅ Type checking (cargo check)
- ✅ Tests (cargo test)

## 📚 Documentation

For detailed implementation guidelines, see:

- [Functional Requirements](features/functional-requirements.md)
- [Non-Functional Requirements](features/non-functional-requirements.md)
- [Implementation Plan](features/IMPLEMENTATION_PLAN.md)
- [Project Setup Guide](PROJECT_SETUP_GUIDE.md)

## 🧪 Testing

Run all tests:

```bash
make test
```

## 📄 License

Dual-licensed under MIT OR Apache-2.0

## 🤝 Contributing

Contributions are welcome! Please read our contributing guidelines and code of conduct before submitting PRs.

1. Fork the repository
2. Create your feature branch (`git checkout -b feat/amazing-feature`)
3. Ensure all quality checks pass (`make quality`)
4. Commit your changes using conventional commits
5. Push to the branch (`git push origin feat/amazing-feature`)
6. Open a Pull Request

## 🔗 Links

- [Repository](https://github.com/ricardoferreirades/typed-money)
- [Documentation](https://docs.rs/typed-money)
- [Crates.io](https://crates.io/crates/typed-money)

---

**Status:** 🚧 In Development

This project is currently in active development. APIs may change before 1.0.0 release.

