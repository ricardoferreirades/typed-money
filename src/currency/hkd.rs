use crate::Currency;

/// Hong Kong Dollar (HKD)
///
/// The Hong Kong dollar is the official currency of Hong Kong.
/// It is subdivided into 100 cents.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, HKD};
///
/// let amount = Amount::<HKD>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(HKD::CODE, "HKD");
/// assert_eq!(HKD::SYMBOL, "HK$");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HKD;

impl Currency for HKD {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "HKD";
    const SYMBOL: &'static str = "HK$";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_hkd_currency_properties() {
        assert_eq!(HKD::DECIMALS, 2);
        assert_eq!(HKD::CODE, "HKD");
        assert_eq!(HKD::SYMBOL, "HK$");
    }

    #[test]
    fn test_hkd_amount_creation() {
        let amount = Amount::<HKD>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_hkd_amount_with_cents() {
        let amount = Amount::<HKD>::from_minor(10050); // 100.50 HKD
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
