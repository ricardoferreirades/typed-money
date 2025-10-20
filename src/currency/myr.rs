use crate::Currency;

/// Malaysian Ringgit (MYR)
///
/// The Malaysian ringgit is the currency of Malaysia.
/// It is subdivided into 100 sen.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, MYR};
///
/// let amount = Amount::<MYR>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(MYR::CODE, "MYR");
/// assert_eq!(MYR::SYMBOL, "RM");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MYR;

impl Currency for MYR {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "MYR";
    const SYMBOL: &'static str = "RM";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_myr_currency_properties() {
        assert_eq!(MYR::DECIMALS, 2);
        assert_eq!(MYR::CODE, "MYR");
        assert_eq!(MYR::SYMBOL, "RM");
    }

    #[test]
    fn test_myr_amount_creation() {
        let amount = Amount::<MYR>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_myr_amount_with_sen() {
        let amount = Amount::<MYR>::from_minor(10050); // 100.50 MYR
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
