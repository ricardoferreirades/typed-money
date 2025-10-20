//! Aluminum (XAL) base metal implementation.

use super::{Currency, CurrencyType, SymbolPosition, VolatilityRating, LiquidityRating};

/// Aluminum (XAL)
///
/// Aluminum is traded in metric tons with 4 decimal places of precision.
/// This represents the standard trading unit for aluminum in financial markets.
///
/// # Example
///
/// ```
/// use typed_money::{Amount, XAL};
///
/// let aluminum = Amount::<XAL>::from_major(1);
/// println!("{}", aluminum);  // Displays: 1.0000 XAL
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct XAL;

impl Currency for XAL {
    const DECIMALS: u8 = 4;
    const CODE: &'static str = "XAL";
    const SYMBOL: &'static str = "Al";
    
    // Commodity metadata
    const NAME: &'static str = "Aluminum";
    const COUNTRY: &'static str = "Global";
    const REGION: &'static str = "Worldwide";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Commodity;
    const IS_MAJOR: bool = true;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 0; // Aluminum has been used for centuries
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
    fn test_xal_constants() {
        assert_eq!(XAL::DECIMALS, 4);
        assert_eq!(XAL::CODE, "XAL");
        assert_eq!(XAL::SYMBOL, "Al");
    }

    #[test]
    fn test_currency_trait_properties() {
        // Verify XAL is Copy and Clone
        let xal1 = XAL;
        let xal2 = xal1; // Copy

        // Both should still be usable (proving Copy works)
        assert_eq!(XAL::CODE, "XAL");
        let _ = (xal1, xal2); // Use both to prove they're independent

        // Test Clone trait explicitly
        #[allow(clippy::clone_on_copy)]
        let _xal3 = xal1.clone(); // Explicitly test Clone trait
    }
}
