//! Copper (XCU) base metal implementation.

use super::Currency;

/// Copper (XCU)
///
/// Copper is traded in metric tons with 4 decimal places of precision.
/// This represents the standard trading unit for copper in financial markets.
///
/// # Example
///
/// ```
/// use typed_money::{Amount, XCU};
///
/// let copper = Amount::<XCU>::from_major(1);
/// println!("{}", copper);  // Displays: 1.0000 XCU
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct XCU;

impl Currency for XCU {
    const DECIMALS: u8 = 4;
    const CODE: &'static str = "XCU";
    const SYMBOL: &'static str = "Cu";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xcu_constants() {
        assert_eq!(XCU::DECIMALS, 4);
        assert_eq!(XCU::CODE, "XCU");
        assert_eq!(XCU::SYMBOL, "Cu");
    }

    #[test]
    fn test_currency_trait_properties() {
        // Verify XCU is Copy and Clone
        let xcu1 = XCU;
        let xcu2 = xcu1; // Copy

        // Both should still be usable (proving Copy works)
        assert_eq!(XCU::CODE, "XCU");
        let _ = (xcu1, xcu2); // Use both to prove they're independent

        // Test Clone trait explicitly
        #[allow(clippy::clone_on_copy)]
        let _xcu3 = xcu1.clone(); // Explicitly test Clone trait
    }
}
