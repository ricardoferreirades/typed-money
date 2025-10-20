use crate::Currency;

/// Tether (USDT)
///
/// Tether is a cryptocurrency that is pegged to the US dollar. It is designed
/// to maintain a 1:1 ratio with the US dollar and is backed by reserves.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, USDT};
///
/// let amount = Amount::<USDT>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(USDT::CODE, "USDT");
/// assert_eq!(USDT::SYMBOL, "USDT");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct USDT;

impl Currency for USDT {
    const DECIMALS: u8 = 6;
    const CODE: &'static str = "USDT";
    const SYMBOL: &'static str = "USDT";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_usdt_currency_properties() {
        assert_eq!(USDT::DECIMALS, 6);
        assert_eq!(USDT::CODE, "USDT");
        assert_eq!(USDT::SYMBOL, "USDT");
    }

    #[test]
    fn test_usdt_amount_creation() {
        let amount = Amount::<USDT>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_usdt_amount_with_micro() {
        let amount = Amount::<USDT>::from_minor(1_000_000); // 1.000000 USDT
        assert_eq!(amount.to_major_floor(), 1);
        assert_eq!(amount.to_minor(), 1_000_000);
    }
}
