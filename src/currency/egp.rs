use super::{CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};
use crate::Currency;

/// Egyptian Pound (EGP)
///
/// The Egyptian pound is the currency of Egypt.
/// It is subdivided into 100 piastres.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, EGP};
///
/// let amount = Amount::<EGP>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(EGP::CODE, "EGP");
/// assert_eq!(EGP::SYMBOL, "£");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EGP;

impl Currency for EGP {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "EGP";
    const SYMBOL: &'static str = "£";

    // Rich metadata
    const NAME: &'static str = "Egyptian Pound";
    const COUNTRY: &'static str = "Egypt";
    const REGION: &'static str = "Africa";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 1834;
    const ISO_4217_NUMBER: u16 = 818;
    const THOUSANDS_SEPARATOR: char = ',';
    const DECIMAL_SEPARATOR: char = '.';
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
    fn test_egp_currency_properties() {
        assert_eq!(EGP::DECIMALS, 2);
        assert_eq!(EGP::CODE, "EGP");
        assert_eq!(EGP::SYMBOL, "£");
    }

    #[test]
    fn test_egp_amount_creation() {
        let amount = Amount::<EGP>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_egp_amount_with_piastres() {
        let amount = Amount::<EGP>::from_minor(10050); // 100.50 EGP
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
