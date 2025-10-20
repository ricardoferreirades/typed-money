use crate::Currency;
use super::{CurrencyType, SymbolPosition, VolatilityRating, LiquidityRating};

/// Australian Dollar (AUD)
///
/// The Australian dollar is the currency of Australia, including its external territories.
/// It is subdivided into 100 cents.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, AUD};
///
/// let amount = Amount::<AUD>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(AUD::CODE, "AUD");
/// assert_eq!(AUD::SYMBOL, "A$");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AUD;

impl Currency for AUD {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "AUD";
    const SYMBOL: &'static str = "A$";
    
    // Rich metadata
    const NAME: &'static str = "Australian Dollar";
    const COUNTRY: &'static str = "Australia";
    const REGION: &'static str = "Oceania";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = true;
    const IS_STABLE: bool = true;
    const INTRODUCED_YEAR: u16 = 1966;
    const ISO_4217_NUMBER: u16 = 036;
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
    fn test_aud_currency_properties() {
        assert_eq!(AUD::DECIMALS, 2);
        assert_eq!(AUD::CODE, "AUD");
        assert_eq!(AUD::SYMBOL, "A$");
    }

    #[test]
    fn test_aud_amount_creation() {
        let amount = Amount::<AUD>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_aud_amount_with_cents() {
        let amount = Amount::<AUD>::from_minor(10050); // 100.50 AUD
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
