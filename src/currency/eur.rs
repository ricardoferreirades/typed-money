//! Euro currency implementation.

use super::{Currency, CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};

/// Euro
///
/// # Example
///
/// ```
/// use typed_money::{Amount, EUR};
///
/// let amount = Amount::<EUR>::from_major(100);
/// println!("{}", amount);  // Displays: €100.00 EUR
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EUR;

impl Currency for EUR {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "EUR";
    const SYMBOL: &'static str = "€";

    // Rich metadata
    const NAME: &'static str = "Euro";
    const COUNTRY: &'static str = "European Union";
    const REGION: &'static str = "Europe";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = true;
    const IS_STABLE: bool = true;
    const INTRODUCED_YEAR: u16 = 1999;
    const ISO_4217_NUMBER: u16 = 978;
    const THOUSANDS_SEPARATOR: char = '.';
    const DECIMAL_SEPARATOR: char = ',';
    const SYMBOL_POSITION: SymbolPosition = SymbolPosition::After;
    const SPACE_BETWEEN: bool = true;
    const VOLATILITY_RATING: VolatilityRating = VolatilityRating::Low;
    const LIQUIDITY_RATING: LiquidityRating = LiquidityRating::High;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eur_constants() {
        assert_eq!(EUR::DECIMALS, 2);
        assert_eq!(EUR::CODE, "EUR");
        assert_eq!(EUR::SYMBOL, "€");
    }
}
