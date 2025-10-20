use crate::Currency;

/// XRP (XRP)
///
/// XRP is the native cryptocurrency of the XRP Ledger, a decentralized
/// blockchain technology. It is designed to be a fast, efficient, and
/// scalable digital asset for payments.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, XRP};
///
/// let amount = Amount::<XRP>::from_major(1000);
/// assert_eq!(amount.to_major_floor(), 1000);
/// assert_eq!(XRP::CODE, "XRP");
/// assert_eq!(XRP::SYMBOL, "XRP");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct XRP;

impl Currency for XRP {
    const DECIMALS: u8 = 6;
    const CODE: &'static str = "XRP";
    const SYMBOL: &'static str = "XRP";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_xrp_currency_properties() {
        assert_eq!(XRP::DECIMALS, 6);
        assert_eq!(XRP::CODE, "XRP");
        assert_eq!(XRP::SYMBOL, "XRP");
    }

    #[test]
    fn test_xrp_amount_creation() {
        let amount = Amount::<XRP>::from_major(1000);
        assert_eq!(amount.to_major_floor(), 1000);
    }

    #[test]
    fn test_xrp_amount_with_drops() {
        let amount = Amount::<XRP>::from_minor(1_000_000); // 1.000000 XRP
        assert_eq!(amount.to_major_floor(), 1);
        assert_eq!(amount.to_minor(), 1_000_000);
    }
}
