use crate::Currency;

/// Singapore Dollar (SGD)
///
/// The Singapore dollar is the official currency of Singapore.
/// It is divided into 100 cents.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, SGD};
///
/// let amount = Amount::<SGD>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(SGD::CODE, "SGD");
/// assert_eq!(SGD::SYMBOL, "S$");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SGD;

impl Currency for SGD {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "SGD";
    const SYMBOL: &'static str = "S$";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_sgd_currency_properties() {
        assert_eq!(SGD::DECIMALS, 2);
        assert_eq!(SGD::CODE, "SGD");
        assert_eq!(SGD::SYMBOL, "S$");
    }

    #[test]
    fn test_sgd_amount_creation() {
        let amount = Amount::<SGD>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_sgd_amount_with_cents() {
        let amount = Amount::<SGD>::from_minor(10050); // 100.50 SGD
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
