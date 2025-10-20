use super::{CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};
use crate::Currency;

/// Argentine Peso (ARS)
///
/// The Argentine peso is the currency of Argentina.
/// It is subdivided into 100 centavos.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, ARS};
///
/// let amount = Amount::<ARS>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(ARS::CODE, "ARS");
/// assert_eq!(ARS::SYMBOL, "$");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ARS;

impl Currency for ARS {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "ARS";
    const SYMBOL: &'static str = "$";

    // Rich metadata
    const NAME: &'static str = "Argentine Peso";
    const COUNTRY: &'static str = "Argentina";
    const REGION: &'static str = "South America";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 1992;
    const ISO_4217_NUMBER: u16 = 32;
    const THOUSANDS_SEPARATOR: char = '.';
    const DECIMAL_SEPARATOR: char = ',';
    const SYMBOL_POSITION: SymbolPosition = SymbolPosition::Before;
    const SPACE_BETWEEN: bool = false;
    const VOLATILITY_RATING: VolatilityRating = VolatilityRating::High;
    const LIQUIDITY_RATING: LiquidityRating = LiquidityRating::Low;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_ars_currency_properties() {
        assert_eq!(ARS::DECIMALS, 2);
        assert_eq!(ARS::CODE, "ARS");
        assert_eq!(ARS::SYMBOL, "$");
    }

    #[test]
    fn test_ars_amount_creation() {
        let amount = Amount::<ARS>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_ars_amount_with_centavos() {
        let amount = Amount::<ARS>::from_minor(10050); // 100.50 ARS
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
