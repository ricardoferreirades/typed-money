use crate::Currency;
use super::{CurrencyType, SymbolPosition, VolatilityRating, LiquidityRating};

/// Romanian Leu (RON)
///
/// The Romanian leu is the currency of Romania.
/// It is subdivided into 100 bani.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, RON};
///
/// let amount = Amount::<RON>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(RON::CODE, "RON");
/// assert_eq!(RON::SYMBOL, "lei");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RON;

impl Currency for RON {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "RON";
    const SYMBOL: &'static str = "lei";
    
    // Rich metadata
    const NAME: &'static str = "Romanian Leu";
    const COUNTRY: &'static str = "Romania";
    const REGION: &'static str = "Europe";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 2005;
    const ISO_4217_NUMBER: u16 = 946;
    const THOUSANDS_SEPARATOR: char = '.';
    const DECIMAL_SEPARATOR: char = ',';
    const SYMBOL_POSITION: SymbolPosition = SymbolPosition::After;
    const SPACE_BETWEEN: bool = true;
    const VOLATILITY_RATING: VolatilityRating = VolatilityRating::Medium;
    const LIQUIDITY_RATING: LiquidityRating = LiquidityRating::Low;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_ron_currency_properties() {
        assert_eq!(RON::DECIMALS, 2);
        assert_eq!(RON::CODE, "RON");
        assert_eq!(RON::SYMBOL, "lei");
    }

    #[test]
    fn test_ron_amount_creation() {
        let amount = Amount::<RON>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_ron_amount_with_bani() {
        let amount = Amount::<RON>::from_minor(10050); // 100.50 RON
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
