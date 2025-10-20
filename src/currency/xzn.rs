//! Zinc (XZN) base metal implementation.

use super::{Currency, CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};

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

    // Commodity metadata
    const NAME: &'static str = "Zinc";
    const COUNTRY: &'static str = "Global";
    const REGION: &'static str = "Worldwide";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Commodity;
    const IS_MAJOR: bool = true;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 0; // Zinc has been used for centuries
    const ISO_4217_NUMBER: u16 = 0; // No official ISO code for base metals
    const THOUSANDS_SEPARATOR: char = ',';
    const DECIMAL_SEPARATOR: char = '.';
    const SYMBOL_POSITION: SymbolPosition = SymbolPosition::Before;
    const SPACE_BETWEEN: bool = false;
    const VOLATILITY_RATING: VolatilityRating = VolatilityRating::Medium;
    const LIQUIDITY_RATING: LiquidityRating = LiquidityRating::High;
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
