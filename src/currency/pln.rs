use crate::Currency;
use super::{CurrencyType, SymbolPosition, VolatilityRating, LiquidityRating};

/// Polish Złoty (PLN)
///
/// The złoty is the official currency and legal tender of Poland.
/// It is subdivided into 100 groszy.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, PLN};
///
/// let amount = Amount::<PLN>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(PLN::CODE, "PLN");
/// assert_eq!(PLN::SYMBOL, "zł");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PLN;

impl Currency for PLN {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "PLN";
    const SYMBOL: &'static str = "zł";
    
    // Rich metadata
    const NAME: &'static str = "Polish Złoty";
    const COUNTRY: &'static str = "Poland";
    const REGION: &'static str = "Europe";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 1995;
    const ISO_4217_NUMBER: u16 = 985;
    const THOUSANDS_SEPARATOR: char = ' ';
    const DECIMAL_SEPARATOR: char = ',';
    const SYMBOL_POSITION: SymbolPosition = SymbolPosition::After;
    const SPACE_BETWEEN: bool = true;
    const VOLATILITY_RATING: VolatilityRating = VolatilityRating::Medium;
    const LIQUIDITY_RATING: LiquidityRating = LiquidityRating::Medium;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_pln_currency_properties() {
        assert_eq!(PLN::DECIMALS, 2);
        assert_eq!(PLN::CODE, "PLN");
        assert_eq!(PLN::SYMBOL, "zł");
    }

    #[test]
    fn test_pln_amount_creation() {
        let amount = Amount::<PLN>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_pln_amount_with_groszy() {
        let amount = Amount::<PLN>::from_minor(10050); // 100.50 PLN
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
