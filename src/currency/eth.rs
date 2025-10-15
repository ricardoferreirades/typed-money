//! Ethereum currency implementation.

use super::Currency;

/// Ethereum
///
/// # Example
///
/// ```
/// use typed_money::{Amount, ETH};
///
/// let amount = Amount::<ETH>::from_major(1);
/// println!("{}", amount);  // Displays: Ξ1.000000000000000000 ETH
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ETH;

impl Currency for ETH {
    const DECIMALS: u8 = 18;
    const CODE: &'static str = "ETH";
    const SYMBOL: &'static str = "Ξ";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eth_constants() {
        assert_eq!(ETH::DECIMALS, 18);
        assert_eq!(ETH::CODE, "ETH");
        assert_eq!(ETH::SYMBOL, "Ξ");
    }
}
