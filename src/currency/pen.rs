use crate::Currency;

/// Peruvian Sol (PEN)
///
/// The Peruvian sol is the currency of Peru.
/// It is subdivided into 100 céntimos.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, PEN};
///
/// let amount = Amount::<PEN>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(PEN::CODE, "PEN");
/// assert_eq!(PEN::SYMBOL, "S/");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PEN;

impl Currency for PEN {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "PEN";
    const SYMBOL: &'static str = "S/";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_pen_currency_properties() {
        assert_eq!(PEN::DECIMALS, 2);
        assert_eq!(PEN::CODE, "PEN");
        assert_eq!(PEN::SYMBOL, "S/");
    }

    #[test]
    fn test_pen_amount_creation() {
        let amount = Amount::<PEN>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_pen_amount_with_centimos() {
        let amount = Amount::<PEN>::from_minor(10050); // 100.50 PEN
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
