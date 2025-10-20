//! Gold (XAU) precious metal implementation.

use super::{Currency, CurrencyType, SymbolPosition, VolatilityRating, LiquidityRating};

/// Gold (XAU)
///
/// Gold is traded in troy ounces with 4 decimal places of precision.
/// This represents the standard trading unit for gold in financial markets.
///
/// # Example
///
/// ```
/// use typed_money::{Amount, XAU};
///
/// let gold = Amount::<XAU>::from_major(1);
/// println!("{}", gold);  // Displays: 1.0000 XAU
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct XAU;

impl Currency for XAU {
    const DECIMALS: u8 = 4;
    const CODE: &'static str = "XAU";
    const SYMBOL: &'static str = "Au";
    
    // Commodity metadata
    const NAME: &'static str = "Gold";
    const COUNTRY: &'static str = "Global";
    const REGION: &'static str = "Worldwide";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Commodity;
    const IS_MAJOR: bool = true;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 0; // Gold has been used for millennia
    const ISO_4217_NUMBER: u16 = 959; // ISO 4217 code for gold
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
    fn test_xau_constants() {
        assert_eq!(XAU::DECIMALS, 4);
        assert_eq!(XAU::CODE, "XAU");
        assert_eq!(XAU::SYMBOL, "Au");
    }

    #[test]
    fn test_currency_trait_properties() {
        // Verify XAU is Copy and Clone
        let xau1 = XAU;
        let xau2 = xau1; // Copy

        // Both should still be usable (proving Copy works)
        assert_eq!(XAU::CODE, "XAU");
        let _ = (xau1, xau2); // Use both to prove they're independent

        // Test Clone trait explicitly
        #[allow(clippy::clone_on_copy)]
        let _xau3 = xau1.clone(); // Explicitly test Clone trait
    }
}
