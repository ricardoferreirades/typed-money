use crate::Currency;

/// Tunisian Dinar (TND)
///
/// The Tunisian dinar is the currency of Tunisia.
/// It is subdivided into 1000 millimes.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, TND};
///
/// let amount = Amount::<TND>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(TND::CODE, "TND");
/// assert_eq!(TND::SYMBOL, "د.ت");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TND;

impl Currency for TND {
    const DECIMALS: u8 = 3; // Tunisian Dinar uses 3 decimal places (millimes)
    const CODE: &'static str = "TND";
    const SYMBOL: &'static str = "د.ت";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_tnd_currency_properties() {
        assert_eq!(TND::DECIMALS, 3);
        assert_eq!(TND::CODE, "TND");
        assert_eq!(TND::SYMBOL, "د.ت");
    }

    #[test]
    fn test_tnd_amount_creation() {
        let amount = Amount::<TND>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_tnd_amount_with_millimes() {
        let amount = Amount::<TND>::from_minor(100500); // 100.500 TND
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 100500);
    }
}
