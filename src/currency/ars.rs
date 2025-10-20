use crate::Currency;

/// Argentine Peso (ARS)
///
/// The Argentine peso is the currency of Argentina.
/// It is subdivided into 100 centavos.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, ARS};
///
/// let amount = Amount::<ARS>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(ARS::CODE, "ARS");
/// assert_eq!(ARS::SYMBOL, "$");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ARS;

impl Currency for ARS {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "ARS";
    const SYMBOL: &'static str = "$";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_ars_currency_properties() {
        assert_eq!(ARS::DECIMALS, 2);
        assert_eq!(ARS::CODE, "ARS");
        assert_eq!(ARS::SYMBOL, "$");
    }

    #[test]
    fn test_ars_amount_creation() {
        let amount = Amount::<ARS>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_ars_amount_with_centavos() {
        let amount = Amount::<ARS>::from_minor(10050); // 100.50 ARS
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
