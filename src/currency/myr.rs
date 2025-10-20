use crate::Currency;
use super::{CurrencyType, SymbolPosition, VolatilityRating, LiquidityRating};

/// Malaysian Ringgit (MYR)
///
/// The Malaysian ringgit is the currency of Malaysia.
/// It is subdivided into 100 sen.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, MYR};
///
/// let amount = Amount::<MYR>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(MYR::CODE, "MYR");
/// assert_eq!(MYR::SYMBOL, "RM");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MYR;

impl Currency for MYR {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "MYR";
    const SYMBOL: &'static str = "RM";
    
    // Rich metadata
    const NAME: &'static str = "Malaysian Ringgit";
    const COUNTRY: &'static str = "Malaysia";
    const REGION: &'static str = "Asia";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 1967;
    const ISO_4217_NUMBER: u16 = 458;
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
    fn test_myr_currency_properties() {
        assert_eq!(MYR::DECIMALS, 2);
        assert_eq!(MYR::CODE, "MYR");
        assert_eq!(MYR::SYMBOL, "RM");
    }

    #[test]
    fn test_myr_amount_creation() {
        let amount = Amount::<MYR>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_myr_amount_with_sen() {
        let amount = Amount::<MYR>::from_minor(10050); // 100.50 MYR
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
