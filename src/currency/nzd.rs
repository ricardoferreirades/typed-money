use super::{CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};
use crate::Currency;

/// New Zealand Dollar (NZD)
///
/// The New Zealand dollar is the official currency and legal tender of New Zealand,
/// the Cook Islands, Niue, the Ross Dependency, Tokelau, and a British territory,
/// the Pitcairn Islands.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, NZD};
///
/// let amount = Amount::<NZD>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(NZD::CODE, "NZD");
/// assert_eq!(NZD::SYMBOL, "NZ$");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NZD;

impl Currency for NZD {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "NZD";
    const SYMBOL: &'static str = "NZ$";

    // Rich metadata
    const NAME: &'static str = "New Zealand Dollar";
    const COUNTRY: &'static str = "New Zealand";
    const REGION: &'static str = "Oceania";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = true;
    const IS_STABLE: bool = true;
    const INTRODUCED_YEAR: u16 = 1967;
    const ISO_4217_NUMBER: u16 = 554;
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
    use crate::Amount;

    #[test]
    fn test_nzd_currency_properties() {
        assert_eq!(NZD::DECIMALS, 2);
        assert_eq!(NZD::CODE, "NZD");
        assert_eq!(NZD::SYMBOL, "NZ$");
    }

    #[test]
    fn test_nzd_amount_creation() {
        let amount = Amount::<NZD>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_nzd_amount_with_cents() {
        let amount = Amount::<NZD>::from_minor(10050); // 100.50 NZD
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
