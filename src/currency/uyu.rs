use crate::Currency;

/// Uruguayan Peso (UYU)
///
/// The Uruguayan peso is the currency of Uruguay.
/// It is subdivided into 100 centésimos.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, UYU};
///
/// let amount = Amount::<UYU>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(UYU::CODE, "UYU");
/// assert_eq!(UYU::SYMBOL, "$U");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct UYU;

impl Currency for UYU {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "UYU";
    const SYMBOL: &'static str = "$U";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_uyu_currency_properties() {
        assert_eq!(UYU::DECIMALS, 2);
        assert_eq!(UYU::CODE, "UYU");
        assert_eq!(UYU::SYMBOL, "$U");
    }

    #[test]
    fn test_uyu_amount_creation() {
        let amount = Amount::<UYU>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_uyu_amount_with_centésimos() {
        let amount = Amount::<UYU>::from_minor(10050); // 100.50 UYU
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
