//! Silver (XAG) precious metal implementation.

use super::{Currency, CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};

/// Silver (XAG)
///
/// Silver is traded in troy ounces with 4 decimal places of precision.
/// This represents the standard trading unit for silver in financial markets.
///
/// # Example
///
/// ```
/// use typed_money::{Amount, XAG};
///
/// let silver = Amount::<XAG>::from_major(1);
/// println!("{}", silver);  // Displays: 1.0000 XAG
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct XAG;

impl Currency for XAG {
    const DECIMALS: u8 = 4;
    const CODE: &'static str = "XAG";
    const SYMBOL: &'static str = "Ag";

    // Commodity metadata
    const NAME: &'static str = "Silver";
    const COUNTRY: &'static str = "Global";
    const REGION: &'static str = "Worldwide";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Commodity;
    const IS_MAJOR: bool = true;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 0; // Silver has been used for millennia
    const ISO_4217_NUMBER: u16 = 961; // ISO 4217 code for silver
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
    fn test_xag_constants() {
        assert_eq!(XAG::DECIMALS, 4);
        assert_eq!(XAG::CODE, "XAG");
        assert_eq!(XAG::SYMBOL, "Ag");
    }

    #[test]
    fn test_currency_trait_properties() {
        // Verify XAG is Copy and Clone
        let xag1 = XAG;
        let xag2 = xag1; // Copy

        // Both should still be usable (proving Copy works)
        assert_eq!(XAG::CODE, "XAG");
        let _ = (xag1, xag2); // Use both to prove they're independent

        // Test Clone trait explicitly
        #[allow(clippy::clone_on_copy)]
        let _xag3 = xag1.clone(); // Explicitly test Clone trait
    }
}
