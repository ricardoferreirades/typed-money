use super::{CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};
use crate::Currency;

/// Brazilian Real (BRL)
///
/// The Brazilian real is the official currency of Brazil.
/// It is subdivided into 100 centavos.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, BRL};
///
/// let amount = Amount::<BRL>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(BRL::CODE, "BRL");
/// assert_eq!(BRL::SYMBOL, "R$");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BRL;

impl Currency for BRL {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "BRL";
    const SYMBOL: &'static str = "R$";

    // Rich metadata
    const NAME: &'static str = "Brazilian Real";
    const COUNTRY: &'static str = "Brazil";
    const REGION: &'static str = "South America";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 1994;
    const ISO_4217_NUMBER: u16 = 986;
    const THOUSANDS_SEPARATOR: char = '.';
    const DECIMAL_SEPARATOR: char = ',';
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
    fn test_brl_currency_properties() {
        assert_eq!(BRL::DECIMALS, 2);
        assert_eq!(BRL::CODE, "BRL");
        assert_eq!(BRL::SYMBOL, "R$");
    }

    #[test]
    fn test_brl_amount_creation() {
        let amount = Amount::<BRL>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_brl_amount_with_centavos() {
        let amount = Amount::<BRL>::from_minor(10050); // 100.50 BRL
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
