use super::{Currency, CurrencyType, SymbolPosition, VolatilityRating, LiquidityRating};

/// Moroccan Dirham (MAD)
///
/// The Moroccan dirham is the currency of Morocco.
/// It is subdivided into 100 centimes.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, MAD};
///
/// let amount = Amount::<MAD>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(MAD::CODE, "MAD");
/// assert_eq!(MAD::SYMBOL, "د.م.");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MAD;

impl Currency for MAD {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "MAD";
    const SYMBOL: &'static str = "د.م.";
    
    // Rich metadata
    const NAME: &'static str = "Moroccan Dirham";
    const COUNTRY: &'static str = "Morocco";
    const REGION: &'static str = "North Africa";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 1960;
    const ISO_4217_NUMBER: u16 = 504;
    const THOUSANDS_SEPARATOR: char = '.';
    const DECIMAL_SEPARATOR: char = ',';
    const SYMBOL_POSITION: SymbolPosition = SymbolPosition::After;
    const SPACE_BETWEEN: bool = true;
    const VOLATILITY_RATING: VolatilityRating = VolatilityRating::Medium;
    const LIQUIDITY_RATING: LiquidityRating = LiquidityRating::Medium;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_mad_currency_properties() {
        assert_eq!(MAD::DECIMALS, 2);
        assert_eq!(MAD::CODE, "MAD");
        assert_eq!(MAD::SYMBOL, "د.م.");
    }

    #[test]
    fn test_mad_amount_creation() {
        let amount = Amount::<MAD>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_mad_amount_with_centimes() {
        let amount = Amount::<MAD>::from_minor(10050); // 100.50 MAD
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
