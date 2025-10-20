use super::{CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};
use crate::Currency;

/// Philippine Peso (PHP)
///
/// The Philippine peso is the currency of the Philippines.
/// It is subdivided into 100 centavos.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, PHP};
///
/// let amount = Amount::<PHP>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(PHP::CODE, "PHP");
/// assert_eq!(PHP::SYMBOL, "₱");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PHP;

impl Currency for PHP {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "PHP";
    const SYMBOL: &'static str = "₱";

    // Rich metadata
    const NAME: &'static str = "Philippine Peso";
    const COUNTRY: &'static str = "Philippines";
    const REGION: &'static str = "Asia";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 1949;
    const ISO_4217_NUMBER: u16 = 608;
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
    fn test_php_currency_properties() {
        assert_eq!(PHP::DECIMALS, 2);
        assert_eq!(PHP::CODE, "PHP");
        assert_eq!(PHP::SYMBOL, "₱");
    }

    #[test]
    fn test_php_amount_creation() {
        let amount = Amount::<PHP>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_php_amount_with_centavos() {
        let amount = Amount::<PHP>::from_minor(10050); // 100.50 PHP
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
