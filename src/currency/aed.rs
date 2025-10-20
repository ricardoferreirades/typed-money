use crate::Currency;

/// United Arab Emirates Dirham (AED)
///
/// The United Arab Emirates dirham is the currency of the United Arab Emirates.
/// It is subdivided into 100 fils.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, AED};
///
/// let amount = Amount::<AED>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(AED::CODE, "AED");
/// assert_eq!(AED::SYMBOL, "د.إ");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AED;

impl Currency for AED {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "AED";
    const SYMBOL: &'static str = "د.إ";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_aed_currency_properties() {
        assert_eq!(AED::DECIMALS, 2);
        assert_eq!(AED::CODE, "AED");
        assert_eq!(AED::SYMBOL, "د.إ");
    }

    #[test]
    fn test_aed_amount_creation() {
        let amount = Amount::<AED>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_aed_amount_with_fils() {
        let amount = Amount::<AED>::from_minor(10050); // 100.50 AED
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
