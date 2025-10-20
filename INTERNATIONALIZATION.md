# Internationalization Guide

The typed-money library provides comprehensive internationalization (i18n) support with locale-specific formatting, rich metadata, and portfolio analysis capabilities.

## Overview

All 69 supported currencies include:
- **Locale-specific formatting** rules
- **Rich metadata** (country, region, volatility, liquidity)
- **Trading characteristics** for portfolio analysis
- **Historical information** (introduction year, ISO standards)

## Locale-Specific Formatting

### Formatting Rules

Each currency defines locale-specific formatting rules:

```rust
use typed_money::{Amount, USD, EUR, BRL, CurrencyMetadata};

// US formatting (comma thousands, dot decimal)
let usd = Amount::<USD>::from_major(1234);
println!("{}", usd); // $1,234.00 USD

// European formatting (dot thousands, comma decimal)
let eur = Amount::<EUR>::from_major(1234);
println!("{}", eur); // €1.234,00 EUR

// Brazilian formatting (dot thousands, comma decimal)
let brl = Amount::<BRL>::from_major(1234);
println!("{}", brl); // R$1.234,00 BRL
```

### Formatting Components

| Component | Description | Examples |
|-----------|-------------|----------|
| **Thousands Separator** | Separates thousands | `,` (US), `.` (Europe) |
| **Decimal Separator** | Separates decimal part | `.` (US), `,` (Europe) |
| **Symbol Position** | Where currency symbol appears | Before (`$100`), After (`100€`) |
| **Space Between** | Space between symbol and amount | `$100` vs `$ 100` |

### Regional Formatting Styles

#### North American Style
- **Thousands**: Comma (`,`)
- **Decimal**: Dot (`.`)
- **Symbol**: Before amount
- **Space**: No space
- **Example**: `$1,234.56`

#### European Style
- **Thousands**: Dot (`.`)
- **Decimal**: Comma (`,`)
- **Symbol**: After amount
- **Space**: Space between
- **Example**: `1.234,56 €`

#### Asian Style
- **Thousands**: Comma (`,`)
- **Decimal**: Dot (`.`)
- **Symbol**: Before amount
- **Space**: No space
- **Example**: `¥1,234`

#### Middle Eastern Style
- **Thousands**: Comma (`,`)
- **Decimal**: Dot (`.`)
- **Symbol**: After amount
- **Space**: Space between
- **Example**: `1,234.56 د.إ`

## Currency Metadata

### Accessing Metadata

```rust
use typed_money::{Amount, USD, CurrencyMetadata};

let amount = Amount::<USD>::from_major(1000);

// Basic information
println!("Name: {}", amount.currency_name());        // "US Dollar"
println!("Country: {}", amount.currency_country());  // "United States"
println!("Region: {}", amount.currency_region());    // "North America"
println!("Type: {}", amount.currency_type());        // "Fiat"

// Trading characteristics
println!("Major: {}", amount.is_major_currency());   // true
println!("Stable: {}", amount.is_stable_currency()); // true
println!("Volatility: {}", amount.volatility_rating()); // "Low"
println!("Liquidity: {}", amount.liquidity_rating());   // "High"

// Historical information
println!("Introduced: {}", amount.currency_introduced_year()); // 1792
println!("ISO Number: {}", amount.currency_iso_number());      // 840

// Formatting rules
println!("Thousands: '{}'", amount.thousands_separator());     // ','
println!("Decimal: '{}'", amount.decimal_separator());         // '.'
println!("Symbol Position: {}", amount.symbol_position());     // "Before"
println!("Space Between: {}", amount.space_between_symbol());  // false
```

### Currency Types

```rust
use typed_money::{CurrencyType, VolatilityRating, LiquidityRating};

// Currency types
CurrencyType::Fiat           // Traditional government currencies
CurrencyType::Cryptocurrency // Digital currencies
CurrencyType::Commodity      // Precious metals, base metals

// Volatility ratings
VolatilityRating::Low        // Stable currencies (USD, EUR, GBP)
VolatilityRating::Medium     // Most fiat currencies
VolatilityRating::High       // Cryptocurrencies, emerging markets

// Liquidity ratings
LiquidityRating::Low         // Rarely traded currencies
LiquidityRating::Medium      // Regular trading
LiquidityRating::High        // Heavily traded, deep markets
```

