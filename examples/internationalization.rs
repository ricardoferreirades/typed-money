//! Example demonstrating internationalization and localization features.

use typed_money::{
    Amount, Currency, CurrencyMetadata, CurrencyType, LiquidityRating, SymbolPosition,
    VolatilityRating, BRL, BTC, CNY, EUR, GBP, INR, JPY, USD, XAU, ZAR,
};

fn main() {
    println!("=== Internationalization Examples ===\n");

    // Create amounts in different currencies
    let usd_amount = Amount::<USD>::from_major(1234);
    let eur_amount = Amount::<EUR>::from_major(1234);
    let gbp_amount = Amount::<GBP>::from_major(1234);
    let jpy_amount = Amount::<JPY>::from_major(1234);
    let cny_amount = Amount::<CNY>::from_major(1234);
    let brl_amount = Amount::<BRL>::from_major(1234);
    let inr_amount = Amount::<INR>::from_major(1234);
    let zar_amount = Amount::<ZAR>::from_major(1234);
    let gold_amount = Amount::<XAU>::from_major(1);

    // Demonstrate locale-specific formatting
    demonstrate_locale_formatting("US Formatting", usd_amount);
    demonstrate_locale_formatting("European Formatting", eur_amount);
    demonstrate_locale_formatting("British Formatting", gbp_amount);
    demonstrate_locale_formatting("Japanese Formatting", jpy_amount);
    demonstrate_locale_formatting("Chinese Formatting", cny_amount);
    demonstrate_locale_formatting("Brazilian Formatting", brl_amount);
    demonstrate_locale_formatting("Indian Formatting", inr_amount);
    demonstrate_locale_formatting("South African Formatting", zar_amount);
    demonstrate_locale_formatting("Gold Trading Format", gold_amount);

    println!("\n=== Multi-Currency Portfolio ===\n");

    // Demonstrate multi-currency portfolio with internationalization
    analyze_international_portfolio();

    println!("\n=== Currency Localization by Region ===\n");

    // Group currencies by region
    group_currencies_by_region();

    println!("\n=== Trading Hours and Market Analysis ===\n");

    // Demonstrate market analysis using currency metadata
    analyze_trading_markets();

    println!("\n=== Custom Locale Formatting ===\n");

    // Demonstrate custom locale formatting
    demonstrate_custom_formatting();
}

fn demonstrate_locale_formatting<C: Currency>(title: &str, amount: Amount<C>) {
    println!("=== {} ===", title);
    println!("Standard: {}", amount);
    println!("Currency: {} ({})", amount.currency_name(), C::CODE);
    println!("Country: {}", amount.currency_country());
    println!("Region: {}", amount.currency_region());
    println!("Type: {}", amount.currency_type());

    // Show locale-specific formatting information
    println!("Formatting Rules:");
    println!("  Thousands Separator: '{}'", amount.thousands_separator());
    println!("  Decimal Separator: '{}'", amount.decimal_separator());
    println!("  Symbol Position: {}", amount.symbol_position());
    println!("  Space Between Symbol: {}", amount.space_between_symbol());

    // Demonstrate formatted output
    let formatted = format_locale_specific(amount);
    println!("  Locale Formatted: {}", formatted);
    println!();
}

fn format_locale_specific<C: Currency>(amount: Amount<C>) -> String {
    let major = amount.to_major_floor();
    let minor = (amount.to_minor() % 10_i64.pow(C::DECIMALS as u32)) as u64;

    // Format with locale-specific separators
    let formatted_major = format_with_separators(major, C::THOUSANDS_SEPARATOR);
    let formatted_minor = if C::DECIMALS > 0 {
        format!("{:0width$}", minor, width = C::DECIMALS as usize)
    } else {
        String::new()
    };

    // Build the formatted string based on symbol position
    let amount_str = if C::DECIMALS > 0 {
        format!(
            "{}{}{}",
            formatted_major,
            C::DECIMAL_SEPARATOR,
            formatted_minor
        )
    } else {
        formatted_major
    };

    match C::SYMBOL_POSITION {
        SymbolPosition::Before => {
            if C::SPACE_BETWEEN {
                format!("{} {}", C::SYMBOL, amount_str)
            } else {
                format!("{}{}", C::SYMBOL, amount_str)
            }
        }
        SymbolPosition::After => {
            if C::SPACE_BETWEEN {
                format!("{} {}", amount_str, C::SYMBOL)
            } else {
                format!("{}{}", amount_str, C::SYMBOL)
            }
        }
    }
}

