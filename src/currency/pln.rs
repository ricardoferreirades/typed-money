use crate::Currency;

/// Polish Złoty (PLN)
///
/// The złoty is the official currency and legal tender of Poland.
/// It is subdivided into 100 groszy.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, PLN};
///
/// let amount = Amount::<PLN>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(PLN::CODE, "PLN");
/// assert_eq!(PLN::SYMBOL, "zł");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PLN;

impl Currency for PLN {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "PLN";
    const SYMBOL: &'static str = "zł";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_pln_currency_properties() {
        assert_eq!(PLN::DECIMALS, 2);
        assert_eq!(PLN::CODE, "PLN");
        assert_eq!(PLN::SYMBOL, "zł");
    }

    #[test]
    fn test_pln_amount_creation() {
        let amount = Amount::<PLN>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_pln_amount_with_groszy() {
        let amount = Amount::<PLN>::from_minor(10050); // 100.50 PLN
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