## Portfolio Analysis

### Multi-Currency Portfolio

```rust
use typed_money::{Amount, USD, EUR, BTC, XAU, CurrencyMetadata};

let portfolio = vec![
    ("USD", Amount::<USD>::from_major(10000)),
    ("EUR", Amount::<EUR>::from_major(8500)),
    ("BTC", Amount::<BTC>::from_major(1)),
    ("XAU", Amount::<XAU>::from_major(10)),
];

let mut fiat_count = 0;
let mut crypto_count = 0;
let mut commodity_count = 0;
let mut major_count = 0;
let mut stable_count = 0;

for (name, amount) in &portfolio {
    println!("{}: {} - Type: {}, Volatility: {}, Liquidity: {}", 
        name, amount, 
        amount.currency_type(),
        amount.volatility_rating(),
        amount.liquidity_rating()
    );
    
    match amount.currency_type() {
        CurrencyType::Fiat => fiat_count += 1,
        CurrencyType::Cryptocurrency => crypto_count += 1,
        CurrencyType::Commodity => commodity_count += 1,
    }
    
    if amount.is_major_currency() { major_count += 1; }
    if amount.is_stable_currency() { stable_count += 1; }
}

println!("Portfolio Summary:");
println!("  Fiat Currencies: {}", fiat_count);
println!("  Cryptocurrencies: {}", crypto_count);
println!("  Commodities: {}", commodity_count);
println!("  Major Currencies: {}", major_count);
println!("  Stable Currencies: {}", stable_count);
```

### Regional Analysis

```rust
use std::collections::HashMap;

// Group currencies by region
let currencies = vec![
    ("USD", "North America"),
    ("EUR", "Europe"),
    ("JPY", "Asia"),
    ("BRL", "South America"),
    ("ZAR", "Africa"),
    ("XAU", "Worldwide"),
];

let mut regions: HashMap<String, Vec<&str>> = HashMap::new();

for (currency, region) in currencies {
    regions
        .entry(region.to_string())
        .or_default()
        .push(currency);
}

for (region, currencies) in regions {
    println!("{}: {}", region, currencies.join(", "));
}
```

### Trading Analysis

```rust
use typed_money::{Amount, USD, BTC, XAU, CurrencyMetadata};

fn analyze_trading_currency<C: Currency + CurrencyMetadata>(amount: &Amount<C>) {
    println!("Trading Analysis:");
    println!("  Currency: {} ({})", amount.currency_name(), C::CODE);
    println!("  Type: {}", amount.currency_type());
    println!("  Volatility: {}", amount.volatility_rating());
    println!("  Liquidity: {}", amount.liquidity_rating());
    println!("  Major Currency: {}", amount.is_major_currency());
    println!("  Stable Currency: {}", amount.is_stable_currency());
    
    let recommendation = match (amount.volatility_rating(), amount.liquidity_rating()) {
        (VolatilityRating::Low, LiquidityRating::High) => "Excellent for conservative trading",
        (VolatilityRating::Medium, LiquidityRating::High) => "Good for balanced trading",
        (VolatilityRating::High, LiquidityRating::High) => "Suitable for aggressive trading",
        (VolatilityRating::High, LiquidityRating::Medium) => "High risk, moderate liquidity",
        _ => "Consider carefully before trading",
    };
    
    println!("  Recommendation: {}", recommendation);
}

// Analyze different currency types
analyze_trading_currency(&Amount::<USD>::from_major(1000));
analyze_trading_currency(&Amount::<BTC>::from_major(1));
analyze_trading_currency(&Amount::<XAU>::from_major(10));
```

## Custom Formatting

### Locale-Specific Formatting Functions

