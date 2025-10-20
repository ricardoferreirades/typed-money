use crate::Currency;

/// Bahraini Dinar (BHD)
///
/// The Bahraini dinar is the currency of Bahrain.
/// It is subdivided into 1000 fils.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, BHD};
///
/// let amount = Amount::<BHD>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(BHD::CODE, "BHD");
/// assert_eq!(BHD::SYMBOL, "د.ب");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BHD;

impl Currency for BHD {
    const DECIMALS: u8 = 3; // Bahraini Dinar uses 3 decimal places (fils)
    const CODE: &'static str = "BHD";
    const SYMBOL: &'static str = "د.ب";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_bhd_currency_properties() {
        assert_eq!(BHD::DECIMALS, 3);
        assert_eq!(BHD::CODE, "BHD");
        assert_eq!(BHD::SYMBOL, "د.ب");
    }

    #[test]
    fn test_bhd_amount_creation() {
        let amount = Amount::<BHD>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_bhd_amount_with_fils() {
        let amount = Amount::<BHD>::from_minor(100500); // 100.500 BHD
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 100500);
    }
}
