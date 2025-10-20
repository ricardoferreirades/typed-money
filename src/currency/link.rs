use super::{CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};
use crate::Currency;

/// Chainlink (LINK)
///
/// Chainlink is a decentralized oracle network that enables smart contracts
/// to securely access off-chain data feeds, web APIs, and traditional bank
/// payments.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, LINK};
///
/// let amount = Amount::<LINK>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(LINK::CODE, "LINK");
/// assert_eq!(LINK::SYMBOL, "LINK");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LINK;

impl Currency for LINK {
    const DECIMALS: u8 = 18;
    const CODE: &'static str = "LINK";
    const SYMBOL: &'static str = "LINK";

    // Cryptocurrency metadata
    const NAME: &'static str = "Chainlink";
    const COUNTRY: &'static str = "Global";
    const REGION: &'static str = "Worldwide";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Cryptocurrency;
    const IS_MAJOR: bool = true;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 2017;
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
    fn test_link_currency_properties() {
        assert_eq!(LINK::DECIMALS, 18);
        assert_eq!(LINK::CODE, "LINK");
        assert_eq!(LINK::SYMBOL, "LINK");
    }

    #[test]
    fn test_link_amount_creation() {
        let amount = Amount::<LINK>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_link_amount_with_wei() {
        let amount = Amount::<LINK>::from_minor(1_000_000_000_000_000_000); // 1.000000000000000000 LINK
        assert_eq!(amount.to_major_floor(), 1);
        assert_eq!(amount.to_minor(), 1_000_000_000_000_000_000);
    }
}
