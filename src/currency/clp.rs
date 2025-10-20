use super::{CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};
use crate::Currency;

/// Chilean Peso (CLP)
///
/// The Chilean peso is the currency of Chile.
/// It is subdivided into 100 centavos, although centavo coins are no longer in circulation.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, CLP};
///
/// let amount = Amount::<CLP>::from_major(1000);
/// assert_eq!(amount.to_major_floor(), 1000);
/// assert_eq!(CLP::CODE, "CLP");
/// assert_eq!(CLP::SYMBOL, "$");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CLP;

impl Currency for CLP {
    const DECIMALS: u8 = 0; // Chilean Peso typically doesn't use decimal places
    const CODE: &'static str = "CLP";
    const SYMBOL: &'static str = "$";

    // Rich metadata
    const NAME: &'static str = "Chilean Peso";
    const COUNTRY: &'static str = "Chile";
    const REGION: &'static str = "South America";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 1975;
    const ISO_4217_NUMBER: u16 = 152;
    const THOUSANDS_SEPARATOR: char = '.';
    const DECIMAL_SEPARATOR: char = ',';
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
    fn test_clp_currency_properties() {
        assert_eq!(CLP::DECIMALS, 0);
        assert_eq!(CLP::CODE, "CLP");
        assert_eq!(CLP::SYMBOL, "$");
    }

    #[test]
    fn test_clp_amount_creation() {
        let amount = Amount::<CLP>::from_major(1000);
        assert_eq!(amount.to_major_floor(), 1000);
    }

    #[test]
    fn test_clp_amount_with_minor() {
        let amount = Amount::<CLP>::from_minor(1000); // 1000 CLP (no decimals)
        assert_eq!(amount.to_major_floor(), 1000);
        assert_eq!(amount.to_minor(), 1000);
    }
}
