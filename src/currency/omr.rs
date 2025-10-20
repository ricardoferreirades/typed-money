use crate::Currency;
use super::{CurrencyType, SymbolPosition, VolatilityRating, LiquidityRating};

/// Omani Rial (OMR)
///
/// The Omani rial is the currency of Oman.
/// It is subdivided into 1000 baisa.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, OMR};
///
/// let amount = Amount::<OMR>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(OMR::CODE, "OMR");
/// assert_eq!(OMR::SYMBOL, "﷼");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct OMR;

impl Currency for OMR {
    const DECIMALS: u8 = 3; // Omani Rial uses 3 decimal places (baisa)
    const CODE: &'static str = "OMR";
    const SYMBOL: &'static str = "﷼";
    
    // Rich metadata
    const NAME: &'static str = "Omani Rial";
    const COUNTRY: &'static str = "Oman";
    const REGION: &'static str = "Middle East";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = true;
    const INTRODUCED_YEAR: u16 = 1973;
    const ISO_4217_NUMBER: u16 = 512;
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
    fn test_omr_currency_properties() {
        assert_eq!(OMR::DECIMALS, 3);
        assert_eq!(OMR::CODE, "OMR");
        assert_eq!(OMR::SYMBOL, "﷼");
    }

    #[test]
    fn test_omr_amount_creation() {
        let amount = Amount::<OMR>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_omr_amount_with_baisa() {
        let amount = Amount::<OMR>::from_minor(100500); // 100.500 OMR
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 100500);
    }
}
