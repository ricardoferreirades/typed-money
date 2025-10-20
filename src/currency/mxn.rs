use crate::Currency;

/// Mexican Peso (MXN)
///
/// The Mexican peso is the currency of Mexico.
/// It is subdivided into 100 centavos.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, MXN};
///
/// let amount = Amount::<MXN>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(MXN::CODE, "MXN");
/// assert_eq!(MXN::SYMBOL, "$");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MXN;

impl Currency for MXN {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "MXN";
    const SYMBOL: &'static str = "$";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_mxn_currency_properties() {
        assert_eq!(MXN::DECIMALS, 2);
        assert_eq!(MXN::CODE, "MXN");
        assert_eq!(MXN::SYMBOL, "$");
    }

    #[test]
    fn test_mxn_amount_creation() {
        let amount = Amount::<MXN>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_mxn_amount_with_centavos() {
        let amount = Amount::<MXN>::from_minor(10050); // 100.50 MXN
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
