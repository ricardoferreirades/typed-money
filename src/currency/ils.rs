use crate::Currency;

/// Israeli Shekel (ILS)
///
/// The Israeli new shekel is the currency of Israel.
/// It is subdivided into 100 agorot.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, ILS};
///
/// let amount = Amount::<ILS>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(ILS::CODE, "ILS");
/// assert_eq!(ILS::SYMBOL, "₪");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ILS;

impl Currency for ILS {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "ILS";
    const SYMBOL: &'static str = "₪";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_ils_currency_properties() {
        assert_eq!(ILS::DECIMALS, 2);
        assert_eq!(ILS::CODE, "ILS");
        assert_eq!(ILS::SYMBOL, "₪");
    }

    #[test]
    fn test_ils_amount_creation() {
        let amount = Amount::<ILS>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_ils_amount_with_agorot() {
        let amount = Amount::<ILS>::from_minor(10050); // 100.50 ILS
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
