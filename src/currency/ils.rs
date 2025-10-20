use crate::Currency;
use super::{CurrencyType, SymbolPosition, VolatilityRating, LiquidityRating};

/// Israeli Shekel (ILS)
///
/// The Israeli new shekel is the currency of Israel.
/// It is subdivided into 100 agorot.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, ILS};
///
/// let amount = Amount::<ILS>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(ILS::CODE, "ILS");
/// assert_eq!(ILS::SYMBOL, "₪");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ILS;

impl Currency for ILS {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "ILS";
    const SYMBOL: &'static str = "₪";
    
    // Rich metadata
    const NAME: &'static str = "Israeli Shekel";
    const COUNTRY: &'static str = "Israel";
    const REGION: &'static str = "Middle East";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 1985;
    const ISO_4217_NUMBER: u16 = 376;
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
    fn test_ils_currency_properties() {
        assert_eq!(ILS::DECIMALS, 2);
        assert_eq!(ILS::CODE, "ILS");
        assert_eq!(ILS::SYMBOL, "₪");
    }

    #[test]
    fn test_ils_amount_creation() {
        let amount = Amount::<ILS>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_ils_amount_with_agorot() {
        let amount = Amount::<ILS>::from_minor(10050); // 100.50 ILS
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
