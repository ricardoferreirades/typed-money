use crate::Currency;
use super::{CurrencyType, SymbolPosition, VolatilityRating, LiquidityRating};

/// Indonesian Rupiah (IDR)
///
/// The Indonesian rupiah is the currency of Indonesia.
/// It is subdivided into 100 sen, though sen coins are no longer in circulation.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, IDR};
///
/// let amount = Amount::<IDR>::from_major(100000);
/// assert_eq!(amount.to_major_floor(), 100000);
/// assert_eq!(IDR::CODE, "IDR");
/// assert_eq!(IDR::SYMBOL, "Rp");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct IDR;

impl Currency for IDR {
    const DECIMALS: u8 = 0; // Indonesian Rupiah typically doesn't use decimal places
    const CODE: &'static str = "IDR";
    const SYMBOL: &'static str = "Rp";
    
    // Rich metadata
    const NAME: &'static str = "Indonesian Rupiah";
    const COUNTRY: &'static str = "Indonesia";
    const REGION: &'static str = "Asia";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 1949;
    const ISO_4217_NUMBER: u16 = 360;
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
    fn test_idr_currency_properties() {
        assert_eq!(IDR::DECIMALS, 0);
        assert_eq!(IDR::CODE, "IDR");
        assert_eq!(IDR::SYMBOL, "Rp");
    }

    #[test]
    fn test_idr_amount_creation() {
        let amount = Amount::<IDR>::from_major(100000);
        assert_eq!(amount.to_major_floor(), 100000);
    }

    #[test]
    fn test_idr_amount_with_minor() {
        let amount = Amount::<IDR>::from_minor(100000); // 100000 IDR (no decimals)
        assert_eq!(amount.to_major_floor(), 100000);
        assert_eq!(amount.to_minor(), 100000);
    }
}
