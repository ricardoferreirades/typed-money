//! United States Dollar currency implementation.

use super::Currency;

/// United States Dollar
///
/// # Example
///
/// ```
/// use typed_money::{Amount, USD};
///
/// let amount = Amount::<USD>::from_major(100);
/// println!("{}", amount);  // Displays: $100.00 USD
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct USD;

impl Currency for USD {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "USD";
    const SYMBOL: &'static str = "$";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usd_constants() {
        assert_eq!(USD::DECIMALS, 2);
        assert_eq!(USD::CODE, "USD");
        assert_eq!(USD::SYMBOL, "$");
    }

    #[test]
    fn test_currency_trait_properties() {
        // Verify USD is Copy and Clone
        let usd1 = USD;
        let usd2 = usd1; // Copy

        // Both should still be usable (proving Copy works)
        assert_eq!(USD::CODE, "USD");
        let _ = (usd1, usd2); // Use both to prove they're independent

        // Test Clone trait explicitly
        #[allow(clippy::clone_on_copy)]
        let _usd3 = usd1.clone(); // Explicitly test Clone trait
    }
}