fn format_with_separators(mut number: i64, separator: char) -> String {
    if number == 0 {
        return "0".to_string();
    }

    let mut result = String::new();
    let mut count = 0;
    let is_negative = number < 0;

    if is_negative {
        number = -number;
    }

    while number > 0 {
        if count > 0 && count % 3 == 0 {
            result.insert(0, separator);
        }
        result.insert(0, ((number % 10) as u8 + b'0') as char);
        number /= 10;
        count += 1;
    }

    if is_negative {
        result.insert(0, '-');
    }

    result
}

fn analyze_international_portfolio() {
    println!("Portfolio Analysis by Currency Type:");

    // Analyze each currency type individually
    analyze_currency_type("USD", USD::CURRENCY_TYPE, USD::IS_MAJOR, USD::IS_STABLE);
    analyze_currency_type("EUR", EUR::CURRENCY_TYPE, EUR::IS_MAJOR, EUR::IS_STABLE);
    analyze_currency_type("GBP", GBP::CURRENCY_TYPE, GBP::IS_MAJOR, GBP::IS_STABLE);
    analyze_currency_type("JPY", JPY::CURRENCY_TYPE, JPY::IS_MAJOR, JPY::IS_STABLE);
    analyze_currency_type("CNY", CNY::CURRENCY_TYPE, CNY::IS_MAJOR, CNY::IS_STABLE);
    analyze_currency_type("BRL", BRL::CURRENCY_TYPE, BRL::IS_MAJOR, BRL::IS_STABLE);
    analyze_currency_type("INR", INR::CURRENCY_TYPE, INR::IS_MAJOR, INR::IS_STABLE);
    analyze_currency_type("ZAR", ZAR::CURRENCY_TYPE, ZAR::IS_MAJOR, ZAR::IS_STABLE);
    analyze_currency_type("XAU", XAU::CURRENCY_TYPE, XAU::IS_MAJOR, XAU::IS_STABLE);

    println!("\nPortfolio Summary:");
    println!("  Fiat Currencies: 8");
    println!("  Cryptocurrencies: 0");
    println!("  Commodities: 1");
    println!("  Major Currencies: 8");
    println!("  Stable Currencies: 8");
}

fn analyze_currency_type(name: &str, currency_type: CurrencyType, is_major: bool, is_stable: bool) {
    println!(
        "{}: {} - Major: {}, Stable: {}",
        name, currency_type, is_major, is_stable
    );
}

fn group_currencies_by_region() {
    let currencies = vec![
        ("USD", "North America"),
        ("EUR", "Europe"),
        ("GBP", "Europe"),
        ("JPY", "Asia"),
        ("CNY", "Asia"),
        ("BRL", "South America"),
        ("INR", "Asia"),
        ("ZAR", "Africa"),
        ("XAU", "Global"),
    ];

    let mut regions: std::collections::HashMap<String, Vec<&str>> =
        std::collections::HashMap::new();

    for (currency, region) in currencies {
        regions
            .entry(region.to_string())
            .or_default()
            .push(currency);
    }

    for (region, currencies) in regions {
        println!("{}: {}", region, currencies.join(", "));
    }
}

fn analyze_trading_markets() {
    println!("Market Analysis by Currency Type:");

    let usd = Amount::<USD>::from_major(100);
    let btc = Amount::<BTC>::from_major(1);
    let gold = Amount::<XAU>::from_major(1);

    analyze_currency_for_trading("USD", usd);
    analyze_currency_for_trading("BTC", btc);
    analyze_currency_for_trading("XAU", gold);
}

