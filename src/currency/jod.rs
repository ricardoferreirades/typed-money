use crate::Currency;
use super::{CurrencyType, SymbolPosition, VolatilityRating, LiquidityRating};

/// Jordanian Dinar (JOD)
///
/// The Jordanian dinar is the currency of Jordan.
/// It is subdivided into 1000 fils.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, JOD};
///
/// let amount = Amount::<JOD>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(JOD::CODE, "JOD");
/// assert_eq!(JOD::SYMBOL, "د.ا");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct JOD;

impl Currency for JOD {
    const DECIMALS: u8 = 3; // Jordanian Dinar uses 3 decimal places (fils)
    const CODE: &'static str = "JOD";
    const SYMBOL: &'static str = "د.ا";
    
    // Rich metadata
    const NAME: &'static str = "Jordanian Dinar";
    const COUNTRY: &'static str = "Jordan";
    const REGION: &'static str = "Middle East";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = true;
    const INTRODUCED_YEAR: u16 = 1950;
    const ISO_4217_NUMBER: u16 = 400;
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
    fn test_jod_currency_properties() {
        assert_eq!(JOD::DECIMALS, 3);
        assert_eq!(JOD::CODE, "JOD");
        assert_eq!(JOD::SYMBOL, "د.ا");
    }

    #[test]
    fn test_jod_amount_creation() {
        let amount = Amount::<JOD>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_jod_amount_with_fils() {
        let amount = Amount::<JOD>::from_minor(100500); // 100.500 JOD
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 100500);
    }
}
