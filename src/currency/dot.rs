use super::{CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};
use crate::Currency;

/// Polkadot (DOT)
///
/// Polkadot is a blockchain platform that allows different blockchains to
/// transfer messages and value in a trust-free fashion; sharing their unique
/// features while pooling their security.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, DOT};
///
/// let amount = Amount::<DOT>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(DOT::CODE, "DOT");
/// assert_eq!(DOT::SYMBOL, "DOT");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DOT;

impl Currency for DOT {
    const DECIMALS: u8 = 10;
    const CODE: &'static str = "DOT";
    const SYMBOL: &'static str = "DOT";

    // Cryptocurrency metadata
    const NAME: &'static str = "Polkadot";
    const COUNTRY: &'static str = "Global";
    const REGION: &'static str = "Worldwide";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Cryptocurrency;
    const IS_MAJOR: bool = true;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 2020;
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
    fn test_dot_currency_properties() {
        assert_eq!(DOT::DECIMALS, 10);
        assert_eq!(DOT::CODE, "DOT");
        assert_eq!(DOT::SYMBOL, "DOT");
    }

    #[test]
    fn test_dot_amount_creation() {
        let amount = Amount::<DOT>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_dot_amount_with_planck() {
        let amount = Amount::<DOT>::from_minor(1_000_000_000); // 0.1000000000 DOT
        assert_eq!(amount.to_major_floor(), 0);
        assert_eq!(amount.to_minor(), 1_000_000_000);
    }
}
