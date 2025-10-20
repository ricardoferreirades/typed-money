use super::{CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};
use crate::Currency;

/// Norwegian Krone (NOK)
///
/// The Norwegian krone is the currency of Norway.
/// It is subdivided into 100 øre.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, NOK};
///
/// let amount = Amount::<NOK>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(NOK::CODE, "NOK");
/// assert_eq!(NOK::SYMBOL, "kr");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NOK;

impl Currency for NOK {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "NOK";
    const SYMBOL: &'static str = "kr";

    // Rich metadata
    const NAME: &'static str = "Norwegian Krone";
    const COUNTRY: &'static str = "Norway";
    const REGION: &'static str = "Europe";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = true;
    const INTRODUCED_YEAR: u16 = 1875;
    const ISO_4217_NUMBER: u16 = 578;
    const THOUSANDS_SEPARATOR: char = ' ';
    const DECIMAL_SEPARATOR: char = ',';
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
    fn test_nok_currency_properties() {
        assert_eq!(NOK::DECIMALS, 2);
        assert_eq!(NOK::CODE, "NOK");
        assert_eq!(NOK::SYMBOL, "kr");
    }

    #[test]
    fn test_nok_amount_creation() {
        let amount = Amount::<NOK>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_nok_amount_with_ore() {
        let amount = Amount::<NOK>::from_minor(10050); // 100.50 NOK
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
