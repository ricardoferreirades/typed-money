//! Diamond (XDI) precious metal implementation.

use super::Currency;

/// Diamond (XDI)
///
/// Diamond is traded in carats with 4 decimal places of precision.
/// This represents the standard trading unit for diamond in financial markets.
///
/// # Example
///
/// ```
/// use typed_money::{Amount, XDI};
///
/// let diamond = Amount::<XDI>::from_major(1);
/// println!("{}", diamond);  // Displays: 1.0000 XDI
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct XDI;

impl Currency for XDI {
    const DECIMALS: u8 = 4;
    const CODE: &'static str = "XDI";
    const SYMBOL: &'static str = "♦";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xdi_constants() {
        assert_eq!(XDI::DECIMALS, 4);
        assert_eq!(XDI::CODE, "XDI");
        assert_eq!(XDI::SYMBOL, "♦");
    }

    #[test]
    fn test_currency_trait_properties() {
        // Verify XDI is Copy and Clone
        let xdi1 = XDI;
        let xdi2 = xdi1; // Copy

        // Both should still be usable (proving Copy works)
        assert_eq!(XDI::CODE, "XDI");
        let _ = (xdi1, xdi2); // Use both to prove they're independent

        // Test Clone trait explicitly
        #[allow(clippy::clone_on_copy)]
        let _xdi3 = xdi1.clone(); // Explicitly test Clone trait
    }
}
