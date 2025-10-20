use super::{Currency, CurrencyType, SymbolPosition, VolatilityRating, LiquidityRating};

/// Compound (COMP)
///
/// Compound is a decentralized finance (DeFi) protocol that allows users to
/// earn interest on their cryptocurrency holdings or borrow against them.
/// The COMP token is the governance token of the Compound protocol.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, COMP};
///
/// let amount = Amount::<COMP>::from_major(10);
/// assert_eq!(amount.to_major_floor(), 10);
/// assert_eq!(COMP::CODE, "COMP");
/// assert_eq!(COMP::SYMBOL, "COMP");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct COMP;

impl Currency for COMP {
    const DECIMALS: u8 = 18;
    const CODE: &'static str = "COMP";
    const SYMBOL: &'static str = "COMP";
    
    // Rich metadata
    const NAME: &'static str = "Compound";
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
    fn test_comp_currency_properties() {
        assert_eq!(COMP::DECIMALS, 18);
        assert_eq!(COMP::CODE, "COMP");
        assert_eq!(COMP::SYMBOL, "COMP");
    }

    #[test]
    fn test_comp_amount_creation() {
        let amount = Amount::<COMP>::from_major(10);
        assert_eq!(amount.to_major_floor(), 10);
    }

    #[test]
    fn test_comp_amount_with_wei() {
        let amount = Amount::<COMP>::from_minor(1_000_000_000_000_000_000); // 1.000000000000000000 COMP
        assert_eq!(amount.to_major_floor(), 1);
        assert_eq!(amount.to_minor(), 1_000_000_000_000_000_000);
    }
}
