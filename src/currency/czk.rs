use super::{CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};
use crate::Currency;

/// Czech Koruna (CZK)
///
/// The Czech koruna is the currency of the Czech Republic.
/// It is subdivided into 100 haléřů.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, CZK};
///
/// let amount = Amount::<CZK>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(CZK::CODE, "CZK");
/// assert_eq!(CZK::SYMBOL, "Kč");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CZK;

impl Currency for CZK {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "CZK";
    const SYMBOL: &'static str = "Kč";

    // Rich metadata
    const NAME: &'static str = "Czech Koruna";
    const COUNTRY: &'static str = "Czech Republic";
    const REGION: &'static str = "Europe";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 1993;
    const ISO_4217_NUMBER: u16 = 203;
    const THOUSANDS_SEPARATOR: char = ' ';
    const DECIMAL_SEPARATOR: char = ',';
    const SYMBOL_POSITION: SymbolPosition = SymbolPosition::After;
    const SPACE_BETWEEN: bool = true;
    const VOLATILITY_RATING: VolatilityRating = VolatilityRating::Medium;
    const LIQUIDITY_RATING: LiquidityRating = LiquidityRating::Medium;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_czk_currency_properties() {
        assert_eq!(CZK::DECIMALS, 2);
        assert_eq!(CZK::CODE, "CZK");
        assert_eq!(CZK::SYMBOL, "Kč");
    }

    #[test]
    fn test_czk_amount_creation() {
        let amount = Amount::<CZK>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_czk_amount_with_haléřů() {
        let amount = Amount::<CZK>::from_minor(10050); // 100.50 CZK
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
