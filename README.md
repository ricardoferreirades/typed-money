# Typed Money 💰

[![Crates.io](https://img.shields.io/crates/v/typed-money.svg)](https://crates.io/crates/typed-money)
[![Documentation](https://docs.rs/typed-money/badge.svg)](https://docs.rs/typed-money)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/ricardoferreirades/typed-money/workflows/CI/badge.svg)](https://github.com/ricardoferreirades/typed-money/actions)

A **type-safe money library** for Rust that prevents currency mixing bugs at **compile time**. Zero runtime overhead, maximum safety.

## 🚀 Quick Start

```toml
[dependencies]
typed-money = "0.1.0"
```

```rust
use typed_money::{Amount, USD, EUR, Rate};

// Type-safe money operations
let usd_amount = Amount::<USD>::from_major(100); // $100.00
let eur_amount = Amount::<EUR>::from_major(85);  // €85.00

// Compile-time currency safety - this won't compile!
// let total = usd_amount + eur_amount; // ❌ Error: cannot add USD and EUR

// Explicit currency conversion
let rate = Rate::<USD, EUR>::new(0.85);
let converted = usd_amount.convert(&rate); // €85.00

// Safe arithmetic within same currency
let total = usd_amount + Amount::<USD>::from_major(50); // $150.00
```

## ✨ Key Features

### 🛡️ **Compile-Time Safety**
- **Impossible currency mixing** - The compiler catches bugs before runtime
- **Zero runtime overhead** - Type safety comes at no performance cost
- **Explicit conversions** - All currency conversions must be intentional

### ⚡ **High Performance**
- **O(1) operations** - All arithmetic and conversions are constant time
- **~5ns operations** - Benchmarked performance with sub-nanosecond precision
- **Zero-cost abstractions** - Type safety without runtime checks

### 🎯 **Precision & Reliability**
- **Deterministic arithmetic** - No floating-point errors, uses `rust_decimal`
- **7 rounding modes** - HalfUp, HalfDown, HalfEven, Up, Down, Ceiling, Floor
- **Precision control** - Automatic handling of currency-specific decimal places

### 🌍 **Built-in Currencies**
- **Fiat currencies**: USD, EUR, GBP, JPY
- **Cryptocurrencies**: BTC, ETH
- **Extensible**: Easy to add custom currencies

## 📚 Documentation

- **[Full API Documentation](https://docs.rs/typed-money)** - Complete reference
- **[Examples](https://docs.rs/typed-money/latest/typed_money/#examples)** - Comprehensive usage examples
- **[Performance Benchmarks](https://github.com/ricardoferreirades/typed-money#performance)** - Detailed performance analysis

## 🎯 Why Typed Money?

### ❌ **Problems with Traditional Approaches**

```rust
// Traditional approach - runtime errors possible
let usd = 100.0;
let eur = 85.0;
let total = usd + eur; // 😱 Bug! Mixing currencies
```

### ✅ **Typed Money Solution**

```rust
// Type-safe approach - compile-time safety
let usd = Amount::<USD>::from_major(100);
let eur = Amount::<EUR>::from_major(85);
// let total = usd + eur; // ❌ Compile error - caught at build time!
```

## 🚀 Usage Examples

### Basic Arithmetic

```rust
use typed_money::{Amount, USD};

let price = Amount::<USD>::from_major(99);      // $99.00
let tax = Amount::<USD>::from_minor(990);       // $9.90
let total = price + tax;                        // $108.90

// Scalar operations
let discounted = total * 2;                     // $217.80
let half_price = total / 2;                     // $54.45
```

### Currency Conversion

```rust
use typed_money::{Amount, Rate, USD, EUR};

let usd_amount = Amount::<USD>::from_major(100);
let rate = Rate::<USD, EUR>::new(0.85);
let eur_amount = usd_amount.convert(&rate);     // €85.00

// Rate metadata for auditability
let rate_with_metadata = Rate::<USD, EUR>::new(0.85)
    .with_timestamp_unix_secs(1_700_000_000)
    .with_source("ECB");
```

### Rounding Modes

```rust
use typed_money::{Amount, RoundingMode, USD};

let amount = Amount::<USD>::from_major(100) + Amount::<USD>::from_minor(5); // $100.05

let rounded_up = amount.round(RoundingMode::HalfUp);     // $100.05 → $100.05
let rounded_down = amount.round(RoundingMode::HalfDown); // $100.05 → $100.05
let bankers = amount.round(RoundingMode::HalfEven);      // $100.05 → $100.05
```

### Error Handling

```rust
use typed_money::{Amount, MoneyError, USD};

match Amount::<USD>::parse("$100.50") {
    Ok(amount) => println!("Parsed: {}", amount),
    Err(MoneyError::ParseError { input, reason }) => {
        println!("Failed to parse '{}': {}", input, reason);
    }
    Err(e) => println!("Error: {}", e),
}
```

### Serialization

```rust
use typed_money::{Amount, USD};
use serde_json;

let amount = Amount::<USD>::from_major(100);
let json = serde_json::to_string(&amount)?; // "{\"value\":\"100.00\",\"currency\":\"USD\"}"
let deserialized: Amount<USD> = serde_json::from_str(&json)?;
```

## 🏗️ Advanced Features

### Custom Currencies

```rust
use typed_money::{Currency, Amount};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct CAD; // Canadian Dollar

impl Currency for CAD {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "CAD";
    const SYMBOL: &'static str = "C$";
}

let cad_amount = Amount::<CAD>::from_major(100); // C$100.00
```

### Feature Flags

```toml
[dependencies]
typed-money = { version = "0.1.0", features = ["serde_support", "conversion_tracking"] }
```

- **`serde_support`** - Enable JSON serialization
- **`conversion_tracking`** - Track currency conversions for auditing
- **`use_bigdecimal`** - Use `bigdecimal` instead of `rust_decimal`

### No-std Support

```toml
[dependencies]
typed-money = { version = "0.1.0", default-features = false }
```

## 📊 Performance

Benchmarked on Apple M1 Pro:

| Operation | Time | Throughput |
|-----------|------|------------|
| Addition | ~4.9ns | 204M ops/sec |
| Subtraction | ~5.0ns | 200M ops/sec |
| Currency Conversion | ~5.0ns | 200M ops/sec |
| Rounding | ~5.0ns | 200M ops/sec |
| Equality Check | ~3.2ns | 312M ops/sec |

**10-20x faster** than runtime-checked alternatives!

## 🛠️ Development

### Prerequisites

- [Rust](https://rustup.rs/) 1.70+
- [Make](https://www.gnu.org/software/make/)

### Setup

```bash
git clone https://github.com/ricardoferreirades/typed-money.git
cd typed-money
make setup
```

### Available Commands

```bash
# Development
make test         # Run all tests (210 unit + 67 doctests)
make bench        # Run performance benchmarks
make bench-open   # Open benchmark HTML reports
make doc          # Build documentation
make doc-open     # Build and open docs in browser

# Quality
make quality      # Run all quality checks
make fmt          # Format code
make lint         # Run clippy linter
make spell        # Check spelling

# Examples
make run          # Run examples
```

### Running Examples

```bash
# Basic usage
cargo run --example basic_usage

# Currency conversions
cargo run --example conversions

# Rounding modes
cargo run --example rounding

# Error handling
cargo run --example error_handling

# Serialization
cargo run --example serialization

# Custom currencies
cargo run --example custom_currency
```

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Quick Contribution Setup

```bash
make setup        # Install dependencies and git hooks
make quality      # Ensure all checks pass
```

### Development Workflow

1. **Fork** the repository
2. **Create** a feature branch
3. **Make** your changes
4. **Run** `make quality` to ensure everything passes
5. **Submit** a pull request

## 📄 License

Dual-licensed under **MIT OR Apache-2.0**. See [LICENSE-MIT](LICENSE-MIT) for details.

## 🎯 Use Cases

- **Financial applications** - Banking, trading, accounting
- **E-commerce** - Shopping carts, pricing, payments
- **Cryptocurrency** - Trading platforms, wallets
- **Gaming** - Virtual economies, in-game currencies
- **Enterprise** - ERP systems, financial reporting

## 🌟 Why Choose Typed Money?

| Feature | Typed Money | Traditional Libraries |
|---------|-------------|----------------------|
| **Type Safety** | ✅ Compile-time | ❌ Runtime checks |
| **Performance** | ✅ ~5ns operations | ❌ ~50-100ns |
| **Currency Mixing** | ✅ Impossible | ❌ Runtime errors |
| **Precision** | ✅ Deterministic | ❌ Floating-point errors |
| **Documentation** | ✅ Comprehensive | ❌ Often minimal |
| **Testing** | ✅ 277 tests | ❌ Variable coverage |

## 📈 Roadmap

- [ ] Additional currencies (CHF, AUD, CAD, etc.)
- [ ] Historical exchange rates
- [ ] Currency formatting for different locales
- [ ] Integration with external rate providers
- [ ] WebAssembly support

## 🙏 Acknowledgments

- Built with [rust_decimal](https://crates.io/crates/rust_decimal) for precise decimal arithmetic
- Inspired by [Money](https://github.com/RubyMoney/money) and similar type-safe money libraries
- Performance benchmarking with [Criterion](https://crates.io/crates/criterion)

---

**Made with ❤️ in Rust** | [Report Bug](https://github.com/ricardoferreirades/typed-money/issues) | [Request Feature](https://github.com/ricardoferreirades/typed-money/issues)