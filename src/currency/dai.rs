use super::{CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};
use crate::Currency;

/// Dai (DAI)
///
/// Dai is a stablecoin cryptocurrency that aims to maintain a value of $1 USD.
/// It is created and managed by the MakerDAO protocol and is backed by
/// collateral assets rather than fiat currency.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, DAI};
///
/// let amount = Amount::<DAI>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(DAI::CODE, "DAI");
/// assert_eq!(DAI::SYMBOL, "DAI");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DAI;

impl Currency for DAI {
    const DECIMALS: u8 = 18;
    const CODE: &'static str = "DAI";
    const SYMBOL: &'static str = "DAI";

    // Stablecoin metadata
    const NAME: &'static str = "Dai";
    const COUNTRY: &'static str = "Global";
    const REGION: &'static str = "Worldwide";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Cryptocurrency;
    const IS_MAJOR: bool = true;
    const IS_STABLE: bool = true;
    const INTRODUCED_YEAR: u16 = 2017;
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
    fn test_dai_currency_properties() {
        assert_eq!(DAI::DECIMALS, 18);
        assert_eq!(DAI::CODE, "DAI");
        assert_eq!(DAI::SYMBOL, "DAI");
    }

    #[test]
    fn test_dai_amount_creation() {
        let amount = Amount::<DAI>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_dai_amount_with_wei() {
        let amount = Amount::<DAI>::from_minor(1_000_000_000_000_000_000); // 1.000000000000000000 DAI
        assert_eq!(amount.to_major_floor(), 1);
        assert_eq!(amount.to_minor(), 1_000_000_000_000_000_000);
    }
}
