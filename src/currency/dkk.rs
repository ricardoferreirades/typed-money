use crate::Currency;

/// Danish Krone (DKK)
///
/// The Danish krone is the official currency of Denmark.
/// It is subdivided into 100 øre.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, DKK};
///
/// let amount = Amount::<DKK>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(DKK::CODE, "DKK");
/// assert_eq!(DKK::SYMBOL, "kr");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DKK;

impl Currency for DKK {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "DKK";
    const SYMBOL: &'static str = "kr";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_dkk_currency_properties() {
        assert_eq!(DKK::DECIMALS, 2);
        assert_eq!(DKK::CODE, "DKK");
        assert_eq!(DKK::SYMBOL, "kr");
    }

    #[test]
    fn test_dkk_amount_creation() {
        let amount = Amount::<DKK>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_dkk_amount_with_ore() {
        let amount = Amount::<DKK>::from_minor(10050); // 100.50 DKK
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
