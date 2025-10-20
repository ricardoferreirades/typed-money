use crate::Currency;

/// Swiss Franc (CHF)
///
/// The Swiss franc is the currency and legal tender of Switzerland and Liechtenstein.
/// It is also legal tender in the Italian exclave of Campione d'Italia.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, CHF};
///
/// let amount = Amount::<CHF>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(CHF::CODE, "CHF");
/// assert_eq!(CHF::SYMBOL, "CHF");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CHF;

impl Currency for CHF {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "CHF";
    const SYMBOL: &'static str = "CHF";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_chf_currency_properties() {
        assert_eq!(CHF::DECIMALS, 2);
        assert_eq!(CHF::CODE, "CHF");
        assert_eq!(CHF::SYMBOL, "CHF");
    }

    #[test]
    fn test_chf_amount_creation() {
        let amount = Amount::<CHF>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_chf_amount_with_cents() {
        let amount = Amount::<CHF>::from_minor(10050); // 100.50 CHF
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
