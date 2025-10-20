use crate::Currency;
use super::{CurrencyType, SymbolPosition, VolatilityRating, LiquidityRating};

/// Uniswap (UNI)
///
/// Uniswap is a decentralized cryptocurrency exchange that uses a set of
/// smart contracts to enable automated trading of decentralized finance (DeFi) tokens.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, UNI};
///
/// let amount = Amount::<UNI>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(UNI::CODE, "UNI");
/// assert_eq!(UNI::SYMBOL, "UNI");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UNI;

impl Currency for UNI {
    const DECIMALS: u8 = 18;
    const CODE: &'static str = "UNI";
    const SYMBOL: &'static str = "UNI";
    
    // Cryptocurrency metadata
    const NAME: &'static str = "Uniswap";
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
    fn test_uni_currency_properties() {
        assert_eq!(UNI::DECIMALS, 18);
        assert_eq!(UNI::CODE, "UNI");
        assert_eq!(UNI::SYMBOL, "UNI");
    }

    #[test]
    fn test_uni_amount_creation() {
        let amount = Amount::<UNI>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_uni_amount_with_wei() {
        let amount = Amount::<UNI>::from_minor(1_000_000_000_000_000_000); // 1.000000000000000000 UNI
        assert_eq!(amount.to_major_floor(), 1);
        assert_eq!(amount.to_minor(), 1_000_000_000_000_000_000);
    }
}
