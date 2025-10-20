use super::{Currency, CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};

/// SushiSwap (SUSHI)
///
/// SushiSwap is a decentralized exchange (DEX) and automated market maker
/// (AMM) protocol built on Ethereum. The SUSHI token is the governance token
/// of the SushiSwap ecosystem.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, SUSHI};
///
/// let amount = Amount::<SUSHI>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(SUSHI::CODE, "SUSHI");
/// assert_eq!(SUSHI::SYMBOL, "SUSHI");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SUSHI;

impl Currency for SUSHI {
    const DECIMALS: u8 = 18;
    const CODE: &'static str = "SUSHI";
    const SYMBOL: &'static str = "SUSHI";

    // Rich metadata
    const NAME: &'static str = "SushiSwap";
    const COUNTRY: &'static str = "Global";
    const REGION: &'static str = "Worldwide";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Cryptocurrency;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 2020;
    const ISO_4217_NUMBER: u16 = 0; // Not an ISO currency
    const THOUSANDS_SEPARATOR: char = ',';
    const DECIMAL_SEPARATOR: char = '.';
    const SYMBOL_POSITION: SymbolPosition = SymbolPosition::After;
    const SPACE_BETWEEN: bool = true;
    const VOLATILITY_RATING: VolatilityRating = VolatilityRating::High;
    const LIQUIDITY_RATING: LiquidityRating = LiquidityRating::Medium;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_sushi_currency_properties() {
        assert_eq!(SUSHI::DECIMALS, 18);
        assert_eq!(SUSHI::CODE, "SUSHI");
        assert_eq!(SUSHI::SYMBOL, "SUSHI");
    }

    #[test]
    fn test_sushi_amount_creation() {
        let amount = Amount::<SUSHI>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_sushi_amount_with_wei() {
        let amount = Amount::<SUSHI>::from_minor(1_000_000_000_000_000_000); // 1.000000000000000000 SUSHI
        assert_eq!(amount.to_major_floor(), 1);
        assert_eq!(amount.to_minor(), 1_000_000_000_000_000_000);
    }
}