fn analyze_currency_for_trading<C: Currency>(name: &str, amount: Amount<C>) {
    println!("{} Trading Analysis:", name);
    println!("  Currency: {} ({})", amount.currency_name(), C::CODE);
    println!("  Type: {}", amount.currency_type());
    println!("  Volatility: {}", amount.volatility_rating());
    println!("  Liquidity: {}", amount.liquidity_rating());
    println!("  Major Currency: {}", amount.is_major_currency());
    println!("  Stable Currency: {}", amount.is_stable_currency());

    // Trading recommendations based on metadata
    let recommendation = match (
        amount.volatility_rating(),
        amount.liquidity_rating(),
        amount.is_stable_currency(),
    ) {
        (VolatilityRating::Low, LiquidityRating::High, true) => {
            "Excellent for conservative trading"
        }
        (VolatilityRating::Medium, LiquidityRating::High, false) => "Good for balanced trading",
        (VolatilityRating::High, LiquidityRating::High, false) => "Suitable for aggressive trading",
        (VolatilityRating::High, LiquidityRating::Low, false) => "High risk - use with caution",
        _ => "Consider market conditions carefully",
    };

    println!("  Recommendation: {}", recommendation);
    println!();
}

fn demonstrate_custom_formatting() {
    let usd_amount = Amount::<USD>::from_major(1234567);

    println!("Custom Formatting Examples:");
    println!("Original: {}", usd_amount);

    // Different locale styles
    println!("US Style: {}", format_locale_specific(usd_amount));

    // Simulate European formatting
    println!("European Style: {}", format_european_style(usd_amount));

    // Simulate Asian formatting
    println!("Asian Style: {}", format_asian_style(usd_amount));
}

fn format_european_style<C: Currency>(amount: Amount<C>) -> String {
    let major = amount.to_major_floor();
    let minor = (amount.to_minor() % 10_i64.pow(C::DECIMALS as u32)) as u64;

    // European style: dot for thousands, comma for decimal
    let formatted_major = format_with_separators(major, '.');
    let formatted_minor = if C::DECIMALS > 0 {
        format!("{:0width$}", minor, width = C::DECIMALS as usize)
    } else {
        String::new()
    };

    let amount_str = if C::DECIMALS > 0 {
        format!("{},{}", formatted_major, formatted_minor)
    } else {
        formatted_major
    };

    format!("{} {}", amount_str, C::SYMBOL)
}

fn format_asian_style<C: Currency>(amount: Amount<C>) -> String {
    let major = amount.to_major_floor();
    let minor = (amount.to_minor() % 10_i64.pow(C::DECIMALS as u32)) as u64;

    // Asian style: comma for thousands, dot for decimal
    let formatted_major = format_with_separators(major, ',');
    let formatted_minor = if C::DECIMALS > 0 {
        format!("{:0width$}", minor, width = C::DECIMALS as usize)
    } else {
        String::new()
    };

    let amount_str = if C::DECIMALS > 0 {
        format!("{}.{}", formatted_major, formatted_minor)
    } else {
        formatted_major
    };

    format!("{}{}", C::SYMBOL, amount_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_locale_formatting() {
        let usd_amount = Amount::<USD>::from_major(1234);
        let formatted = format_locale_specific(usd_amount);

        // Should format with US-style separators
        assert!(formatted.contains("$"));
        assert!(formatted.contains("1,234"));
    }

    #[test]
    fn test_european_formatting() {
        let eur_amount = Amount::<EUR>::from_major(1234);
        let formatted = format_european_style(eur_amount);

        // Should format with European-style separators
        assert!(formatted.contains("€"));
        assert!(formatted.contains("1.234"));
    }

    #[test]
    fn test_asian_formatting() {
        let jpy_amount = Amount::<JPY>::from_major(1234);
        let formatted = format_asian_style(jpy_amount);

        // Should format with Asian-style separators
        assert!(formatted.contains("¥"));
        assert!(formatted.contains("1,234"));
    }
}
