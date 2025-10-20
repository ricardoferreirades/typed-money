use crate::Currency;
use super::{CurrencyType, SymbolPosition, VolatilityRating, LiquidityRating};

/// Ukrainian Hryvnia (UAH)
///
/// The Ukrainian hryvnia is the currency of Ukraine.
/// It is subdivided into 100 kopiyok.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, UAH};
///
/// let amount = Amount::<UAH>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(UAH::CODE, "UAH");
/// assert_eq!(UAH::SYMBOL, "₴");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UAH;

impl Currency for UAH {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "UAH";
    const SYMBOL: &'static str = "₴";
    
    // Rich metadata
    const NAME: &'static str = "Ukrainian Hryvnia";
    const COUNTRY: &'static str = "Ukraine";
    const REGION: &'static str = "Europe";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 1996;
    const ISO_4217_NUMBER: u16 = 980;
    const THOUSANDS_SEPARATOR: char = ' ';
    const DECIMAL_SEPARATOR: char = ',';
    const SYMBOL_POSITION: SymbolPosition = SymbolPosition::After;
    const SPACE_BETWEEN: bool = true;
    const VOLATILITY_RATING: VolatilityRating = VolatilityRating::High;
    const LIQUIDITY_RATING: LiquidityRating = LiquidityRating::Low;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_uah_currency_properties() {
        assert_eq!(UAH::DECIMALS, 2);
        assert_eq!(UAH::CODE, "UAH");
        assert_eq!(UAH::SYMBOL, "₴");
    }

    #[test]
    fn test_uah_amount_creation() {
        let amount = Amount::<UAH>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_uah_amount_with_kopiyok() {
        let amount = Amount::<UAH>::from_minor(10050); // 100.50 UAH
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
