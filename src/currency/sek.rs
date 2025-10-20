use crate::Currency;

/// Swedish Krona (SEK)
///
/// The Swedish krona is the currency of Sweden.
/// It is subdivided into 100 öre.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, SEK};
///
/// let amount = Amount::<SEK>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(SEK::CODE, "SEK");
/// assert_eq!(SEK::SYMBOL, "kr");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SEK;

impl Currency for SEK {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "SEK";
    const SYMBOL: &'static str = "kr";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_sek_currency_properties() {
        assert_eq!(SEK::DECIMALS, 2);
        assert_eq!(SEK::CODE, "SEK");
        assert_eq!(SEK::SYMBOL, "kr");
    }

    #[test]
    fn test_sek_amount_creation() {
        let amount = Amount::<SEK>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_sek_amount_with_ore() {
        let amount = Amount::<SEK>::from_minor(10050); // 100.50 SEK
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
