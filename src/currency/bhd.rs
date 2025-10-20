use super::{CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};
use crate::Currency;

/// Bahraini Dinar (BHD)
///
/// The Bahraini dinar is the currency of Bahrain.
/// It is subdivided into 1000 fils.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, BHD};
///
/// let amount = Amount::<BHD>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(BHD::CODE, "BHD");
/// assert_eq!(BHD::SYMBOL, "د.ب");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BHD;

impl Currency for BHD {
    const DECIMALS: u8 = 3; // Bahraini Dinar uses 3 decimal places (fils)
    const CODE: &'static str = "BHD";
    const SYMBOL: &'static str = "د.ب";

    // Rich metadata
    const NAME: &'static str = "Bahraini Dinar";
    const COUNTRY: &'static str = "Bahrain";
    const REGION: &'static str = "Middle East";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = true;
    const INTRODUCED_YEAR: u16 = 1965;
    const ISO_4217_NUMBER: u16 = 48;
    const THOUSANDS_SEPARATOR: char = ',';
    const DECIMAL_SEPARATOR: char = '.';
    const SYMBOL_POSITION: SymbolPosition = SymbolPosition::After;
    const SPACE_BETWEEN: bool = true;
    const VOLATILITY_RATING: VolatilityRating = VolatilityRating::Low;
    const LIQUIDITY_RATING: LiquidityRating = LiquidityRating::Medium;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_bhd_currency_properties() {
        assert_eq!(BHD::DECIMALS, 3);
        assert_eq!(BHD::CODE, "BHD");
        assert_eq!(BHD::SYMBOL, "د.ب");
    }

    #[test]
    fn test_bhd_amount_creation() {
        let amount = Amount::<BHD>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_bhd_amount_with_fils() {
        let amount = Amount::<BHD>::from_minor(100500); // 100.500 BHD
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 100500);
    }
}
