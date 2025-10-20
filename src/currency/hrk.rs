use crate::Currency;
use super::{CurrencyType, SymbolPosition, VolatilityRating, LiquidityRating};

/// Croatian Kuna (HRK)
///
/// The Croatian kuna was the currency of Croatia from 1994 until 2023.
/// It was subdivided into 100 lipa.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, HRK};
///
/// let amount = Amount::<HRK>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(HRK::CODE, "HRK");
/// assert_eq!(HRK::SYMBOL, "kn");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HRK;

impl Currency for HRK {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "HRK";
    const SYMBOL: &'static str = "kn";
    
    // Rich metadata (Historical currency - replaced by EUR in 2023)
    const NAME: &'static str = "Croatian Kuna";
    const COUNTRY: &'static str = "Croatia";
    const REGION: &'static str = "Europe";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 1994;
    const ISO_4217_NUMBER: u16 = 191;
    const THOUSANDS_SEPARATOR: char = '.';
    const DECIMAL_SEPARATOR: char = ',';
    const SYMBOL_POSITION: SymbolPosition = SymbolPosition::After;
    const SPACE_BETWEEN: bool = true;
    const VOLATILITY_RATING: VolatilityRating = VolatilityRating::Medium;
    const LIQUIDITY_RATING: LiquidityRating = LiquidityRating::Low;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_hrk_currency_properties() {
        assert_eq!(HRK::DECIMALS, 2);
        assert_eq!(HRK::CODE, "HRK");
        assert_eq!(HRK::SYMBOL, "kn");
    }

    #[test]
    fn test_hrk_amount_creation() {
        let amount = Amount::<HRK>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_hrk_amount_with_lipa() {
        let amount = Amount::<HRK>::from_minor(10050); // 100.50 HRK
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
