use crate::Currency;

/// Binance USD (BUSD)
///
/// Binance USD is a USD-backed stablecoin issued by Binance in partnership
/// with Paxos. It is designed to maintain a 1:1 ratio with the US dollar
/// and is regulated by the New York State Department of Financial Services.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, BUSD};
///
/// let amount = Amount::<BUSD>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(BUSD::CODE, "BUSD");
/// assert_eq!(BUSD::SYMBOL, "BUSD");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BUSD;

impl Currency for BUSD {
    const DECIMALS: u8 = 18;
    const CODE: &'static str = "BUSD";
    const SYMBOL: &'static str = "BUSD";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_busd_currency_properties() {
        assert_eq!(BUSD::DECIMALS, 18);
        assert_eq!(BUSD::CODE, "BUSD");
        assert_eq!(BUSD::SYMBOL, "BUSD");
    }

    #[test]
    fn test_busd_amount_creation() {
        let amount = Amount::<BUSD>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_busd_amount_with_wei() {
        let amount = Amount::<BUSD>::from_minor(1_000_000_000_000_000_000); // 1.000000000000000000 BUSD
        assert_eq!(amount.to_major_floor(), 1);
        assert_eq!(amount.to_minor(), 1_000_000_000_000_000_000);
    }
}
