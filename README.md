# Typed Money 💰

[![Crates.io](https://img.shields.io/crates/v/typed-money.svg)](https://crates.io/crates/typed-money)
[![Documentation](https://docs.rs/typed-money/badge.svg)](https://docs.rs/typed-money)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/github/actions/workflow/status/ricardoferreirades/typed-money/ci.yml?branch=master)](https://github.com/ricardoferreirades/typed-money/actions)

> A type-safe, compile-time guaranteed money library for Rust that makes currency mixing bugs impossible.

## 🎯 Why Typed Money?

**The Problem:** Traditional money libraries use runtime checks or allow implicit conversions, leading to catastrophic bugs:

```rust
// ❌ Runtime disaster waiting to happen
let usd_price = Money::new(100, "USD");
let eur_price = Money::new(85, "EUR");
let total = usd_price + eur_price;  // 💥 Mixed currencies!
```

**The Solution:** Typed Money uses Rust's type system to prevent currency mixing at **compile time**:

```rust
// ✅ Compile-time safety
let usd_price = Amount::<USD>::from_major(100);
let eur_price = Amount::<EUR>::from_major(85);
// let total = usd_price + eur_price;  // ❌ Won't compile!
```

## ✨ Features

- 🛡️ **Type-Safe** - Currency mixing prevented at compile time
- 🚀 **Zero-Cost Abstractions** - O(1) operations, no runtime overhead
- 🔒 **100% Safe Rust** - `#![forbid(unsafe_code)]`, memory-safe by design
- 🎯 **Deterministic** - Uses `rust_decimal` for precise arithmetic (no floating point)
- 🌍 **No-std Compatible** - Works in embedded systems and WebAssembly
- 🔄 **Explicit Conversions** - Exchange rates are always explicit and auditable
- 📦 **Extensible** - Add custom currencies without modifying the library
- ⚡ **Performance** - Compile-time types eliminate runtime checks

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
typed-money = "0.1.0"

# Optional features
[features]
serde = ["dep:serde"]           # Serialization support
bigdecimal = ["dep:bigdecimal"] # Alternative to rust_decimal
```

## 🚀 Quick Start

```rust
use typed_money::{Amount, USD, EUR, JPY, Rate};

fn main() {
    // Create amounts with type-safe currencies
    let salary = Amount::<USD>::from_major(5000);  // $5,000.00
    let bonus = Amount::<USD>::from_minor(50000);  // $500.00 (from cents)
    
    // Type-safe arithmetic
    let total = salary + bonus;  // $5,500.00
    println!("Total: {}", total);  // "Total: $5,500.00 USD"
    
    // Explicit currency conversion
    let usd_to_eur = Rate::<USD, EUR>::new(0.85);
    let salary_eur = salary.convert(&usd_to_eur);  // €4,250.00
    
    // This won't compile! ❌
    // let mixed = salary + salary_eur;  // Compile error: type mismatch
}
```

## 📚 Use Cases

### 1. E-Commerce Platform

```rust
use typed_money::{Amount, USD, EUR, GBP, Rate};

struct Product {
    name: String,
    price: Amount<USD>,
}

struct ShoppingCart {
    items: Vec<(Product, u32)>,  // (product, quantity)
}

impl ShoppingCart {
    fn total(&self) -> Amount<USD> {
        self.items
            .iter()
            .map(|(product, qty)| product.price * (*qty as i64))
            .sum()
    }
    
    fn total_in_currency<C>(&self, rate: &Rate<USD, C>) -> Amount<C> 
    where
        C: Currency,
    {
        self.total().convert(rate)
    }
}

fn main() {
    let cart = ShoppingCart {
        items: vec![
            (Product { name: "Laptop".into(), price: Amount::from_major(999) }, 1),
            (Product { name: "Mouse".into(), price: Amount::from_major(29) }, 2),
        ],
    };
    
    let total = cart.total();  // $1,057.00
    
    // Show prices in different currencies
    let eur_rate = Rate::<USD, EUR>::new(0.85);
    let gbp_rate = Rate::<USD, GBP>::new(0.73);
    
    println!("USD: {}", total);
    println!("EUR: {}", cart.total_in_currency(&eur_rate));
    println!("GBP: {}", cart.total_in_currency(&gbp_rate));
}
```

### 2. Cryptocurrency Exchange

```rust
use typed_money::{Amount, USD, BTC, ETH, Rate};

struct Exchange {
    btc_usd_rate: Rate<BTC, USD>,
    eth_usd_rate: Rate<ETH, USD>,
}

impl Exchange {
    fn swap_btc_to_eth(&self, btc_amount: Amount<BTC>) -> Amount<ETH> {
        // Convert BTC -> USD -> ETH
        let usd_amount = btc_amount.convert(&self.btc_usd_rate);
        let eth_usd_rate = Rate::<USD, ETH>::new(
            1.0 / self.eth_usd_rate.rate()
        );
        usd_amount.convert(&eth_usd_rate)
    }
    
    fn portfolio_value_usd(
        &self,
        btc_balance: Amount<BTC>,
        eth_balance: Amount<ETH>,
    ) -> Amount<USD> {
        btc_balance.convert(&self.btc_usd_rate) 
            + eth_balance.convert(&self.eth_usd_rate)
    }
}

fn main() {
    let exchange = Exchange {
        btc_usd_rate: Rate::new(45000.0),
        eth_usd_rate: Rate::new(3000.0),
    };
    
    let my_btc = Amount::<BTC>::from_major(2);      // 2 BTC
    let my_eth = Amount::<ETH>::from_major(10);     // 10 ETH
    
    let portfolio_value = exchange.portfolio_value_usd(my_btc, my_eth);
    println!("Portfolio value: {}", portfolio_value);  // $120,000 USD
}
```

### 3. International Payroll System

```rust
use typed_money::{Amount, USD, EUR, GBP, JPY, Rate, RoundingMode};

struct Employee {
    name: String,
    salary: Amount<USD>,
    currency: CurrencyType,
}

enum CurrencyType {
    USD,
    EUR,
    GBP,
    JPY,
}

struct PayrollSystem {
    exchange_rates: ExchangeRates,
}

struct ExchangeRates {
    usd_to_eur: Rate<USD, EUR>,
    usd_to_gbp: Rate<USD, GBP>,
    usd_to_jpy: Rate<USD, JPY>,
}

impl PayrollSystem {
    fn calculate_net_pay(&self, employee: &Employee) -> String {
        match employee.currency {
            CurrencyType::USD => {
                format!("{}", employee.salary)
            },
            CurrencyType::EUR => {
                let amount = employee.salary
                    .convert(&self.exchange_rates.usd_to_eur)
                    .round(RoundingMode::HalfEven);
                format!("{}", amount)
            },
            CurrencyType::GBP => {
                let amount = employee.salary
                    .convert(&self.exchange_rates.usd_to_gbp)
                    .round(RoundingMode::HalfEven);
                format!("{}", amount)
            },
            CurrencyType::JPY => {
                let amount = employee.salary
                    .convert(&self.exchange_rates.usd_to_jpy)
                    .round(RoundingMode::HalfEven);
                format!("{}", amount)
            },
        }
    }
}
```

### 4. Tax Calculation System

```rust
use typed_money::{Amount, USD, RoundingMode};

fn calculate_tax(income: Amount<USD>, tax_rate: f64) -> Amount<USD> {
    (income * tax_rate).round(RoundingMode::HalfUp)
}

fn calculate_net_income(
    gross: Amount<USD>,
    federal_rate: f64,
    state_rate: f64,
) -> Amount<USD> {
    let federal_tax = calculate_tax(gross, federal_rate);
    let state_tax = calculate_tax(gross, state_rate);
    let total_tax = federal_tax + state_tax;
    
    gross - total_tax
}

fn main() {
    let gross_income = Amount::<USD>::from_major(100_000);
    let net_income = calculate_net_income(
        gross_income,
        0.22,  // 22% federal
        0.05,  // 5% state
    );
    
    println!("Gross: {}", gross_income);
    println!("Net:   {}", net_income);
}
```

### 5. Multi-Currency Invoice System

```rust
use typed_money::{Amount, Currency, Rate};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Invoice<C: Currency> {
    id: String,
    items: Vec<LineItem<C>>,
    tax_rate: f64,
}

#[derive(Serialize, Deserialize)]
struct LineItem<C: Currency> {
    description: String,
    quantity: u32,
    unit_price: Amount<C>,
}

impl<C: Currency> Invoice<C> {
    fn subtotal(&self) -> Amount<C> {
        self.items
            .iter()
            .map(|item| item.unit_price * item.quantity as i64)
            .sum()
    }
    
    fn tax(&self) -> Amount<C> {
        (self.subtotal() * self.tax_rate)
            .round(RoundingMode::HalfUp)
    }
    
    fn total(&self) -> Amount<C> {
        self.subtotal() + self.tax()
    }
    
    fn convert_to<D: Currency>(&self, rate: &Rate<C, D>) -> Invoice<D> {
        Invoice {
            id: self.id.clone(),
            items: self.items
                .iter()
                .map(|item| LineItem {
                    description: item.description.clone(),
                    quantity: item.quantity,
                    unit_price: item.unit_price.convert(rate),
                })
                .collect(),
            tax_rate: self.tax_rate,
        }
    }
}
```

## 🔧 Advanced Features

### Custom Currencies

Define your own currencies:

```rust
use typed_money::{Currency, Amount};

#[derive(Debug, Clone, Copy)]
struct BitcoinSatoshi;

impl Currency for BitcoinSatoshi {
    const DECIMALS: u8 = 0;  // Satoshis have no decimals
    const CODE: &'static str = "SAT";
    const SYMBOL: &'static str = "⚡";
}

fn main() {
    let payment = Amount::<BitcoinSatoshi>::from_minor(100_000);
    println!("{}", payment);  // "⚡100000 SAT"
}
```

### Custom Units (Beyond Currency)

```rust
use typed_money::{Amount, Unit};

#[derive(Debug, Clone, Copy)]
struct Kilogram;

impl Unit for Kilogram {
    const DECIMALS: u8 = 3;
    const CODE: &'static str = "KG";
    const SYMBOL: &'static str = "kg";
}

fn main() {
    let weight = Amount::<Kilogram>::from_major(5);  // 5.000 kg
    let total_weight = weight * 3;                    // 15.000 kg
}
```

### Rounding Modes

```rust
use typed_money::{Amount, USD, RoundingMode};

let amount = Amount::<USD>::from_str("10.125").unwrap();

// Different rounding strategies
let half_up = amount.round(RoundingMode::HalfUp);      // $10.13
let half_down = amount.round(RoundingMode::HalfDown);  // $10.12
let half_even = amount.round(RoundingMode::HalfEven);  // $10.12 (banker's)
let ceiling = amount.round(RoundingMode::Ceiling);      // $10.13
let floor = amount.round(RoundingMode::Floor);          // $10.12
```

### Serialization with Serde

```rust
use typed_money::{Amount, USD};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Transaction {
    id: u64,
    amount: Amount<USD>,
    description: String,
}

fn main() {
    let tx = Transaction {
        id: 1,
        amount: Amount::<USD>::from_major(100),
        description: "Payment".into(),
    };
    
    let json = serde_json::to_string(&tx).unwrap();
    println!("{}", json);
    // {"id":1,"amount":"100.00","description":"Payment"}
}
```

### Error Handling

```rust
use typed_money::{Amount, USD, EUR, Rate, MoneyError};

fn process_payment(amount: Amount<USD>) -> Result<Amount<EUR>, MoneyError> {
    // Validate amount
    if amount < Amount::from_major(0) {
        return Err(MoneyError::InvalidAmount);
    }
    
    // Get exchange rate (could fail)
    let rate = get_exchange_rate()?;
    
    // Convert
    Ok(amount.convert(&rate))
}

fn get_exchange_rate() -> Result<Rate<USD, EUR>, MoneyError> {
    // In real app, fetch from API
    Rate::try_new(0.85)
        .ok_or(MoneyError::InvalidRate)
}
```

## 🏗️ Architecture & Design

### Type Safety Guarantees

```rust
// ✅ Same currency operations - always safe
let a = Amount::<USD>::from_major(100);
let b = Amount::<USD>::from_major(50);
let sum = a + b;  // Works!

// ❌ Cross-currency operations - compile error
let c = Amount::<EUR>::from_major(100);
// let invalid = a + c;  // Compile error: type mismatch

// ✅ Explicit conversion required
let rate = Rate::<USD, EUR>::new(0.85);
let converted = a.convert(&rate);
let valid = converted + c;  // Now it works!
```

### Zero Runtime Overhead

The type system ensures zero runtime cost:

```rust
// These have the same performance characteristics
let fast = Amount::<USD>::from_major(100) + Amount::<USD>::from_major(50);
let also_fast = 100 + 50;  // Same O(1) complexity

// Currency checks happen at compile time, not runtime
```

### Auditability

```rust
use typed_money::{Amount, USD, EUR, Rate};
use chrono::Utc;

struct AuditableRate<From: Currency, To: Currency> {
    rate: Rate<From, To>,
    timestamp: DateTime<Utc>,
    source: String,
}

impl AuditableRate<USD, EUR> {
    fn convert(&self, amount: Amount<USD>) -> (Amount<EUR>, AuditLog) {
        let result = amount.convert(&self.rate);
        let log = AuditLog {
            from_amount: amount,
            to_amount: result,
            rate_used: self.rate.value(),
            timestamp: self.timestamp,
            source: self.source.clone(),
        };
        (result, log)
    }
}
```

## 🧪 Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_arithmetic() {
        let a = Amount::<USD>::from_major(100);
        let b = Amount::<USD>::from_major(50);
        assert_eq!(a + b, Amount::<USD>::from_major(150));
    }

    #[test]
    fn test_conversion() {
        let usd = Amount::<USD>::from_major(100);
        let rate = Rate::<USD, EUR>::new(0.85);
        let eur = usd.convert(&rate);
        assert_eq!(eur, Amount::<EUR>::from_major(85));
    }

    #[test]
    fn test_rounding() {
        let amount = Amount::<USD>::from_str("10.125").unwrap();
        let rounded = amount.round(RoundingMode::HalfUp);
        assert_eq!(rounded, Amount::<USD>::from_str("10.13").unwrap());
    }
}
```

## 🛠️ Development

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install development tools
cargo install taplo-cli typos-cli
```

### Setup

```bash
git clone https://github.com/ricardoferreirades/typed-money.git
cd typed-money

# Setup git hooks for quality enforcement
make setup-hooks
```

### Available Commands

```bash
make run          # Run the application
make test         # Run all tests
make fmt          # Format code
make lint         # Run clippy linter
make lint-fix     # Auto-fix linting issues
make check        # Type check (fast)
make spell        # Check spelling
make spell-fix    # Fix spelling issues
make quality      # Run all quality checks
make build        # Build release version
```

### Quality Standards

This project enforces professional quality standards:

- ✅ **Zero warnings** - `clippy` with `-D warnings`
- ✅ **100% safe Rust** - `#![forbid(unsafe_code)]`
- ✅ **Spell checking** - All code and docs
- ✅ **Auto-formatting** - `rustfmt` + `taplo`
- ✅ **Pre-push hooks** - Automated quality gates

All checks run automatically before push!

## 📖 Documentation

- [API Documentation](https://docs.rs/typed-money)
- [Functional Requirements](features/functional-requirements.md)
- [Non-Functional Requirements](features/non-functional-requirements.md)
- [Implementation Plan](features/IMPLEMENTATION_PLAN.md)
- [Project Setup Guide](PROJECT_SETUP_GUIDE.md)

## 🗺️ Roadmap

- [x] Core type system with compile-time safety
- [x] Basic arithmetic operations
- [x] Currency conversion with explicit rates
- [x] Rounding modes
- [ ] Serde serialization support
- [ ] BigDecimal alternative backend
- [ ] Property-based testing
- [ ] Benchmarks suite
- [ ] More built-in currencies
- [ ] Locale-aware formatting
- [ ] Currency symbol parsing

## 🤝 Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feat/amazing-feature`)
3. Make your changes
4. Ensure all quality checks pass (`make quality`)
5. Write tests for new functionality
6. Commit using [conventional commits](https://www.conventionalcommits.org/)
7. Push to your fork
8. Open a Pull Request

### Commit Message Format

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`

Examples:
- `feat(conversion): add support for indirect rate chains`
- `fix(rounding): correct banker's rounding implementation`
- `docs(readme): add e-commerce use case example`

## 📄 License

Dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## 🙏 Acknowledgments

Inspired by:
- [Joda-Money](https://www.joda.org/joda-money/) (Java)
- [Money](https://github.com/RubyMoney/money) (Ruby)
- [Dinero.js](https://dinerojs.com/) (JavaScript)
- [rust_decimal](https://github.com/paupino/rust-decimal) (Rust decimal arithmetic)

## 📞 Support

- 📫 [Issues](https://github.com/ricardoferreirades/typed-money/issues)
- 💬 [Discussions](https://github.com/ricardoferreirades/typed-money/discussions)
- 🐦 Twitter: [@ricardoferreirades](https://twitter.com/ricardoferreirades)

---

**Status:** 🚧 In Development | Not yet production-ready

Made with ❤️ by the Rust community
