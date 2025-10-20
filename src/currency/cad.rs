use crate::Currency;

/// Canadian Dollar (CAD)
///
/// The Canadian dollar is the currency of Canada. It is abbreviated with the dollar sign $,
/// or sometimes C$ to distinguish it from other dollar-denominated currencies.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, CAD};
///
/// let amount = Amount::<CAD>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(CAD::CODE, "CAD");
/// assert_eq!(CAD::SYMBOL, "C$");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CAD;

impl Currency for CAD {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "CAD";
    const SYMBOL: &'static str = "C$";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_cad_currency_properties() {
        assert_eq!(CAD::DECIMALS, 2);
        assert_eq!(CAD::CODE, "CAD");
        assert_eq!(CAD::SYMBOL, "C$");
    }

    #[test]
    fn test_cad_amount_creation() {
        let amount = Amount::<CAD>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_cad_amount_with_cents() {
        let amount = Amount::<CAD>::from_minor(10050); // 100.50 CAD
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
