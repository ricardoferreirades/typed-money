use super::{Currency, CurrencyType, SymbolPosition, VolatilityRating, LiquidityRating};

/// Paraguayan Guarani (PYG)
///
/// The Paraguayan guarani is the currency of Paraguay.
/// It has no subdivisions.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, PYG};
///
/// let amount = Amount::<PYG>::from_major(100000);
/// assert_eq!(amount.to_major_floor(), 100000);
/// assert_eq!(PYG::CODE, "PYG");
/// assert_eq!(PYG::SYMBOL, "₲");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PYG;

impl Currency for PYG {
    const DECIMALS: u8 = 0; // Paraguayan Guarani has no subdivisions
    const CODE: &'static str = "PYG";
    const SYMBOL: &'static str = "₲";
    
    // Rich metadata
    const NAME: &'static str = "Paraguayan Guarani";
    const COUNTRY: &'static str = "Paraguay";
    const REGION: &'static str = "South America";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 1943;
    const ISO_4217_NUMBER: u16 = 600;
    const THOUSANDS_SEPARATOR: char = '.';
    const DECIMAL_SEPARATOR: char = ',';
    const SYMBOL_POSITION: SymbolPosition = SymbolPosition::Before;
    const SPACE_BETWEEN: bool = false;
    const VOLATILITY_RATING: VolatilityRating = VolatilityRating::High;
    const LIQUIDITY_RATING: LiquidityRating = LiquidityRating::Low;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_pyg_currency_properties() {
        assert_eq!(PYG::DECIMALS, 0);
        assert_eq!(PYG::CODE, "PYG");
        assert_eq!(PYG::SYMBOL, "₲");
    }

    #[test]
    fn test_pyg_amount_creation() {
        let amount = Amount::<PYG>::from_major(100000);
        assert_eq!(amount.to_major_floor(), 100000);
    }

    #[test]
    fn test_pyg_amount_with_minor() {
        let amount = Amount::<PYG>::from_minor(100000); // 100000 PYG (no decimals)
        assert_eq!(amount.to_major_floor(), 100000);
        assert_eq!(amount.to_minor(), 100000);
    }
}
