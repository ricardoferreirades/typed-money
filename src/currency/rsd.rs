use crate::Currency;

/// Serbian Dinar (RSD)
///
/// The Serbian dinar is the currency of Serbia.
/// It is subdivided into 100 para.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, RSD};
///
/// let amount = Amount::<RSD>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(RSD::CODE, "RSD");
/// assert_eq!(RSD::SYMBOL, "дин");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RSD;

impl Currency for RSD {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "RSD";
    const SYMBOL: &'static str = "дин";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_rsd_currency_properties() {
        assert_eq!(RSD::DECIMALS, 2);
        assert_eq!(RSD::CODE, "RSD");
        assert_eq!(RSD::SYMBOL, "дин");
    }

    #[test]
    fn test_rsd_amount_creation() {
        let amount = Amount::<RSD>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_rsd_amount_with_para() {
        let amount = Amount::<RSD>::from_minor(10050); // 100.50 RSD
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
