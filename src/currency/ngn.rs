use super::{Currency, CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};

/// Nigerian Naira (NGN)
///
/// The Nigerian naira is the currency of Nigeria.
/// It is subdivided into 100 kobo.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, NGN};
///
/// let amount = Amount::<NGN>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(NGN::CODE, "NGN");
/// assert_eq!(NGN::SYMBOL, "₦");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NGN;

impl Currency for NGN {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "NGN";
    const SYMBOL: &'static str = "₦";

    // Rich metadata
    const NAME: &'static str = "Nigerian Naira";
    const COUNTRY: &'static str = "Nigeria";
    const REGION: &'static str = "West Africa";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 1973;
    const ISO_4217_NUMBER: u16 = 566;
    const THOUSANDS_SEPARATOR: char = ',';
    const DECIMAL_SEPARATOR: char = '.';
    const SYMBOL_POSITION: SymbolPosition = SymbolPosition::Before;
    const SPACE_BETWEEN: bool = false;
    const VOLATILITY_RATING: VolatilityRating = VolatilityRating::High;
    const LIQUIDITY_RATING: LiquidityRating = LiquidityRating::Medium;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_ngn_currency_properties() {
        assert_eq!(NGN::DECIMALS, 2);
        assert_eq!(NGN::CODE, "NGN");
        assert_eq!(NGN::SYMBOL, "₦");
    }

    #[test]
    fn test_ngn_amount_creation() {
        let amount = Amount::<NGN>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_ngn_amount_with_kobo() {
        let amount = Amount::<NGN>::from_minor(10050); // 100.50 NGN
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
