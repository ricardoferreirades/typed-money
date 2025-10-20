use super::{CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};
use crate::Currency;

/// Turkish Lira (TRY)
///
/// The Turkish lira is the currency of Turkey and Northern Cyprus.
/// It is subdivided into 100 kuruş.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, TRY};
///
/// let amount = Amount::<TRY>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(TRY::CODE, "TRY");
/// assert_eq!(TRY::SYMBOL, "₺");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TRY;

impl Currency for TRY {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "TRY";
    const SYMBOL: &'static str = "₺";

    // Rich metadata
    const NAME: &'static str = "Turkish Lira";
    const COUNTRY: &'static str = "Turkey";
    const REGION: &'static str = "Middle East";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 2005;
    const ISO_4217_NUMBER: u16 = 949;
    const THOUSANDS_SEPARATOR: char = '.';
    const DECIMAL_SEPARATOR: char = ',';
    const SYMBOL_POSITION: SymbolPosition = SymbolPosition::After;
    const SPACE_BETWEEN: bool = true;
    const VOLATILITY_RATING: VolatilityRating = VolatilityRating::High;
    const LIQUIDITY_RATING: LiquidityRating = LiquidityRating::Medium;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_try_currency_properties() {
        assert_eq!(TRY::DECIMALS, 2);
        assert_eq!(TRY::CODE, "TRY");
        assert_eq!(TRY::SYMBOL, "₺");
    }

    #[test]
    fn test_try_amount_creation() {
        let amount = Amount::<TRY>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_try_amount_with_kuruş() {
        let amount = Amount::<TRY>::from_minor(10050); // 100.50 TRY
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
