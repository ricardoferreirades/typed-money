use crate::Currency;

/// Australian Dollar (AUD)
///
/// The Australian dollar is the currency of Australia, including its external territories.
/// It is subdivided into 100 cents.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, AUD};
///
/// let amount = Amount::<AUD>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(AUD::CODE, "AUD");
/// assert_eq!(AUD::SYMBOL, "A$");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AUD;

impl Currency for AUD {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "AUD";
    const SYMBOL: &'static str = "A$";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_aud_currency_properties() {
        assert_eq!(AUD::DECIMALS, 2);
        assert_eq!(AUD::CODE, "AUD");
        assert_eq!(AUD::SYMBOL, "A$");
    }

    #[test]
    fn test_aud_amount_creation() {
        let amount = Amount::<AUD>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_aud_amount_with_cents() {
        let amount = Amount::<AUD>::from_minor(10050); // 100.50 AUD
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
