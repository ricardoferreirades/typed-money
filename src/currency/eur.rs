//! Euro currency implementation.

use super::Currency;

/// Euro
///
/// # Example
///
/// ```
/// use typed_money::{Amount, EUR};
///
/// let amount = Amount::<EUR>::from_major(100);
/// println!("{}", amount);  // Displays: €100.00 EUR
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct EUR;

impl Currency for EUR {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "EUR";
    const SYMBOL: &'static str = "€";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eur_constants() {
        assert_eq!(EUR::DECIMALS, 2);
        assert_eq!(EUR::CODE, "EUR");
        assert_eq!(EUR::SYMBOL, "€");
    }
}
