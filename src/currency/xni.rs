//! Nickel (XNI) base metal implementation.

use super::Currency;

/// Nickel (XNI)
///
/// Nickel is traded in metric tons with 4 decimal places of precision.
/// This represents the standard trading unit for nickel in financial markets.
///
/// # Example
///
/// ```
/// use typed_money::{Amount, XNI};
///
/// let nickel = Amount::<XNI>::from_major(1);
/// println!("{}", nickel);  // Displays: 1.0000 XNI
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct XNI;

impl Currency for XNI {
    const DECIMALS: u8 = 4;
    const CODE: &'static str = "XNI";
    const SYMBOL: &'static str = "Ni";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xni_constants() {
        assert_eq!(XNI::DECIMALS, 4);
        assert_eq!(XNI::CODE, "XNI");
        assert_eq!(XNI::SYMBOL, "Ni");
    }

    #[test]
    fn test_currency_trait_properties() {
        // Verify XNI is Copy and Clone
        let xni1 = XNI;
        let xni2 = xni1; // Copy

        // Both should still be usable (proving Copy works)
        assert_eq!(XNI::CODE, "XNI");
        let _ = (xni1, xni2); // Use both to prove they're independent

        // Test Clone trait explicitly
        #[allow(clippy::clone_on_copy)]
        let _xni3 = xni1.clone(); // Explicitly test Clone trait
    }
}
