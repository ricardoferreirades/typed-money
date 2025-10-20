use super::{Currency, CurrencyType, SymbolPosition, VolatilityRating, LiquidityRating};

/// Uruguayan Peso (UYU)
///
/// The Uruguayan peso is the currency of Uruguay.
/// It is subdivided into 100 centésimos.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, UYU};
///
/// let amount = Amount::<UYU>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(UYU::CODE, "UYU");
/// assert_eq!(UYU::SYMBOL, "$U");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UYU;

impl Currency for UYU {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "UYU";
    const SYMBOL: &'static str = "$U";
    
    // Rich metadata
    const NAME: &'static str = "Uruguayan Peso";
    const COUNTRY: &'static str = "Uruguay";
    const REGION: &'static str = "South America";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 1993;
    const ISO_4217_NUMBER: u16 = 858;
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
    fn test_uyu_currency_properties() {
        assert_eq!(UYU::DECIMALS, 2);
        assert_eq!(UYU::CODE, "UYU");
        assert_eq!(UYU::SYMBOL, "$U");
    }

    #[test]
    fn test_uyu_amount_creation() {
        let amount = Amount::<UYU>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_uyu_amount_with_centésimos() {
        let amount = Amount::<UYU>::from_minor(10050); // 100.50 UYU
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
