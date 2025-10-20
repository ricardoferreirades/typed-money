use crate::Currency;
use super::{CurrencyType, SymbolPosition, VolatilityRating, LiquidityRating};

/// Aave (AAVE)
///
/// Aave is a decentralized finance protocol that allows people to lend and
/// borrow cryptocurrencies. It is an open-source and non-custodial liquidity
/// protocol for earning interest on deposits and borrowing assets.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, AAVE};
///
/// let amount = Amount::<AAVE>::from_major(10);
/// assert_eq!(amount.to_major_floor(), 10);
/// assert_eq!(AAVE::CODE, "AAVE");
/// assert_eq!(AAVE::SYMBOL, "AAVE");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AAVE;

impl Currency for AAVE {
    const DECIMALS: u8 = 18;
    const CODE: &'static str = "AAVE";
    const SYMBOL: &'static str = "AAVE";
    
    // Cryptocurrency metadata
    const NAME: &'static str = "Aave";
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
    fn test_aave_currency_properties() {
        assert_eq!(AAVE::DECIMALS, 18);
        assert_eq!(AAVE::CODE, "AAVE");
        assert_eq!(AAVE::SYMBOL, "AAVE");
    }

    #[test]
    fn test_aave_amount_creation() {
        let amount = Amount::<AAVE>::from_major(10);
        assert_eq!(amount.to_major_floor(), 10);
    }

    #[test]
    fn test_aave_amount_with_wei() {
        let amount = Amount::<AAVE>::from_minor(1_000_000_000_000_000_000); // 1.000000000000000000 AAVE
        assert_eq!(amount.to_major_floor(), 1);
        assert_eq!(amount.to_minor(), 1_000_000_000_000_000_000);
    }
}
