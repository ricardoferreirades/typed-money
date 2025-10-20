use crate::Currency;

/// Cardano (ADA)
///
/// Cardano is a blockchain platform for changemakers, innovators, and visionaries,
/// with the tools and technologies required to create possibility for the many,
/// as well as the few, and bring about positive global change.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, ADA};
///
/// let amount = Amount::<ADA>::from_major(1000);
/// assert_eq!(amount.to_major_floor(), 1000);
/// assert_eq!(ADA::CODE, "ADA");
/// assert_eq!(ADA::SYMBOL, "₳");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ADA;

impl Currency for ADA {
    const DECIMALS: u8 = 6;
    const CODE: &'static str = "ADA";
    const SYMBOL: &'static str = "₳";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_ada_currency_properties() {
        assert_eq!(ADA::DECIMALS, 6);
        assert_eq!(ADA::CODE, "ADA");
        assert_eq!(ADA::SYMBOL, "₳");
    }

    #[test]
    fn test_ada_amount_creation() {
        let amount = Amount::<ADA>::from_major(1000);
        assert_eq!(amount.to_major_floor(), 1000);
    }

    #[test]
    fn test_ada_amount_with_lovelace() {
        let amount = Amount::<ADA>::from_minor(1_000_000); // 1.000000 ADA
        assert_eq!(amount.to_major_floor(), 1);
        assert_eq!(amount.to_minor(), 1_000_000);
    }
}
