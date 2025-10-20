//! Palladium (XPD) precious metal implementation.

use super::Currency;

/// Palladium (XPD)
///
/// Palladium is traded in troy ounces with 4 decimal places of precision.
/// This represents the standard trading unit for palladium in financial markets.
///
/// # Example
///
/// ```
/// use typed_money::{Amount, XPD};
///
/// let palladium = Amount::<XPD>::from_major(1);
/// println!("{}", palladium);  // Displays: 1.0000 XPD
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct XPD;

impl Currency for XPD {
    const DECIMALS: u8 = 4;
    const CODE: &'static str = "XPD";
    const SYMBOL: &'static str = "Pd";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xpd_constants() {
        assert_eq!(XPD::DECIMALS, 4);
        assert_eq!(XPD::CODE, "XPD");
        assert_eq!(XPD::SYMBOL, "Pd");
    }

    #[test]
    fn test_currency_trait_properties() {
        // Verify XPD is Copy and Clone
        let xpd1 = XPD;
        let xpd2 = xpd1; // Copy

        // Both should still be usable (proving Copy works)
        assert_eq!(XPD::CODE, "XPD");
        let _ = (xpd1, xpd2); // Use both to prove they're independent

        // Test Clone trait explicitly
        #[allow(clippy::clone_on_copy)]
        let _xpd3 = xpd1.clone(); // Explicitly test Clone trait
    }
}
