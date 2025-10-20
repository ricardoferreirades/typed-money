use super::{CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};
use crate::Currency;

/// Kuwaiti Dinar (KWD)
///
/// The Kuwaiti dinar is the currency of Kuwait.
/// It is subdivided into 1000 fils.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, KWD};
///
/// let amount = Amount::<KWD>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(KWD::CODE, "KWD");
/// assert_eq!(KWD::SYMBOL, "د.ك");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct KWD;

impl Currency for KWD {
    const DECIMALS: u8 = 3; // Kuwaiti Dinar uses 3 decimal places (fils)
    const CODE: &'static str = "KWD";
    const SYMBOL: &'static str = "د.ك";

    // Rich metadata
    const NAME: &'static str = "Kuwaiti Dinar";
    const COUNTRY: &'static str = "Kuwait";
    const REGION: &'static str = "Middle East";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = true;
    const INTRODUCED_YEAR: u16 = 1961;
    const ISO_4217_NUMBER: u16 = 414;
    const THOUSANDS_SEPARATOR: char = ',';
    const DECIMAL_SEPARATOR: char = '.';
    const SYMBOL_POSITION: SymbolPosition = SymbolPosition::After;
    const SPACE_BETWEEN: bool = true;
    const VOLATILITY_RATING: VolatilityRating = VolatilityRating::Low;
    const LIQUIDITY_RATING: LiquidityRating = LiquidityRating::Medium;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_kwd_currency_properties() {
        assert_eq!(KWD::DECIMALS, 3);
        assert_eq!(KWD::CODE, "KWD");
        assert_eq!(KWD::SYMBOL, "د.ك");
    }

    #[test]
    fn test_kwd_amount_creation() {
        let amount = Amount::<KWD>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_kwd_amount_with_fils() {
        let amount = Amount::<KWD>::from_minor(100500); // 100.500 KWD
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 100500);
    }
}
