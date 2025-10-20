use super::{Currency, CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};

/// Ghanaian Cedi (GHS)
///
/// The Ghanaian cedi is the currency of Ghana.
/// It is subdivided into 100 pesewas.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, GHS};
///
/// let amount = Amount::<GHS>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(GHS::CODE, "GHS");
/// assert_eq!(GHS::SYMBOL, "₵");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct GHS;

impl Currency for GHS {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "GHS";
    const SYMBOL: &'static str = "₵";

    // Rich metadata
    const NAME: &'static str = "Ghanaian Cedi";
    const COUNTRY: &'static str = "Ghana";
    const REGION: &'static str = "West Africa";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 2007;
    const ISO_4217_NUMBER: u16 = 936;
    const THOUSANDS_SEPARATOR: char = ',';
    const DECIMAL_SEPARATOR: char = '.';
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
    fn test_ghs_currency_properties() {
        assert_eq!(GHS::DECIMALS, 2);
        assert_eq!(GHS::CODE, "GHS");
        assert_eq!(GHS::SYMBOL, "₵");
    }

    #[test]
    fn test_ghs_amount_creation() {
        let amount = Amount::<GHS>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_ghs_amount_with_pesewas() {
        let amount = Amount::<GHS>::from_minor(10050); // 100.50 GHS
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