```rust
use typed_money::{Amount, USD, CurrencyMetadata};

fn format_with_locale<C: Currency + CurrencyMetadata>(amount: &Amount<C>) -> String {
    let major = amount.to_major_floor();
    let minor = amount.to_minor() % 10_i64.pow(C::DECIMALS as u32) as u64;
    
    let formatted_major = if C::THOUSANDS_SEPARATOR == ',' {
        format_number_with_commas(major as u64)
    } else {
        format_number_with_dots(major as u64)
    };
    
    if C::DECIMALS == 0 {
        formatted_major
    } else {
        let minor_str = format!("{:0width$}", minor, width = C::DECIMALS as usize);
        format!("{}{}{}", formatted_major, C::DECIMAL_SEPARATOR, minor_str)
    }
}

fn format_number_with_commas(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    let mut count = 0;
    
    for ch in s.chars().rev() {
        if count > 0 && count % 3 == 0 {
            result.push(',');
        }
        result.push(ch);
        count += 1;
    }
    
    result.chars().rev().collect()
}

fn format_number_with_dots(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    let mut count = 0;
    
    for ch in s.chars().rev() {
        if count > 0 && count % 3 == 0 {
            result.push('.');
        }
        result.push(ch);
        count += 1;
    }
    
    result.chars().rev().collect()
}

// Usage
let usd_amount = Amount::<USD>::from_major(1234567);
println!("US Style: {}", format_with_locale(&usd_amount)); // "1,234,567.00"
```

## Examples

### Complete Internationalization Example

```rust
use typed_money::{
    Amount, USD, EUR, BRL, CNY, INR, XAU, BTC,
    CurrencyMetadata, CurrencyType, VolatilityRating, LiquidityRating
};

fn main() {
    println!("=== Internationalization Examples ===\n");
    
    // Demonstrate different formatting styles
    let amounts = vec![
        ("USD", Amount::<USD>::from_major(1234)),
        ("EUR", Amount::<EUR>::from_major(1234)),
        ("BRL", Amount::<BRL>::from_major(1234)),
        ("CNY", Amount::<CNY>::from_major(1234)),
        ("INR", Amount::<INR>::from_major(1234)),
    ];
    
    for (name, amount) in amounts {
        println!("{}: {} (Locale: {})", 
            name, 
            amount, 
            format_with_locale(&amount)
        );
    }
    
    // Portfolio analysis
    let portfolio = vec![
        ("USD", Amount::<USD>::from_major(10000)),
        ("EUR", Amount::<EUR>::from_major(8500)),
        ("BTC", Amount::<BTC>::from_major(1)),
        ("XAU", Amount::<XAU>::from_major(10)),
    ];
    
    analyze_portfolio(&portfolio);
}

fn analyze_portfolio(portfolio: &[(&str, impl CurrencyMetadata)]) {
    println!("\n=== Portfolio Analysis ===");
    
    let mut fiat_count = 0;
    let mut crypto_count = 0;
    let mut commodity_count = 0;
    
    for (name, amount) in portfolio {
        println!("{}: {} - Type: {}, Volatility: {}, Liquidity: {}", 
            name, amount, 
            amount.currency_type(),
            amount.volatility_rating(),
            amount.liquidity_rating()
        );
        
        match amount.currency_type() {
            CurrencyType::Fiat => fiat_count += 1,
            CurrencyType::Cryptocurrency => crypto_count += 1,
            CurrencyType::Commodity => commodity_count += 1,
        }
    }
    
    println!("\nPortfolio Summary:");
    println!("  Fiat Currencies: {}", fiat_count);
    println!("  Cryptocurrencies: {}", crypto_count);
    println!("  Commodities: {}", commodity_count);
}
```

## Best Practices

1. **Use locale-specific formatting** for user-facing displays
2. **Leverage metadata** for portfolio analysis and risk assessment
3. **Group currencies by region** for geographic analysis
4. **Consider volatility and liquidity** for trading decisions
5. **Use type-safe operations** to prevent currency mixing bugs

## Migration from v0.1.x

The internationalization features are fully backward compatible. Existing code will continue to work, and you can gradually adopt the new metadata features:

```rust
// Old way (still works)
let amount = Amount::<USD>::from_major(100);
println!("{}", amount); // $100.00 USD

// New way (with metadata)
use typed_money::CurrencyMetadata;
println!("Currency: {}", amount.currency_name()); // "US Dollar"
println!("Region: {}", amount.currency_region()); // "North America"
```
