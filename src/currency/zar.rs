use super::{CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};
use crate::Currency;

/// South African Rand (ZAR)
///
/// The South African rand is the currency of South Africa.
/// It is subdivided into 100 cents.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, ZAR};
///
/// let amount = Amount::<ZAR>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(ZAR::CODE, "ZAR");
/// assert_eq!(ZAR::SYMBOL, "R");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ZAR;

impl Currency for ZAR {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "ZAR";
    const SYMBOL: &'static str = "R";

    // Rich metadata
    const NAME: &'static str = "South African Rand";
    const COUNTRY: &'static str = "South Africa";
    const REGION: &'static str = "Africa";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 1961;
    const ISO_4217_NUMBER: u16 = 710;
    const THOUSANDS_SEPARATOR: char = ',';
    const DECIMAL_SEPARATOR: char = '.';
    const SYMBOL_POSITION: SymbolPosition = SymbolPosition::Before;
    const SPACE_BETWEEN: bool = false;
    const VOLATILITY_RATING: VolatilityRating = VolatilityRating::High;
    const LIQUIDITY_RATING: LiquidityRating = LiquidityRating::Medium;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_zar_currency_properties() {
        assert_eq!(ZAR::DECIMALS, 2);
        assert_eq!(ZAR::CODE, "ZAR");
        assert_eq!(ZAR::SYMBOL, "R");
    }

    #[test]
    fn test_zar_amount_creation() {
        let amount = Amount::<ZAR>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_zar_amount_with_cents() {
        let amount = Amount::<ZAR>::from_minor(10050); // 100.50 ZAR
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
