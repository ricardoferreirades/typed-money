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
