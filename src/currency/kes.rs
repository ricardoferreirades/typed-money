use crate::Currency;

/// Kenyan Shilling (KES)
///
/// The Kenyan shilling is the currency of Kenya.
/// It is subdivided into 100 cents.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, KES};
///
/// let amount = Amount::<KES>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(KES::CODE, "KES");
/// assert_eq!(KES::SYMBOL, "KSh");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct KES;

impl Currency for KES {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "KES";
    const SYMBOL: &'static str = "KSh";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_kes_currency_properties() {
        assert_eq!(KES::DECIMALS, 2);
        assert_eq!(KES::CODE, "KES");
        assert_eq!(KES::SYMBOL, "KSh");
    }

    #[test]
    fn test_kes_amount_creation() {
        let amount = Amount::<KES>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_kes_amount_with_cents() {
        let amount = Amount::<KES>::from_minor(10050); // 100.50 KES
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
