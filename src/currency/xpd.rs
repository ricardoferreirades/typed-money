//! Palladium (XPD) precious metal implementation.

use super::{Currency, CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};

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

    // Commodity metadata
    const NAME: &'static str = "Palladium";
    const COUNTRY: &'static str = "Global";
    const REGION: &'static str = "Worldwide";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Commodity;
    const IS_MAJOR: bool = true;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 0; // Palladium has been used for centuries
    const ISO_4217_NUMBER: u16 = 964; // ISO 4217 code for palladium
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
