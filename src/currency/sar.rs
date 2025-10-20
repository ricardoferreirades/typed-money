use crate::Currency;
use super::{CurrencyType, SymbolPosition, VolatilityRating, LiquidityRating};

/// Saudi Riyal (SAR)
///
/// The Saudi riyal is the currency of Saudi Arabia.
/// It is subdivided into 100 halalas.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, SAR};
///
/// let amount = Amount::<SAR>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(SAR::CODE, "SAR");
/// assert_eq!(SAR::SYMBOL, "﷼");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SAR;

impl Currency for SAR {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "SAR";
    const SYMBOL: &'static str = "﷼";
    
    // Rich metadata
    const NAME: &'static str = "Saudi Riyal";
    const COUNTRY: &'static str = "Saudi Arabia";
    const REGION: &'static str = "Middle East";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = true;
    const INTRODUCED_YEAR: u16 = 1960;
    const ISO_4217_NUMBER: u16 = 682;
    const THOUSANDS_SEPARATOR: char = ',';
    const DECIMAL_SEPARATOR: char = '.';
    const SYMBOL_POSITION: SymbolPosition = SymbolPosition::After;
    const SPACE_BETWEEN: bool = true;
    const VOLATILITY_RATING: VolatilityRating = VolatilityRating::Low;
    const LIQUIDITY_RATING: LiquidityRating = LiquidityRating::High;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_sar_currency_properties() {
        assert_eq!(SAR::DECIMALS, 2);
        assert_eq!(SAR::CODE, "SAR");
        assert_eq!(SAR::SYMBOL, "﷼");
    }

    #[test]
    fn test_sar_amount_creation() {
        let amount = Amount::<SAR>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_sar_amount_with_halalas() {
        let amount = Amount::<SAR>::from_minor(10050); // 100.50 SAR
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
