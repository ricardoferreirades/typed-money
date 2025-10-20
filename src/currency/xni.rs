//! Nickel (XNI) base metal implementation.

use super::{Currency, CurrencyType, SymbolPosition, VolatilityRating, LiquidityRating};

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
    
    // Commodity metadata
    const NAME: &'static str = "Nickel";
    const COUNTRY: &'static str = "Global";
    const REGION: &'static str = "Worldwide";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Commodity;
    const IS_MAJOR: bool = true;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 0; // Nickel has been used for centuries
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
