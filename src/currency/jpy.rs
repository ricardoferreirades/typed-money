//! Japanese Yen currency implementation.

use super::Currency;

/// Japanese Yen
///
/// # Example
///
/// ```
/// use typed_money::{Amount, JPY};
///
/// let amount = Amount::<JPY>::from_major(1000);
/// println!("{}", amount);  // Displays: ¥1000 JPY
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct JPY;

impl Currency for JPY {
    const DECIMALS: u8 = 0;
    const CODE: &'static str = "JPY";
    const SYMBOL: &'static str = "¥";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jpy_constants() {
        assert_eq!(JPY::DECIMALS, 0);
        assert_eq!(JPY::CODE, "JPY");
        assert_eq!(JPY::SYMBOL, "¥");
    }
}
