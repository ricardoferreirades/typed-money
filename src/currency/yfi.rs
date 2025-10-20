use super::{Currency, CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};

/// Yearn Finance (YFI)
///
/// Yearn Finance is a decentralized finance (DeFi) protocol that provides
/// yield farming and automated yield optimization strategies. The YFI token
/// is the governance token of the Yearn Finance ecosystem.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, YFI};
///
/// let amount = Amount::<YFI>::from_major(1);
/// assert_eq!(amount.to_major_floor(), 1);
/// assert_eq!(YFI::CODE, "YFI");
/// assert_eq!(YFI::SYMBOL, "YFI");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct YFI;

impl Currency for YFI {
    const DECIMALS: u8 = 18;
    const CODE: &'static str = "YFI";
    const SYMBOL: &'static str = "YFI";

    // Rich metadata
    const NAME: &'static str = "Yearn Finance";
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
    fn test_yfi_currency_properties() {
        assert_eq!(YFI::DECIMALS, 18);
        assert_eq!(YFI::CODE, "YFI");
        assert_eq!(YFI::SYMBOL, "YFI");
    }

    #[test]
    fn test_yfi_amount_creation() {
        let amount = Amount::<YFI>::from_major(1);
        assert_eq!(amount.to_major_floor(), 1);
    }

    #[test]
    fn test_yfi_amount_with_wei() {
        let amount = Amount::<YFI>::from_minor(1_000_000_000_000_000_000); // 1.000000000000000000 YFI
        assert_eq!(amount.to_major_floor(), 1);
        assert_eq!(amount.to_minor(), 1_000_000_000_000_000_000);
    }
}
