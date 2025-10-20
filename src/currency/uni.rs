use crate::Currency;

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
