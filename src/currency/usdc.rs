use super::{CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};
use crate::Currency;

/// USD Coin (USDC)
///
/// USD Coin is a fully-backed U.S. dollar stablecoin. It is issued by Centre,
/// a consortium that includes Coinbase and Circle, and is designed to maintain
/// a 1:1 ratio with the US dollar.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, USDC};
///
/// let amount = Amount::<USDC>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(USDC::CODE, "USDC");
/// assert_eq!(USDC::SYMBOL, "USDC");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct USDC;

impl Currency for USDC {
    const DECIMALS: u8 = 6;
    const CODE: &'static str = "USDC";
    const SYMBOL: &'static str = "USDC";

    // Stablecoin metadata
    const NAME: &'static str = "USD Coin";
    const COUNTRY: &'static str = "Global";
    const REGION: &'static str = "Worldwide";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Cryptocurrency;
    const IS_MAJOR: bool = true;
    const IS_STABLE: bool = true;
    const INTRODUCED_YEAR: u16 = 2018;
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
    fn test_usdc_currency_properties() {
        assert_eq!(USDC::DECIMALS, 6);
        assert_eq!(USDC::CODE, "USDC");
        assert_eq!(USDC::SYMBOL, "USDC");
    }

    #[test]
    fn test_usdc_amount_creation() {
        let amount = Amount::<USDC>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_usdc_amount_with_micro() {
        let amount = Amount::<USDC>::from_minor(1_000_000); // 1.000000 USDC
        assert_eq!(amount.to_major_floor(), 1);
        assert_eq!(amount.to_minor(), 1_000_000);
    }
}
