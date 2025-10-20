use crate::Currency;

/// Turkish Lira (TRY)
///
/// The Turkish lira is the currency of Turkey and Northern Cyprus.
/// It is subdivided into 100 kuruş.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, TRY};
///
/// let amount = Amount::<TRY>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(TRY::CODE, "TRY");
/// assert_eq!(TRY::SYMBOL, "₺");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TRY;

impl Currency for TRY {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "TRY";
    const SYMBOL: &'static str = "₺";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_try_currency_properties() {
        assert_eq!(TRY::DECIMALS, 2);
        assert_eq!(TRY::CODE, "TRY");
        assert_eq!(TRY::SYMBOL, "₺");
    }

    #[test]
    fn test_try_amount_creation() {
        let amount = Amount::<TRY>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_try_amount_with_kuruş() {
        let amount = Amount::<TRY>::from_minor(10050); // 100.50 TRY
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
