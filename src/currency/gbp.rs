//! British Pound Sterling currency implementation.

use super::Currency;

/// British Pound Sterling
///
/// # Example
///
/// ```
/// use typed_money::{Amount, GBP};
///
/// let amount = Amount::<GBP>::from_major(100);
/// println!("{}", amount);  // Displays: £100.00 GBP
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct GBP;

impl Currency for GBP {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "GBP";
    const SYMBOL: &'static str = "£";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gbp_constants() {
        assert_eq!(GBP::DECIMALS, 2);
        assert_eq!(GBP::CODE, "GBP");
        assert_eq!(GBP::SYMBOL, "£");
    }
}
