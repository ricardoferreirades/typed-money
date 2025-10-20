//! Platinum (XPT) precious metal implementation.

use super::{Currency, CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};

/// Platinum (XPT)
///
/// Platinum is traded in troy ounces with 4 decimal places of precision.
/// This represents the standard trading unit for platinum in financial markets.
///
/// # Example
///
/// ```
/// use typed_money::{Amount, XPT};
///
/// let platinum = Amount::<XPT>::from_major(1);
/// println!("{}", platinum);  // Displays: 1.0000 XPT
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct XPT;

impl Currency for XPT {
    const DECIMALS: u8 = 4;
    const CODE: &'static str = "XPT";
    const SYMBOL: &'static str = "Pt";

    // Commodity metadata
    const NAME: &'static str = "Platinum";
    const COUNTRY: &'static str = "Global";
    const REGION: &'static str = "Worldwide";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Commodity;
    const IS_MAJOR: bool = true;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 0; // Platinum has been used for centuries
    const ISO_4217_NUMBER: u16 = 962; // ISO 4217 code for platinum
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
    fn test_xpt_constants() {
        assert_eq!(XPT::DECIMALS, 4);
        assert_eq!(XPT::CODE, "XPT");
        assert_eq!(XPT::SYMBOL, "Pt");
    }

    #[test]
    fn test_currency_trait_properties() {
        // Verify XPT is Copy and Clone
        let xpt1 = XPT;
        let xpt2 = xpt1; // Copy

        // Both should still be usable (proving Copy works)
        assert_eq!(XPT::CODE, "XPT");
        let _ = (xpt1, xpt2); // Use both to prove they're independent

        // Test Clone trait explicitly
        #[allow(clippy::clone_on_copy)]
        let _xpt3 = xpt1.clone(); // Explicitly test Clone trait
    }
}
