use super::{CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};
use crate::Currency;

/// Chinese Yuan (CNY)
///
/// The renminbi is the official currency of the People's Republic of China.
/// The yuan is the basic unit of the renminbi, but is also used to refer to the Chinese currency generally.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, CNY};
///
/// let amount = Amount::<CNY>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(CNY::CODE, "CNY");
/// assert_eq!(CNY::SYMBOL, "¥");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CNY;

impl Currency for CNY {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "CNY";
    const SYMBOL: &'static str = "¥";

    // Rich metadata
    const NAME: &'static str = "Chinese Yuan";
    const COUNTRY: &'static str = "China";
    const REGION: &'static str = "Asia";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = true;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 1948;
    const ISO_4217_NUMBER: u16 = 156;
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
    fn test_cny_currency_properties() {
        assert_eq!(CNY::DECIMALS, 2);
        assert_eq!(CNY::CODE, "CNY");
        assert_eq!(CNY::SYMBOL, "¥");
    }

    #[test]
    fn test_cny_amount_creation() {
        let amount = Amount::<CNY>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_cny_amount_with_cents() {
        let amount = Amount::<CNY>::from_minor(10050); // 100.50 CNY
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
