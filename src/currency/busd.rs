use super::{CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};
use crate::Currency;

/// Binance USD (BUSD)
///
/// Binance USD is a USD-backed stablecoin issued by Binance in partnership
/// with Paxos. It is designed to maintain a 1:1 ratio with the US dollar
/// and is regulated by the New York State Department of Financial Services.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, BUSD};
///
/// let amount = Amount::<BUSD>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(BUSD::CODE, "BUSD");
/// assert_eq!(BUSD::SYMBOL, "BUSD");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BUSD;

impl Currency for BUSD {
    const DECIMALS: u8 = 18;
    const CODE: &'static str = "BUSD";
    const SYMBOL: &'static str = "BUSD";

    // Stablecoin metadata
    const NAME: &'static str = "Binance USD";
    const COUNTRY: &'static str = "Global";
    const REGION: &'static str = "Worldwide";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Cryptocurrency;
    const IS_MAJOR: bool = true;
    const IS_STABLE: bool = true;
    const INTRODUCED_YEAR: u16 = 2019;
    const ISO_4217_NUMBER: u16 = 0; // No official ISO code for cryptocurrencies
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
    fn test_busd_currency_properties() {
        assert_eq!(BUSD::DECIMALS, 18);
        assert_eq!(BUSD::CODE, "BUSD");
        assert_eq!(BUSD::SYMBOL, "BUSD");
    }

    #[test]
    fn test_busd_amount_creation() {
        let amount = Amount::<BUSD>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_busd_amount_with_wei() {
        let amount = Amount::<BUSD>::from_minor(1_000_000_000_000_000_000); // 1.000000000000000000 BUSD
        assert_eq!(amount.to_major_floor(), 1);
        assert_eq!(amount.to_minor(), 1_000_000_000_000_000_000);
    }
}
