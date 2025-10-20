use crate::Currency;

/// Chainlink (LINK)
///
/// Chainlink is a decentralized oracle network that enables smart contracts
/// to securely access off-chain data feeds, web APIs, and traditional bank
/// payments.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, LINK};
///
/// let amount = Amount::<LINK>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(LINK::CODE, "LINK");
/// assert_eq!(LINK::SYMBOL, "LINK");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct LINK;

impl Currency for LINK {
    const DECIMALS: u8 = 18;
    const CODE: &'static str = "LINK";
    const SYMBOL: &'static str = "LINK";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_link_currency_properties() {
        assert_eq!(LINK::DECIMALS, 18);
        assert_eq!(LINK::CODE, "LINK");
        assert_eq!(LINK::SYMBOL, "LINK");
    }

    #[test]
    fn test_link_amount_creation() {
        let amount = Amount::<LINK>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_link_amount_with_wei() {
        let amount = Amount::<LINK>::from_minor(1_000_000_000_000_000_000); // 1.000000000000000000 LINK
        assert_eq!(amount.to_major_floor(), 1);
        assert_eq!(amount.to_minor(), 1_000_000_000_000_000_000);
    }
}
