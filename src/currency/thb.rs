use crate::Currency;

/// Thai Baht (THB)
///
/// The Thai baht is the currency of Thailand.
/// It is subdivided into 100 satang.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, THB};
///
/// let amount = Amount::<THB>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(THB::CODE, "THB");
/// assert_eq!(THB::SYMBOL, "฿");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct THB;

impl Currency for THB {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "THB";
    const SYMBOL: &'static str = "฿";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_thb_currency_properties() {
        assert_eq!(THB::DECIMALS, 2);
        assert_eq!(THB::CODE, "THB");
        assert_eq!(THB::SYMBOL, "฿");
    }

    #[test]
    fn test_thb_amount_creation() {
        let amount = Amount::<THB>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_thb_amount_with_satang() {
        let amount = Amount::<THB>::from_minor(10050); // 100.50 THB
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
