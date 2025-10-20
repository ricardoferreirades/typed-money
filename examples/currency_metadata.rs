//! Example demonstrating currency metadata functionality.

use typed_money::{
    Amount, Currency, CurrencyMetadata, CurrencyType, LiquidityRating, SymbolPosition,
    VolatilityRating, BTC, USD, XAU,
};

fn main() {
    println!("=== Currency Metadata Examples ===\n");

    // Create amounts in different currencies
    let usd_amount = Amount::<USD>::from_major(100);
    let btc_amount = Amount::<BTC>::from_major(1);
    let gold_amount = Amount::<XAU>::from_major(1);

    // Demonstrate metadata access
    demonstrate_currency_metadata("USD (Fiat)", usd_amount);
    demonstrate_currency_metadata("BTC (Cryptocurrency)", btc_amount);
    demonstrate_currency_metadata("XAU (Commodity)", gold_amount);

    println!("\n=== Currency Comparison ===\n");

    // Compare different currency types
    println!("USD Analysis:");
    analyze_currency("USD", usd_amount);

    println!("BTC Analysis:");
    analyze_currency("BTC", btc_amount);

    println!("XAU Analysis:");
    analyze_currency("XAU", gold_amount);

    println!("=== Formatting Information ===\n");

    // Show formatting metadata
    println!("USD Formatting:");
    show_formatting("USD", usd_amount);

    println!("BTC Formatting:");
    show_formatting("BTC", btc_amount);

    println!("XAU Formatting:");
    show_formatting("XAU", gold_amount);

    println!("=== Currency Information Summary ===\n");

    // Show complete currency info
    println!("USD: {}", usd_amount.currency_info());
    println!("BTC: {}", btc_amount.currency_info());
    println!("XAU: {}", gold_amount.currency_info());
}

fn demonstrate_currency_metadata<C: Currency>(currency_name: &str, amount: Amount<C>) {
    println!("=== {} ===", currency_name);
    println!("Amount: {}", amount);
    println!("Currency Name: {}", amount.currency_name());
    println!("Country: {}", amount.currency_country());
    println!("Region: {}", amount.currency_region());
    println!("Type: {}", amount.currency_type());
    println!("Is Major: {}", amount.is_major_currency());
    println!("Is Stable: {}", amount.is_stable_currency());
    println!("ISO Number: {}", amount.currency_iso_number());
    println!("Volatility: {}", amount.volatility_rating());
    println!("Liquidity: {}", amount.liquidity_rating());
    println!();
}

fn analyze_currency<C: Currency>(_name: &str, amount: Amount<C>) {
    println!("  Type: {}", amount.currency_type());
    println!("  Major Currency: {}", amount.is_major_currency());
    println!("  Stable Currency: {}", amount.is_stable_currency());
    println!("  Volatility: {}", amount.volatility_rating());
    println!("  Liquidity: {}", amount.liquidity_rating());
    println!(
        "  Introduced: {}",
        if amount.currency_introduced_year() == 0 {
            "Ancient times".to_string()
        } else {
            amount.currency_introduced_year().to_string()
        }
    );
    println!();
}

fn show_formatting<C: Currency>(_name: &str, amount: Amount<C>) {
    println!("  Thousands Separator: '{}'", amount.thousands_separator());
    println!("  Decimal Separator: '{}'", amount.decimal_separator());
    println!("  Symbol Position: {}", amount.symbol_position());
    println!("  Space Between Symbol: {}", amount.space_between_symbol());
    println!();
}

// Example of creating a custom currency with metadata
#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
struct CustomCurrency;

impl Currency for CustomCurrency {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "CUSTOM";
    const SYMBOL: &'static str = "C";

    // Provide rich metadata
    const NAME: &'static str = "Custom Currency";
    const COUNTRY: &'static str = "Custom Country";
    const REGION: &'static str = "Custom Region";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = true;
    const INTRODUCED_YEAR: u16 = 2024;
    const ISO_4217_NUMBER: u16 = 999;
    const THOUSANDS_SEPARATOR: char = '.';
    const DECIMAL_SEPARATOR: char = ',';
    const SYMBOL_POSITION: SymbolPosition = SymbolPosition::After;
    const SPACE_BETWEEN: bool = true;
    const VOLATILITY_RATING: VolatilityRating = VolatilityRating::Low;
    const LIQUIDITY_RATING: LiquidityRating = LiquidityRating::Medium;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_currency_metadata() {
        let amount = Amount::<CustomCurrency>::from_major(100);

        assert_eq!(amount.currency_name(), "Custom Currency");
        assert_eq!(amount.currency_country(), "Custom Country");
        assert_eq!(amount.currency_type(), CurrencyType::Fiat);
        assert_eq!(amount.is_major_currency(), false);
        assert_eq!(amount.is_stable_currency(), true);
        assert_eq!(amount.symbol_position(), SymbolPosition::After);
        assert_eq!(amount.space_between_symbol(), true);
    }
}
