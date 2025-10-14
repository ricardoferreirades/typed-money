# Typed Money

A type-safe money library for Rust that prevents currency mixing bugs at compile time.

## 📋 Description

Typed Money is a Rust library designed to provide compile-time guarantees for monetary operations. By leveraging Rust's type system, it makes currency mixing bugs impossible to write - the compiler will catch them before your code even runs.

## 🎯 Purpose

Financial applications require absolute precision and correctness. Traditional money libraries often rely on runtime checks or allow implicit conversions, which can lead to catastrophic bugs in production. Typed Money takes a different approach:

- **Compile-time safety** - Currency types are checked at compile time, not runtime
- **Zero-cost abstractions** - Type safety comes with no performance overhead
- **Explicit conversions** - Currency conversions must be explicit and auditable
- **Deterministic arithmetic** - No floating-point errors, all operations are precise

## 🛠️ Development

### Prerequisites

- [Rust](https://rustup.rs/) (1.70 or later)
- [Make](https://www.gnu.org/software/make/) (usually pre-installed on macOS/Linux)

### Setup

```bash
# Clone the repository
git clone https://github.com/ricardoferreirades/typed-money.git
cd typed-money

# Install all dependencies and configure git hooks (one command!)
make setup
```

This will:
- Install development tools (taplo-cli, typos-cli)
- Configure git hooks for quality enforcement
- Display available commands

### Available Commands

```bash
make setup        # Complete setup (install deps + git hooks)
make install-deps # Install development dependencies only
make run          # Run the application
make test         # Run tests
make fmt          # Format code
make lint         # Run clippy linter
make check        # Type check
make spell        # Check spelling
make quality      # Run all quality checks
```

## 📄 License

Dual-licensed under MIT OR Apache-2.0

## 🤝 Contributing

Contributions are welcome! Please read the implementation plan and ensure all quality checks pass before submitting PRs.

---

**Made with ❤️ in Rust**
