use super::{Currency, CurrencyType, SymbolPosition, VolatilityRating, LiquidityRating};

/// Bolivian Boliviano (BOB)
///
/// The Bolivian boliviano is the currency of Bolivia.
/// It is subdivided into 100 centavos.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, BOB};
///
/// let amount = Amount::<BOB>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(BOB::CODE, "BOB");
/// assert_eq!(BOB::SYMBOL, "Bs");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BOB;

impl Currency for BOB {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "BOB";
    const SYMBOL: &'static str = "Bs";
    
    // Rich metadata
    const NAME: &'static str = "Bolivian Boliviano";
    const COUNTRY: &'static str = "Bolivia";
    const REGION: &'static str = "South America";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 1987;
    const ISO_4217_NUMBER: u16 = 068;
    const THOUSANDS_SEPARATOR: char = '.';
    const DECIMAL_SEPARATOR: char = ',';
    const SYMBOL_POSITION: SymbolPosition = SymbolPosition::Before;
    const SPACE_BETWEEN: bool = false;
    const VOLATILITY_RATING: VolatilityRating = VolatilityRating::Medium;
    const LIQUIDITY_RATING: LiquidityRating = LiquidityRating::Low;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_bob_currency_properties() {
        assert_eq!(BOB::DECIMALS, 2);
        assert_eq!(BOB::CODE, "BOB");
        assert_eq!(BOB::SYMBOL, "Bs");
    }

    #[test]
    fn test_bob_amount_creation() {
        let amount = Amount::<BOB>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_bob_amount_with_centavos() {
        let amount = Amount::<BOB>::from_minor(10050); // 100.50 BOB
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
