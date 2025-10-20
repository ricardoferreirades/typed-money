//! Zinc (XZN) base metal implementation.

use super::Currency;

/// Zinc (XZN)
///
/// Zinc is traded in metric tons with 4 decimal places of precision.
/// This represents the standard trading unit for zinc in financial markets.
///
/// # Example
///
/// ```
/// use typed_money::{Amount, XZN};
///
/// let zinc = Amount::<XZN>::from_major(1);
/// println!("{}", zinc);  // Displays: 1.0000 XZN
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct XZN;

impl Currency for XZN {
    const DECIMALS: u8 = 4;
    const CODE: &'static str = "XZN";
    const SYMBOL: &'static str = "Zn";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xzn_constants() {
        assert_eq!(XZN::DECIMALS, 4);
        assert_eq!(XZN::CODE, "XZN");
        assert_eq!(XZN::SYMBOL, "Zn");
    }

    #[test]
    fn test_currency_trait_properties() {
        // Verify XZN is Copy and Clone
        let xzn1 = XZN;
        let xzn2 = xzn1; // Copy

        // Both should still be usable (proving Copy works)
        assert_eq!(XZN::CODE, "XZN");
        let _ = (xzn1, xzn2); // Use both to prove they're independent

        // Test Clone trait explicitly
        #[allow(clippy::clone_on_copy)]
        let _xzn3 = xzn1.clone(); // Explicitly test Clone trait
    }
}
