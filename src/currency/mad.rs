use crate::Currency;

/// Moroccan Dirham (MAD)
///
/// The Moroccan dirham is the currency of Morocco.
/// It is subdivided into 100 centimes.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, MAD};
///
/// let amount = Amount::<MAD>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(MAD::CODE, "MAD");
/// assert_eq!(MAD::SYMBOL, "د.م.");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MAD;

impl Currency for MAD {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "MAD";
    const SYMBOL: &'static str = "د.م.";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_mad_currency_properties() {
        assert_eq!(MAD::DECIMALS, 2);
        assert_eq!(MAD::CODE, "MAD");
        assert_eq!(MAD::SYMBOL, "د.م.");
    }

    #[test]
    fn test_mad_amount_creation() {
        let amount = Amount::<MAD>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_mad_amount_with_centimes() {
        let amount = Amount::<MAD>::from_minor(10050); // 100.50 MAD
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
