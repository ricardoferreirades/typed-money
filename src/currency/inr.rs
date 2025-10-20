use crate::Currency;

/// Indian Rupee (INR)
///
/// The Indian rupee is the official currency of India.
/// It is subdivided into 100 paise.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, INR};
///
/// let amount = Amount::<INR>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(INR::CODE, "INR");
/// assert_eq!(INR::SYMBOL, "₹");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct INR;

impl Currency for INR {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "INR";
    const SYMBOL: &'static str = "₹";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_inr_currency_properties() {
        assert_eq!(INR::DECIMALS, 2);
        assert_eq!(INR::CODE, "INR");
        assert_eq!(INR::SYMBOL, "₹");
    }

    #[test]
    fn test_inr_amount_creation() {
        let amount = Amount::<INR>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_inr_amount_with_paise() {
        let amount = Amount::<INR>::from_minor(10050); // 100.50 INR
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
