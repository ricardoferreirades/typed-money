//! Bitcoin currency implementation.

use super::{Currency, CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};

/// Bitcoin
///
/// # Example
///
/// ```
/// use typed_money::{Amount, BTC};
///
/// let amount = Amount::<BTC>::from_major(1);
/// println!("{}", amount);  // Displays: ₿1.00000000 BTC
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BTC;

impl Currency for BTC {
    const DECIMALS: u8 = 8;
    const CODE: &'static str = "BTC";
    const SYMBOL: &'static str = "₿";

    // Cryptocurrency metadata
    const NAME: &'static str = "Bitcoin";
    const COUNTRY: &'static str = "Global";
    const REGION: &'static str = "Worldwide";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Cryptocurrency;
    const IS_MAJOR: bool = true;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 2009;
    const ISO_4217_NUMBER: u16 = 0; // Not officially assigned
    const THOUSANDS_SEPARATOR: char = ',';
    const DECIMAL_SEPARATOR: char = '.';
    const SYMBOL_POSITION: SymbolPosition = SymbolPosition::Before;
    const SPACE_BETWEEN: bool = false;
    const VOLATILITY_RATING: VolatilityRating = VolatilityRating::High;
    const LIQUIDITY_RATING: LiquidityRating = LiquidityRating::High;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_btc_constants() {
        assert_eq!(BTC::DECIMALS, 8);
        assert_eq!(BTC::CODE, "BTC");
        assert_eq!(BTC::SYMBOL, "₿");
    }
}
