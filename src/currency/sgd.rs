use super::{CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};
use crate::Currency;

/// Singapore Dollar (SGD)
///
/// The Singapore dollar is the official currency of Singapore.
/// It is divided into 100 cents.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, SGD};
///
/// let amount = Amount::<SGD>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(SGD::CODE, "SGD");
/// assert_eq!(SGD::SYMBOL, "S$");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SGD;

impl Currency for SGD {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "SGD";
    const SYMBOL: &'static str = "S$";

    // Rich metadata
    const NAME: &'static str = "Singapore Dollar";
    const COUNTRY: &'static str = "Singapore";
    const REGION: &'static str = "Asia";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = true;
    const INTRODUCED_YEAR: u16 = 1967;
    const ISO_4217_NUMBER: u16 = 702;
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
    fn test_sgd_currency_properties() {
        assert_eq!(SGD::DECIMALS, 2);
        assert_eq!(SGD::CODE, "SGD");
        assert_eq!(SGD::SYMBOL, "S$");
    }

    #[test]
    fn test_sgd_amount_creation() {
        let amount = Amount::<SGD>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_sgd_amount_with_cents() {
        let amount = Amount::<SGD>::from_minor(10050); // 100.50 SGD
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
