use crate::Currency;

/// Nigerian Naira (NGN)
///
/// The Nigerian naira is the currency of Nigeria.
/// It is subdivided into 100 kobo.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, NGN};
///
/// let amount = Amount::<NGN>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(NGN::CODE, "NGN");
/// assert_eq!(NGN::SYMBOL, "₦");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NGN;

impl Currency for NGN {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "NGN";
    const SYMBOL: &'static str = "₦";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_ngn_currency_properties() {
        assert_eq!(NGN::DECIMALS, 2);
        assert_eq!(NGN::CODE, "NGN");
        assert_eq!(NGN::SYMBOL, "₦");
    }

    #[test]
    fn test_ngn_amount_creation() {
        let amount = Amount::<NGN>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_ngn_amount_with_kobo() {
        let amount = Amount::<NGN>::from_minor(10050); // 100.50 NGN
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
