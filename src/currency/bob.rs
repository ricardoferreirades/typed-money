use crate::Currency;

/// Bolivian Boliviano (BOB)
///
/// The Bolivian boliviano is the currency of Bolivia.
/// It is subdivided into 100 centavos.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, BOB};
///
/// let amount = Amount::<BOB>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(BOB::CODE, "BOB");
/// assert_eq!(BOB::SYMBOL, "Bs");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct BOB;

impl Currency for BOB {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "BOB";
    const SYMBOL: &'static str = "Bs";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_bob_currency_properties() {
        assert_eq!(BOB::DECIMALS, 2);
        assert_eq!(BOB::CODE, "BOB");
        assert_eq!(BOB::SYMBOL, "Bs");
    }

    #[test]
    fn test_bob_amount_creation() {
        let amount = Amount::<BOB>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_bob_amount_with_centavos() {
        let amount = Amount::<BOB>::from_minor(10050); // 100.50 BOB
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
