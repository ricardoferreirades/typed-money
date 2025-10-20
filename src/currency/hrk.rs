use crate::Currency;

/// Croatian Kuna (HRK)
///
/// The Croatian kuna was the currency of Croatia from 1994 until 2023.
/// It was subdivided into 100 lipa.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, HRK};
///
/// let amount = Amount::<HRK>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(HRK::CODE, "HRK");
/// assert_eq!(HRK::SYMBOL, "kn");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HRK;

impl Currency for HRK {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "HRK";
    const SYMBOL: &'static str = "kn";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_hrk_currency_properties() {
        assert_eq!(HRK::DECIMALS, 2);
        assert_eq!(HRK::CODE, "HRK");
        assert_eq!(HRK::SYMBOL, "kn");
    }

    #[test]
    fn test_hrk_amount_creation() {
        let amount = Amount::<HRK>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_hrk_amount_with_lipa() {
        let amount = Amount::<HRK>::from_minor(10050); // 100.50 HRK
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
