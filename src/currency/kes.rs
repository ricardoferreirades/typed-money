use super::{Currency, CurrencyType, SymbolPosition, VolatilityRating, LiquidityRating};

/// Kenyan Shilling (KES)
///
/// The Kenyan shilling is the currency of Kenya.
/// It is subdivided into 100 cents.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, KES};
///
/// let amount = Amount::<KES>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(KES::CODE, "KES");
/// assert_eq!(KES::SYMBOL, "KSh");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct KES;

impl Currency for KES {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "KES";
    const SYMBOL: &'static str = "KSh";
    
    // Rich metadata
    const NAME: &'static str = "Kenyan Shilling";
    const COUNTRY: &'static str = "Kenya";
    const REGION: &'static str = "East Africa";
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
    const IS_MAJOR: bool = false;
    const IS_STABLE: bool = false;
    const INTRODUCED_YEAR: u16 = 1966;
    const ISO_4217_NUMBER: u16 = 404;
    const THOUSANDS_SEPARATOR: char = ',';
    const DECIMAL_SEPARATOR: char = '.';
    const SYMBOL_POSITION: SymbolPosition = SymbolPosition::Before;
    const SPACE_BETWEEN: bool = false;
    const VOLATILITY_RATING: VolatilityRating = VolatilityRating::Medium;
    const LIQUIDITY_RATING: LiquidityRating = LiquidityRating::Medium;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_kes_currency_properties() {
        assert_eq!(KES::DECIMALS, 2);
        assert_eq!(KES::CODE, "KES");
        assert_eq!(KES::SYMBOL, "KSh");
    }

    #[test]
    fn test_kes_amount_creation() {
        let amount = Amount::<KES>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_kes_amount_with_cents() {
        let amount = Amount::<KES>::from_minor(10050); // 100.50 KES
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
