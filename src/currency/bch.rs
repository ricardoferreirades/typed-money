use super::{CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};
use crate::Currency;

/// Bitcoin Cash (BCH)
///
/// Bitcoin Cash is a cryptocurrency that is a fork of Bitcoin. It was created
/// to address Bitcoin's scalability issues by increasing the block size limit,
/// allowing for more transactions per block.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, BCH};
///
/// let amount = Amount::<BCH>::from_major(1);
/// assert_eq!(amount.to_major_floor(), 1);
/// assert_eq!(BCH::CODE, "BCH");
/// assert_eq!(BCH::SYMBOL, "₿");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BCH;

impl Currency for BCH {
    const DECIMALS: u8 = 8;
    const CODE: &'static str = "BCH";
    const SYMBOL: &'static str = "₿";

    // Cryptocurrency metadata
    const NAME: &'static str = "Bitcoin Cash";
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
    fn test_bch_currency_properties() {
        assert_eq!(BCH::DECIMALS, 8);
        assert_eq!(BCH::CODE, "BCH");
        assert_eq!(BCH::SYMBOL, "₿");
    }

    #[test]
    fn test_bch_amount_creation() {
        let amount = Amount::<BCH>::from_major(1);
        assert_eq!(amount.to_major_floor(), 1);
    }

    #[test]
    fn test_bch_amount_with_satoshis() {
        let amount = Amount::<BCH>::from_minor(100_000_000); // 1.00000000 BCH
        assert_eq!(amount.to_major_floor(), 1);
        assert_eq!(amount.to_minor(), 100_000_000);
    }
}
