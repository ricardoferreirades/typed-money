# Typed Money

[![Crates.io](https://img.shields.io/crates/v/typed-money.svg)](https://crates.io/crates/typed-money)
[![Documentation](https://docs.rs/typed-money/badge.svg)](https://docs.rs/typed-money)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://github.com/ricardoferreirades/typed-money/workflows/CI/badge.svg)](https://github.com/ricardoferreirades/typed-money/actions)

A type-safe money library for Rust that prevents currency mixing bugs at compile time. Zero runtime overhead, maximum safety.

## Overview

Typed Money solves a fundamental problem in financial software: preventing currency mixing errors that can lead to costly bugs, incorrect calculations, and financial losses. Traditional approaches rely on runtime checks or primitive types that offer no protection against accidentally adding dollars to euros or mixing different currency units.

This library provides compile-time type safety for monetary values, ensuring that currency operations are explicit and safe. Every currency is a distinct type, making it impossible to accidentally mix currencies in arithmetic operations. The compiler catches these errors before your code ever runs, eliminating an entire class of financial bugs.

## Key Benefits

### Compile-Time Safety
- **Impossible currency mixing**: The compiler prevents adding USD to EUR at build time
- **Zero runtime overhead**: Type safety comes at no performance cost
- **Explicit conversions**: All currency conversions must be intentional and documented

### Financial Precision
- **Deterministic arithmetic**: No floating-point errors, uses decimal arithmetic
- **Currency-aware precision**: Automatic handling of currency-specific decimal places
- **Multiple rounding modes**: Seven rounding strategies for different business requirements

### Developer Experience
- **Clear error messages**: Compiler errors guide you to correct usage
- **Rich metadata**: Access currency information, formatting rules, and trading characteristics
- **Comprehensive examples**: Extensive documentation and examples for all features

### Performance
- **Sub-nanosecond operations**: All arithmetic and conversions are O(1)
- **Memory efficient**: Minimal allocation with optimized memory layout
- **Scalable**: Handles high-frequency trading and large-scale financial applications

## Use Cases

### Financial Applications
- **Banking systems**: Account balances, transfers, and currency conversions
- **Trading platforms**: Order management, portfolio calculations, and risk assessment
- **Payment processing**: Multi-currency transactions and settlement
- **Accounting software**: Financial reporting and compliance calculations

### E-commerce
- **Shopping carts**: Multi-currency pricing and tax calculations
- **Marketplace platforms**: Seller payments in different currencies
- **Subscription billing**: Recurring payments with currency conversion
- **Inventory management**: Cost tracking across different currencies

### Cryptocurrency
- **Trading bots**: Automated trading with multiple cryptocurrencies
- **Wallet applications**: Balance tracking and transaction management
- **DeFi protocols**: Yield farming and liquidity calculations
- **Portfolio management**: Multi-asset portfolio tracking

### Enterprise Systems
- **ERP systems**: Financial modules with multi-currency support
- **Risk management**: Exposure calculations across currencies
- **Compliance reporting**: Regulatory calculations and reporting
- **International operations**: Multi-country financial management

### Gaming and Virtual Economies
- **Game currencies**: In-game money with multiple currency types
- **Virtual marketplaces**: Player-to-player trading systems
- **Subscription models**: Premium currency and real money integration
- **Economic simulation**: Virtual economy modeling and analysis

## Problem Solved

### Traditional Approach Problems

```rust
// Traditional approach - runtime errors possible
let usd_balance = 1000.0;
let eur_balance = 850.0;
let total = usd_balance + eur_balance; // Bug! Mixing currencies
```

### Typed Money Solution

```rust
// Type-safe approach - compile-time safety
let usd_balance = Amount::<USD>::from_major(1000);
let eur_balance = Amount::<EUR>::from_major(850);
// let total = usd_balance + eur_balance; // Compile error - caught at build time!
```

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
typed-money = "*"
```

## Basic Usage

### Creating Amounts

```rust
use typed_money::{Amount, USD, EUR};

// Create amounts from major units (dollars, euros)
let usd_amount = Amount::<USD>::from_major(100); // $100.00
let eur_amount = Amount::<EUR>::from_major(85);  // €85.00

// Create amounts from minor units (cents, euro cents)
let usd_cents = Amount::<USD>::from_minor(10000); // $100.00
let eur_cents = Amount::<EUR>::from_minor(8500);  // €85.00
```

### Safe Arithmetic

```rust
use typed_money::{Amount, USD};

let price = Amount::<USD>::from_major(99);      // $99.00
let tax = Amount::<USD>::from_minor(990);       // $9.90
let total = price + tax;                        // $108.90

// Scalar operations
let discounted = total * 2;                     // $217.80
let half_price = total / 2;                     // $54.45

// This won't compile - different currencies
// let mixed = price + Amount::<EUR>::from_major(10); // Error!
```

### Currency Conversion

```rust
use typed_money::{Amount, Rate, USD, EUR};

let usd_amount = Amount::<USD>::from_major(100);
let rate = Rate::<USD, EUR>::new(0.85);
let eur_amount = usd_amount.convert(&rate);     // €85.00

// Rate with metadata for auditability
let rate_with_metadata = Rate::<USD, EUR>::new(0.85)
    .with_timestamp_unix_secs(1_700_000_000)
    .with_source("ECB");
```

## Advanced Features

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

### Internationalization

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
```

### Precious Metals and Commodities

```rust
use typed_money::{Amount, XAU, XAG, XPT, XPD, XDI, XCU, XAL, CurrencyMetadata};

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

## Supported Currencies

The library supports 69 currencies across multiple categories:

- **Major fiat currencies**: USD, EUR, GBP, JPY, CHF, CAD, AUD, NZD
- **Regional currencies**: Asian, European, American, African, Middle Eastern
- **Cryptocurrencies**: BTC, ETH, LTC, BCH, XRP, ADA, DOT, LINK, UNI, AAVE
- **Stablecoins**: USDT, USDC, DAI, BUSD
- **DeFi tokens**: SUSHI, COMP, MKR, YFI
- **Precious metals**: XAU (Gold), XAG (Silver), XPT (Platinum), XPD (Palladium), XDI (Diamond)
- **Base metals**: XCU (Copper), XAL (Aluminum), XZN (Zinc), XNI (Nickel)

See [CURRENCIES.md](CURRENCIES.md) for a complete reference.

## Performance

Benchmarked on Apple M1 Pro:

| Operation | Time | Throughput |
|-----------|------|------------|
| Addition | ~4.9ns | 204M ops/sec |
| Subtraction | ~5.0ns | 200M ops/sec |
| Currency Conversion | ~5.0ns | 200M ops/sec |
| Rounding | ~5.0ns | 200M ops/sec |
| Equality Check | ~3.2ns | 312M ops/sec |

See [PERFORMANCE.md](PERFORMANCE.md) for detailed benchmarks.

## Feature Flags

```toml
[dependencies]
typed-money = { features = ["serde_support", "conversion_tracking"] }
```

- **`serde_support`** - Enable JSON serialization
- **`conversion_tracking`** - Track currency conversions for auditing
- **`use_bigdecimal`** - Use `bigdecimal` instead of `rust_decimal`

## No-std Support

```toml
[dependencies]
typed-money = { default-features = false }
```

## Documentation

- **[Full API Documentation](https://docs.rs/typed-money)** - Complete reference
- **[Examples](https://docs.rs/typed-money/latest/typed_money/#examples)** - Comprehensive usage examples
- **[Currency Reference](CURRENCIES.md)** - Complete list of supported currencies
- **[Internationalization Guide](INTERNATIONALIZATION.md)** - Locale formatting and metadata
- **[Performance Analysis](PERFORMANCE.md)** - Detailed benchmarks and optimization tips

## Examples

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

# Internationalization features
cargo run --example internationalization

# Currency metadata
cargo run --example currency_metadata

# Precious metals
cargo run --example precious_metals
```

## Why Choose Typed Money?

| Feature | Typed Money | Traditional Libraries |
|---------|-------------|----------------------|
| **Type Safety** | Compile-time | Runtime checks |
| **Performance** | ~5ns operations | ~50-100ns |
| **Currency Mixing** | Impossible | Runtime errors |
| **Precision** | Deterministic | Floating-point errors |
| **Documentation** | Comprehensive | Often minimal |
| **Testing** | 437 tests | Variable coverage |

## Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

## License

Dual-licensed under **MIT OR Apache-2.0**. See [LICENSE-MIT](LICENSE-MIT) for details.

## Acknowledgments

- Built with [rust_decimal](https://crates.io/crates/rust_decimal) for precise decimal arithmetic
- Inspired by [Money](https://github.com/RubyMoney/money) and similar type-safe money libraries
- Performance benchmarking with [Criterion](https://crates.io/crates/criterion)

---

**Made with Rust** | [Report Bug](https://github.com/ricardoferreirades/typed-money/issues) | [Request Feature](https://github.com/ricardoferreirades/typed-money/issues)