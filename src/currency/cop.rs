use crate::Currency;

/// Colombian Peso (COP)
///
/// The Colombian peso is the currency of Colombia.
/// It is subdivided into 100 centavos.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, COP};
///
/// let amount = Amount::<COP>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(COP::CODE, "COP");
/// assert_eq!(COP::SYMBOL, "$");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct COP;

impl Currency for COP {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "COP";
    const SYMBOL: &'static str = "$";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_cop_currency_properties() {
        assert_eq!(COP::DECIMALS, 2);
        assert_eq!(COP::CODE, "COP");
        assert_eq!(COP::SYMBOL, "$");
    }

    #[test]
    fn test_cop_amount_creation() {
        let amount = Amount::<COP>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_cop_amount_with_centavos() {
        let amount = Amount::<COP>::from_minor(10050); // 100.50 COP
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
