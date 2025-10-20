use super::{CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};
use crate::Currency;

/// Swedish Krona (SEK)
///
/// The Swedish krona is the currency of Sweden.
/// It is subdivided into 100 öre.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, SEK};
///
/// let amount = Amount::<SEK>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(SEK::CODE, "SEK");
/// assert_eq!(SEK::SYMBOL, "kr");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SEK;

impl Currency for SEK {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "SEK";
    const SYMBOL: &'static str = "kr";

    // Rich metadata
    const NAME: &'static str = "Swedish Krona";
    const COUNTRY: &'static str = "Sweden";
    const REGION: &'static str = "Europe";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = true;
    const INTRODUCED_YEAR: u16 = 1873;
    const ISO_4217_NUMBER: u16 = 752;
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
    fn test_sek_currency_properties() {
        assert_eq!(SEK::DECIMALS, 2);
        assert_eq!(SEK::CODE, "SEK");
        assert_eq!(SEK::SYMBOL, "kr");
    }

    #[test]
    fn test_sek_amount_creation() {
        let amount = Amount::<SEK>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_sek_amount_with_ore() {
        let amount = Amount::<SEK>::from_minor(10050); // 100.50 SEK
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
