//! Ethereum currency implementation.

use super::{Currency, CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};

/// Ethereum
///
/// # Example
///
/// ```
/// use typed_money::{Amount, ETH};
///
/// let amount = Amount::<ETH>::from_major(1);
/// println!("{}", amount);  // Displays: Ξ1.000000000000000000 ETH
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ETH;

impl Currency for ETH {
    const DECIMALS: u8 = 18;
    const CODE: &'static str = "ETH";
    const SYMBOL: &'static str = "Ξ";

    // Cryptocurrency metadata
    const NAME: &'static str = "Ethereum";
    const COUNTRY: &'static str = "Global";
    const REGION: &'static str = "Worldwide";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Cryptocurrency;
    const IS_MAJOR: bool = true;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 2015;
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

    #[test]
    fn test_eth_constants() {
        assert_eq!(ETH::DECIMALS, 18);
        assert_eq!(ETH::CODE, "ETH");
        assert_eq!(ETH::SYMBOL, "Ξ");
    }
}
