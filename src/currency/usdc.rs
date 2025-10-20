use crate::Currency;

/// USD Coin (USDC)
///
/// USD Coin is a fully-backed U.S. dollar stablecoin. It is issued by Centre,
/// a consortium that includes Coinbase and Circle, and is designed to maintain
/// a 1:1 ratio with the US dollar.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, USDC};
///
/// let amount = Amount::<USDC>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(USDC::CODE, "USDC");
/// assert_eq!(USDC::SYMBOL, "USDC");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct USDC;

impl Currency for USDC {
    const DECIMALS: u8 = 6;
    const CODE: &'static str = "USDC";
    const SYMBOL: &'static str = "USDC";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_usdc_currency_properties() {
        assert_eq!(USDC::DECIMALS, 6);
        assert_eq!(USDC::CODE, "USDC");
        assert_eq!(USDC::SYMBOL, "USDC");
    }

    #[test]
    fn test_usdc_amount_creation() {
        let amount = Amount::<USDC>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_usdc_amount_with_micro() {
        let amount = Amount::<USDC>::from_minor(1_000_000); // 1.000000 USDC
        assert_eq!(amount.to_major_floor(), 1);
        assert_eq!(amount.to_minor(), 1_000_000);
    }
}
