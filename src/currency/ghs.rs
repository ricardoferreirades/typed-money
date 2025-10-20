use crate::Currency;

/// Ghanaian Cedi (GHS)
///
/// The Ghanaian cedi is the currency of Ghana.
/// It is subdivided into 100 pesewas.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, GHS};
///
/// let amount = Amount::<GHS>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(GHS::CODE, "GHS");
/// assert_eq!(GHS::SYMBOL, "₵");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct GHS;

impl Currency for GHS {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "GHS";
    const SYMBOL: &'static str = "₵";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_ghs_currency_properties() {
        assert_eq!(GHS::DECIMALS, 2);
        assert_eq!(GHS::CODE, "GHS");
        assert_eq!(GHS::SYMBOL, "₵");
    }

    #[test]
    fn test_ghs_amount_creation() {
        let amount = Amount::<GHS>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_ghs_amount_with_pesewas() {
        let amount = Amount::<GHS>::from_minor(10050); // 100.50 GHS
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
