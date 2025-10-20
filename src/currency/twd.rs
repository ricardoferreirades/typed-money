use super::{CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};
use crate::Currency;

/// New Taiwan Dollar (TWD)
///
/// The New Taiwan dollar is the official currency of Taiwan.
/// It is subdivided into 100 cents.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, TWD};
///
/// let amount = Amount::<TWD>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(TWD::CODE, "TWD");
/// assert_eq!(TWD::SYMBOL, "NT$");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TWD;

impl Currency for TWD {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "TWD";
    const SYMBOL: &'static str = "NT$";

    // Rich metadata
    const NAME: &'static str = "New Taiwan Dollar";
    const COUNTRY: &'static str = "Taiwan";
    const REGION: &'static str = "Asia";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 1949;
    const ISO_4217_NUMBER: u16 = 901;
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
    fn test_twd_currency_properties() {
        assert_eq!(TWD::DECIMALS, 2);
        assert_eq!(TWD::CODE, "TWD");
        assert_eq!(TWD::SYMBOL, "NT$");
    }

    #[test]
    fn test_twd_amount_creation() {
        let amount = Amount::<TWD>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_twd_amount_with_cents() {
        let amount = Amount::<TWD>::from_minor(10050); // 100.50 TWD
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
