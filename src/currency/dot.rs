use crate::Currency;

/// Polkadot (DOT)
///
/// Polkadot is a blockchain platform that allows different blockchains to
/// transfer messages and value in a trust-free fashion; sharing their unique
/// features while pooling their security.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, DOT};
///
/// let amount = Amount::<DOT>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(DOT::CODE, "DOT");
/// assert_eq!(DOT::SYMBOL, "DOT");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DOT;

impl Currency for DOT {
    const DECIMALS: u8 = 10;
    const CODE: &'static str = "DOT";
    const SYMBOL: &'static str = "DOT";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_dot_currency_properties() {
        assert_eq!(DOT::DECIMALS, 10);
        assert_eq!(DOT::CODE, "DOT");
        assert_eq!(DOT::SYMBOL, "DOT");
    }

    #[test]
    fn test_dot_amount_creation() {
        let amount = Amount::<DOT>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_dot_amount_with_planck() {
        let amount = Amount::<DOT>::from_minor(1_000_000_000); // 0.1000000000 DOT
        assert_eq!(amount.to_major_floor(), 0);
        assert_eq!(amount.to_minor(), 1_000_000_000);
    }
}
