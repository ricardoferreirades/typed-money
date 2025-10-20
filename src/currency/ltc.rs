use super::{CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};
use crate::Currency;

/// Litecoin (LTC)
///
/// Litecoin is a peer-to-peer cryptocurrency and open-source software project
/// released under the MIT/X11 license. It is based on Bitcoin but with faster
/// transaction confirmation times and improved storage efficiency.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, LTC};
///
/// let amount = Amount::<LTC>::from_major(1);
/// assert_eq!(amount.to_major_floor(), 1);
/// assert_eq!(LTC::CODE, "LTC");
/// assert_eq!(LTC::SYMBOL, "Ł");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LTC;

impl Currency for LTC {
    const DECIMALS: u8 = 8;
    const CODE: &'static str = "LTC";
    const SYMBOL: &'static str = "Ł";

    // Cryptocurrency metadata
    const NAME: &'static str = "Litecoin";
    const COUNTRY: &'static str = "Global";
    const REGION: &'static str = "Worldwide";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Cryptocurrency;
    const IS_MAJOR: bool = true;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 2011;
    const ISO_4217_NUMBER: u16 = 0; // No official ISO code for cryptocurrencies
    const THOUSANDS_SEPARATOR: char = ',';
    const DECIMAL_SEPARATOR: char = '.';
    const SYMBOL_POSITION: SymbolPosition = SymbolPosition::Before;
    const SPACE_BETWEEN: bool = false;
    const VOLATILITY_RATING: VolatilityRating = VolatilityRating::High;
    const LIQUIDITY_RATING: LiquidityRating = LiquidityRating::High;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_ltc_currency_properties() {
        assert_eq!(LTC::DECIMALS, 8);
        assert_eq!(LTC::CODE, "LTC");
        assert_eq!(LTC::SYMBOL, "Ł");
    }

    #[test]
    fn test_ltc_amount_creation() {
        let amount = Amount::<LTC>::from_major(1);
        assert_eq!(amount.to_major_floor(), 1);
    }

    #[test]
    fn test_ltc_amount_with_satoshis() {
        let amount = Amount::<LTC>::from_minor(100_000_000); // 1.00000000 LTC
        assert_eq!(amount.to_major_floor(), 1);
        assert_eq!(amount.to_minor(), 100_000_000);
    }
}
