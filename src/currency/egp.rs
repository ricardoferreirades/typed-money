use crate::Currency;

/// Egyptian Pound (EGP)
///
/// The Egyptian pound is the currency of Egypt.
/// It is subdivided into 100 piastres.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, EGP};
///
/// let amount = Amount::<EGP>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(EGP::CODE, "EGP");
/// assert_eq!(EGP::SYMBOL, "£");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct EGP;

impl Currency for EGP {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "EGP";
    const SYMBOL: &'static str = "£";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_egp_currency_properties() {
        assert_eq!(EGP::DECIMALS, 2);
        assert_eq!(EGP::CODE, "EGP");
        assert_eq!(EGP::SYMBOL, "£");
    }

    #[test]
    fn test_egp_amount_creation() {
        let amount = Amount::<EGP>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_egp_amount_with_piastres() {
        let amount = Amount::<EGP>::from_minor(10050); // 100.50 EGP
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
