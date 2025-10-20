//! Japanese Yen currency implementation.

use super::{Currency, CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};

/// Japanese Yen
///
/// # Example
///
/// ```
/// use typed_money::{Amount, JPY};
///
/// let amount = Amount::<JPY>::from_major(1000);
/// println!("{}", amount);  // Displays: ¥1000 JPY
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct JPY;

impl Currency for JPY {
    const DECIMALS: u8 = 0;
    const CODE: &'static str = "JPY";
    const SYMBOL: &'static str = "¥";

    // Rich metadata
    const NAME: &'static str = "Japanese Yen";
    const COUNTRY: &'static str = "Japan";
    const REGION: &'static str = "Asia";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = true;
    const IS_STABLE: bool = true;
    const INTRODUCED_YEAR: u16 = 1871;
    const ISO_4217_NUMBER: u16 = 392;
    const THOUSANDS_SEPARATOR: char = ',';
    const DECIMAL_SEPARATOR: char = '.';
    const SYMBOL_POSITION: SymbolPosition = SymbolPosition::Before;
    const SPACE_BETWEEN: bool = false;
    const VOLATILITY_RATING: VolatilityRating = VolatilityRating::Low;
    const LIQUIDITY_RATING: LiquidityRating = LiquidityRating::High;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jpy_constants() {
        assert_eq!(JPY::DECIMALS, 0);
        assert_eq!(JPY::CODE, "JPY");
        assert_eq!(JPY::SYMBOL, "¥");
    }
}
