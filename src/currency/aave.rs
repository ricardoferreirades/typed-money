use crate::Currency;

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
