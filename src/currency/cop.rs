use super::{Currency, CurrencyType, SymbolPosition, VolatilityRating, LiquidityRating};

/// Colombian Peso (COP)
///
/// The Colombian peso is the currency of Colombia.
/// It is subdivided into 100 centavos.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, COP};
///
/// let amount = Amount::<COP>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(COP::CODE, "COP");
/// assert_eq!(COP::SYMBOL, "$");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct COP;

impl Currency for COP {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "COP";
    const SYMBOL: &'static str = "$";
    
    // Rich metadata
    const NAME: &'static str = "Colombian Peso";
    const COUNTRY: &'static str = "Colombia";
    const REGION: &'static str = "South America";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 1810;
    const ISO_4217_NUMBER: u16 = 170;
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
    fn test_cop_currency_properties() {
        assert_eq!(COP::DECIMALS, 2);
        assert_eq!(COP::CODE, "COP");
        assert_eq!(COP::SYMBOL, "$");
    }

    #[test]
    fn test_cop_amount_creation() {
        let amount = Amount::<COP>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_cop_amount_with_centavos() {
        let amount = Amount::<COP>::from_minor(10050); // 100.50 COP
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
