use super::{Currency, CurrencyType, SymbolPosition, VolatilityRating, LiquidityRating};

/// Tunisian Dinar (TND)
///
/// The Tunisian dinar is the currency of Tunisia.
/// It is subdivided into 1000 millimes.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, TND};
///
/// let amount = Amount::<TND>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(TND::CODE, "TND");
/// assert_eq!(TND::SYMBOL, "د.ت");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TND;

impl Currency for TND {
    const DECIMALS: u8 = 3; // Tunisian Dinar uses 3 decimal places (millimes)
    const CODE: &'static str = "TND";
    const SYMBOL: &'static str = "د.ت";
    
    // Rich metadata
    const NAME: &'static str = "Tunisian Dinar";
    const COUNTRY: &'static str = "Tunisia";
    const REGION: &'static str = "North Africa";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 1958;
    const ISO_4217_NUMBER: u16 = 788;
    const THOUSANDS_SEPARATOR: char = '.';
    const DECIMAL_SEPARATOR: char = ',';
    const SYMBOL_POSITION: SymbolPosition = SymbolPosition::After;
    const SPACE_BETWEEN: bool = true;
    const VOLATILITY_RATING: VolatilityRating = VolatilityRating::Medium;
    const LIQUIDITY_RATING: LiquidityRating = LiquidityRating::Low;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_tnd_currency_properties() {
        assert_eq!(TND::DECIMALS, 3);
        assert_eq!(TND::CODE, "TND");
        assert_eq!(TND::SYMBOL, "د.ت");
    }

    #[test]
    fn test_tnd_amount_creation() {
        let amount = Amount::<TND>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_tnd_amount_with_millimes() {
        let amount = Amount::<TND>::from_minor(100500); // 100.500 TND
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 100500);
    }
}
