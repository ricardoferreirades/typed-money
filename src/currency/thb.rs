use super::{CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};
use crate::Currency;

/// Thai Baht (THB)
///
/// The Thai baht is the currency of Thailand.
/// It is subdivided into 100 satang.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, THB};
///
/// let amount = Amount::<THB>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(THB::CODE, "THB");
/// assert_eq!(THB::SYMBOL, "฿");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct THB;

impl Currency for THB {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "THB";
    const SYMBOL: &'static str = "฿";

    // Rich metadata
    const NAME: &'static str = "Thai Baht";
    const COUNTRY: &'static str = "Thailand";
    const REGION: &'static str = "Asia";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 1897;
    const ISO_4217_NUMBER: u16 = 764;
    const THOUSANDS_SEPARATOR: char = ',';
    const DECIMAL_SEPARATOR: char = '.';
    const SYMBOL_POSITION: SymbolPosition = SymbolPosition::Before;
    const SPACE_BETWEEN: bool = false;
    const VOLATILITY_RATING: VolatilityRating = VolatilityRating::Medium;
    const LIQUIDITY_RATING: LiquidityRating = LiquidityRating::Medium;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_thb_currency_properties() {
        assert_eq!(THB::DECIMALS, 2);
        assert_eq!(THB::CODE, "THB");
        assert_eq!(THB::SYMBOL, "฿");
    }

    #[test]
    fn test_thb_amount_creation() {
        let amount = Amount::<THB>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_thb_amount_with_satang() {
        let amount = Amount::<THB>::from_minor(10050); // 100.50 THB
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
