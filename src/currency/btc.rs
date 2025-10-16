//! Bitcoin currency implementation.

use super::Currency;

/// Bitcoin
///
/// # Example
///
/// ```
/// use typed_money::{Amount, BTC};
///
/// let amount = Amount::<BTC>::from_major(1);
/// println!("{}", amount);  // Displays: ₿1.00000000 BTC
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BTC;

impl Currency for BTC {
    const DECIMALS: u8 = 8;
    const CODE: &'static str = "BTC";
    const SYMBOL: &'static str = "₿";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_btc_constants() {
        assert_eq!(BTC::DECIMALS, 8);
        assert_eq!(BTC::CODE, "BTC");
        assert_eq!(BTC::SYMBOL, "₿");
    }
}
