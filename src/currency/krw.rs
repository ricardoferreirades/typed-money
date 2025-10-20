use crate::Currency;
use super::{CurrencyType, SymbolPosition, VolatilityRating, LiquidityRating};

/// South Korean Won (KRW)
///
/// The South Korean won is the official currency of South Korea.
/// A single won is divided into 100 jeon, the monetary subunit.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, KRW};
///
/// let amount = Amount::<KRW>::from_major(1000);
/// assert_eq!(amount.to_major_floor(), 1000);
/// assert_eq!(KRW::CODE, "KRW");
/// assert_eq!(KRW::SYMBOL, "₩");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct KRW;

impl Currency for KRW {
    const DECIMALS: u8 = 0; // Korean Won typically doesn't use decimal places
    const CODE: &'static str = "KRW";
    const SYMBOL: &'static str = "₩";
    
    // Rich metadata
    const NAME: &'static str = "South Korean Won";
    const COUNTRY: &'static str = "South Korea";
    const REGION: &'static str = "Asia";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 1962;
    const ISO_4217_NUMBER: u16 = 410;
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
    fn test_krw_currency_properties() {
        assert_eq!(KRW::DECIMALS, 0);
        assert_eq!(KRW::CODE, "KRW");
        assert_eq!(KRW::SYMBOL, "₩");
    }

    #[test]
    fn test_krw_amount_creation() {
        let amount = Amount::<KRW>::from_major(1000);
        assert_eq!(amount.to_major_floor(), 1000);
    }

    #[test]
    fn test_krw_amount_with_minor() {
        let amount = Amount::<KRW>::from_minor(1000); // 1000 KRW (no decimals)
        assert_eq!(amount.to_major_floor(), 1000);
        assert_eq!(amount.to_minor(), 1000);
    }
}
