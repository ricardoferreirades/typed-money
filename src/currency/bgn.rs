use super::{CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};
use crate::Currency;

/// Bulgarian Lev (BGN)
///
/// The Bulgarian lev is the currency of Bulgaria.
/// It is subdivided into 100 stotinki.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, BGN};
///
/// let amount = Amount::<BGN>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(BGN::CODE, "BGN");
/// assert_eq!(BGN::SYMBOL, "лв");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BGN;

impl Currency for BGN {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "BGN";
    const SYMBOL: &'static str = "лв";

    // Rich metadata
    const NAME: &'static str = "Bulgarian Lev";
    const COUNTRY: &'static str = "Bulgaria";
    const REGION: &'static str = "Europe";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = true;
    const INTRODUCED_YEAR: u16 = 1999;
    const ISO_4217_NUMBER: u16 = 975;
    const THOUSANDS_SEPARATOR: char = ' ';
    const DECIMAL_SEPARATOR: char = ',';
    const SYMBOL_POSITION: SymbolPosition = SymbolPosition::After;
    const SPACE_BETWEEN: bool = true;
    const VOLATILITY_RATING: VolatilityRating = VolatilityRating::Low;
    const LIQUIDITY_RATING: LiquidityRating = LiquidityRating::Low;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_bgn_currency_properties() {
        assert_eq!(BGN::DECIMALS, 2);
        assert_eq!(BGN::CODE, "BGN");
        assert_eq!(BGN::SYMBOL, "лв");
    }

    #[test]
    fn test_bgn_amount_creation() {
        let amount = Amount::<BGN>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_bgn_amount_with_stotinki() {
        let amount = Amount::<BGN>::from_minor(10050); // 100.50 BGN
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
