//! British Pound Sterling currency implementation.

use super::{Currency, CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};

/// British Pound Sterling
///
/// # Example
///
/// ```
/// use typed_money::{Amount, GBP};
///
/// let amount = Amount::<GBP>::from_major(100);
/// println!("{}", amount);  // Displays: £100.00 GBP
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GBP;

impl Currency for GBP {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "GBP";
    const SYMBOL: &'static str = "£";

    // Rich metadata
    const NAME: &'static str = "British Pound Sterling";
    const COUNTRY: &'static str = "United Kingdom";
    const REGION: &'static str = "Europe";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = true;
    const IS_STABLE: bool = true;
    const INTRODUCED_YEAR: u16 = 1694;
    const ISO_4217_NUMBER: u16 = 826;
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
    fn test_gbp_constants() {
        assert_eq!(GBP::DECIMALS, 2);
        assert_eq!(GBP::CODE, "GBP");
        assert_eq!(GBP::SYMBOL, "£");
    }
}
