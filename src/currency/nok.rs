use crate::Currency;

/// Norwegian Krone (NOK)
///
/// The Norwegian krone is the currency of Norway.
/// It is subdivided into 100 øre.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, NOK};
///
/// let amount = Amount::<NOK>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(NOK::CODE, "NOK");
/// assert_eq!(NOK::SYMBOL, "kr");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NOK;

impl Currency for NOK {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "NOK";
    const SYMBOL: &'static str = "kr";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_nok_currency_properties() {
        assert_eq!(NOK::DECIMALS, 2);
        assert_eq!(NOK::CODE, "NOK");
        assert_eq!(NOK::SYMBOL, "kr");
    }

    #[test]
    fn test_nok_amount_creation() {
        let amount = Amount::<NOK>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_nok_amount_with_ore() {
        let amount = Amount::<NOK>::from_minor(10050); // 100.50 NOK
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
