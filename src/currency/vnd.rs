use super::{CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};
use crate::Currency;

/// Vietnamese Dong (VND)
///
/// The Vietnamese dong is the currency of Vietnam.
/// It is subdivided into 10 hào and 100 xu, though these are no longer in circulation.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, VND};
///
/// let amount = Amount::<VND>::from_major(1000000);
/// assert_eq!(amount.to_major_floor(), 1000000);
/// assert_eq!(VND::CODE, "VND");
/// assert_eq!(VND::SYMBOL, "₫");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct VND;

impl Currency for VND {
    const DECIMALS: u8 = 0; // Vietnamese Dong typically doesn't use decimal places
    const CODE: &'static str = "VND";
    const SYMBOL: &'static str = "₫";

    // Rich metadata
    const NAME: &'static str = "Vietnamese Dong";
    const COUNTRY: &'static str = "Vietnam";
    const REGION: &'static str = "Asia";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 1978;
    const ISO_4217_NUMBER: u16 = 704;
    const THOUSANDS_SEPARATOR: char = '.';
    const DECIMAL_SEPARATOR: char = ',';
    const SYMBOL_POSITION: SymbolPosition = SymbolPosition::After;
    const SPACE_BETWEEN: bool = true;
    const VOLATILITY_RATING: VolatilityRating = VolatilityRating::High;
    const LIQUIDITY_RATING: LiquidityRating = LiquidityRating::Low;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_vnd_currency_properties() {
        assert_eq!(VND::DECIMALS, 0);
        assert_eq!(VND::CODE, "VND");
        assert_eq!(VND::SYMBOL, "₫");
    }

    #[test]
    fn test_vnd_amount_creation() {
        let amount = Amount::<VND>::from_major(1000000);
        assert_eq!(amount.to_major_floor(), 1000000);
    }

    #[test]
    fn test_vnd_amount_with_minor() {
        let amount = Amount::<VND>::from_minor(1000000); // 1000000 VND (no decimals)
        assert_eq!(amount.to_major_floor(), 1000000);
        assert_eq!(amount.to_minor(), 1000000);
    }
}
