use crate::Currency;
use super::{CurrencyType, SymbolPosition, VolatilityRating, LiquidityRating};

/// Canadian Dollar (CAD)
///
/// The Canadian dollar is the currency of Canada. It is abbreviated with the dollar sign $,
/// or sometimes C$ to distinguish it from other dollar-denominated currencies.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, CAD};
///
/// let amount = Amount::<CAD>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(CAD::CODE, "CAD");
/// assert_eq!(CAD::SYMBOL, "C$");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CAD;

impl Currency for CAD {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "CAD";
    const SYMBOL: &'static str = "C$";
    
    // Rich metadata
    const NAME: &'static str = "Canadian Dollar";
    const COUNTRY: &'static str = "Canada";
    const REGION: &'static str = "North America";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = true;
    const IS_STABLE: bool = true;
    const INTRODUCED_YEAR: u16 = 1858;
    const ISO_4217_NUMBER: u16 = 124;
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
    fn test_cad_currency_properties() {
        assert_eq!(CAD::DECIMALS, 2);
        assert_eq!(CAD::CODE, "CAD");
        assert_eq!(CAD::SYMBOL, "C$");
    }

    #[test]
    fn test_cad_amount_creation() {
        let amount = Amount::<CAD>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_cad_amount_with_cents() {
        let amount = Amount::<CAD>::from_minor(10050); // 100.50 CAD
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
