use super::{CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};
use crate::Currency;

/// Indian Rupee (INR)
///
/// The Indian rupee is the official currency of India.
/// It is subdivided into 100 paise.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, INR};
///
/// let amount = Amount::<INR>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(INR::CODE, "INR");
/// assert_eq!(INR::SYMBOL, "₹");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct INR;

impl Currency for INR {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "INR";
    const SYMBOL: &'static str = "₹";

    // Rich metadata
    const NAME: &'static str = "Indian Rupee";
    const COUNTRY: &'static str = "India";
    const REGION: &'static str = "Asia";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 1957;
    const ISO_4217_NUMBER: u16 = 356;
    const THOUSANDS_SEPARATOR: char = ',';
    const DECIMAL_SEPARATOR: char = '.';
    const SYMBOL_POSITION: SymbolPosition = SymbolPosition::Before;
    const SPACE_BETWEEN: bool = false;
    const VOLATILITY_RATING: VolatilityRating = VolatilityRating::Medium;
    const LIQUIDITY_RATING: LiquidityRating = LiquidityRating::High;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_inr_currency_properties() {
        assert_eq!(INR::DECIMALS, 2);
        assert_eq!(INR::CODE, "INR");
        assert_eq!(INR::SYMBOL, "₹");
    }

    #[test]
    fn test_inr_amount_creation() {
        let amount = Amount::<INR>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_inr_amount_with_paise() {
        let amount = Amount::<INR>::from_minor(10050); // 100.50 INR
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
