use super::{CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};
use crate::Currency;

/// Qatari Riyal (QAR)
///
/// The Qatari riyal is the currency of Qatar.
/// It is subdivided into 100 dirhams.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, QAR};
///
/// let amount = Amount::<QAR>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(QAR::CODE, "QAR");
/// assert_eq!(QAR::SYMBOL, "﷼");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct QAR;

impl Currency for QAR {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "QAR";
    const SYMBOL: &'static str = "﷼";

    // Rich metadata
    const NAME: &'static str = "Qatari Riyal";
    const COUNTRY: &'static str = "Qatar";
    const REGION: &'static str = "Middle East";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = true;
    const INTRODUCED_YEAR: u16 = 1973;
    const ISO_4217_NUMBER: u16 = 634;
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
    fn test_qar_currency_properties() {
        assert_eq!(QAR::DECIMALS, 2);
        assert_eq!(QAR::CODE, "QAR");
        assert_eq!(QAR::SYMBOL, "﷼");
    }

    #[test]
    fn test_qar_amount_creation() {
        let amount = Amount::<QAR>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_qar_amount_with_dirhams() {
        let amount = Amount::<QAR>::from_minor(10050); // 100.50 QAR
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
