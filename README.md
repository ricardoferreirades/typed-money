# Typed Money 💰

[![Crates.io](https://img.shields.io/crates/v/typed-money.svg)](https://crates.io/crates/typed-money)
[![Documentation](https://docs.rs/typed-money/badge.svg)](https://docs.rs/typed-money)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/ricardoferreirades/typed-money/workflows/CI/badge.svg)](https://github.com/ricardoferreirades/typed-money/actions)

A **type-safe money library** for Rust that prevents currency mixing bugs at **compile time**. Zero runtime overhead, maximum safety.

## 🎉 **v0.2.0 - Major Release!**

**NEW FEATURES:**
- 🌍 **69 currencies** with full internationalization support
- 📊 **Rich metadata system** - Country, region, volatility, liquidity ratings
- 💎 **Precious metals** - Gold, Silver, Platinum, Palladium, Diamond
- 🏭 **Base metals** - Copper, Aluminum, Zinc, Nickel  
- 🚀 **Expanded crypto** - Major cryptos, stablecoins, DeFi tokens
- 🎨 **Locale formatting** - European, Asian, American, African styles
- 📈 **Portfolio analysis** - Built-in trading insights and risk assessment

## 🚀 Quick Start

```toml
[dependencies]
typed-money = "0.2.0"
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

### 🌍 **Comprehensive Currency Support**
- **69 currencies** with full internationalization support
- **Major fiat**: USD, EUR, GBP, JPY, CHF, CAD, AUD, NZD
- **Regional currencies**: Asian, European, American, African, Middle Eastern
- **Cryptocurrencies**: BTC, ETH, LTC, BCH, XRP, ADA, DOT, LINK, UNI, AAVE
- **Stablecoins**: USDT, USDC, DAI, BUSD
- **DeFi tokens**: SUSHI, COMP, MKR, YFI
- **Precious metals**: XAU (Gold), XAG (Silver), XPT (Platinum), XPD (Palladium), XDI (Diamond)
- **Base metals**: XCU (Copper), XAL (Aluminum), XZN (Zinc), XNI (Nickel)
- **Rich metadata**: Country, region, volatility, liquidity ratings, formatting rules

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

### Internationalization & Metadata

```rust
use typed_money::{Amount, USD, EUR, BRL, CurrencyMetadata};

// Access rich currency metadata
let usd_amount = Amount::<USD>::from_major(1234);
println!("Currency: {}", usd_amount.currency_name());     // "US Dollar"
println!("Country: {}", usd_amount.currency_country());   // "United States"
println!("Region: {}", usd_amount.currency_region());     // "North America"
println!("Type: {}", usd_amount.currency_type());         // "Fiat"
println!("Volatility: {}", usd_amount.volatility_rating()); // "Low"
println!("Liquidity: {}", usd_amount.liquidity_rating());   // "High"

// Locale-specific formatting
let eur_amount = Amount::<EUR>::from_major(1234);
println!("European format: {}", eur_amount); // "€1.234,00 EUR"

let brl_amount = Amount::<BRL>::from_major(1234);
println!("Brazilian format: {}", brl_amount); // "R$1.234,00 BRL"

// Portfolio analysis
let portfolio = vec![
    ("USD", Amount::<USD>::from_major(10000)),
    ("EUR", Amount::<EUR>::from_major(8500)),
    ("BTC", Amount::<BTC>::from_major(1)),
    ("XAU", Amount::<XAU>::from_major(10)),
];

for (name, amount) in &portfolio {
    println!("{}: {} - Type: {}, Volatility: {}, Liquidity: {}", 
        name, amount, 
        amount.currency_type(),
        amount.volatility_rating(),
        amount.liquidity_rating()
    );
}
```

### Precious Metals & Commodities

```rust
use typed_money::{Amount, XAU, XAG, XPT, XPD, XDI};

// Precious metals with 4 decimal precision
let gold = Amount::<XAU>::from_major(1);        // Au1.0000 XAU
let silver = Amount::<XAG>::from_major(100);    // Ag100.0000 XAG
let platinum = Amount::<XPT>::from_major(1);    // Pt1.0000 XPT
let palladium = Amount::<XPD>::from_major(1);   // Pd1.0000 XPD
let diamond = Amount::<XDI>::from_major(1);     // Di1.0000 XDI

// Base metals
let copper = Amount::<XCU>::from_major(1000);   // Cu1000.0000 XCU
let aluminum = Amount::<XAL>::from_major(1000); // Al1000.0000 XAL

// Access commodity metadata
println!("Gold region: {}", gold.currency_region());     // "Worldwide"
println!("Gold type: {}", gold.currency_type());         // "Commodity"
println!("Gold volatility: {}", gold.volatility_rating()); // "Medium"
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
typed-money = { version = "0.2.0", features = ["serde_support", "conversion_tracking"] }
```

- **`serde_support`** - Enable JSON serialization
- **`conversion_tracking`** - Track currency conversions for auditing
- **`use_bigdecimal`** - Use `bigdecimal` instead of `rust_decimal`

### No-std Support

```toml
[dependencies]
typed-money = { version = "0.2.0", default-features = false }
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
make test         # Run all tests (437 unit + doctests)
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

# NEW: Internationalization features
cargo run --example internationalization

# NEW: Currency metadata
cargo run --example currency_metadata

# NEW: Precious metals
cargo run --example precious_metals

# NEW: Global currencies
cargo run --example global_currencies
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
| **Testing** | ✅ 437 tests | ❌ Variable coverage |

## 📈 Roadmap

- [x] **Additional currencies** - 69 currencies with full internationalization support
- [x] **Currency formatting for different locales** - Complete i18n implementation
- [x] **Rich metadata system** - Country, region, volatility, liquidity ratings
- [x] **Precious metals support** - Gold, Silver, Platinum, Palladium, Diamond
- [x] **Base metals support** - Copper, Aluminum, Zinc, Nickel
- [x] **Cryptocurrency expansion** - Major cryptos, stablecoins, DeFi tokens
- [ ] Historical exchange rates
- [ ] Integration with external rate providers
- [ ] WebAssembly support
- [ ] Real-time rate updates

## 🙏 Acknowledgments

- Built with [rust_decimal](https://crates.io/crates/rust_decimal) for precise decimal arithmetic
- Inspired by [Money](https://github.com/RubyMoney/money) and similar type-safe money libraries
- Performance benchmarking with [Criterion](https://crates.io/crates/criterion)

---

**Made with ❤️ in Rust** | [Report Bug](https://github.com/ricardoferreirades/typed-money/issues) | [Request Feature](https://github.com/ricardoferreirades/typed-money/issues)