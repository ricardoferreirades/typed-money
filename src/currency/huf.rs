use crate::Currency;

/// Hungarian Forint (HUF)
///
/// The forint is the currency of Hungary.
/// It is subdivided into 100 fillér, although fillér coins are no longer in circulation.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, HUF};
///
/// let amount = Amount::<HUF>::from_major(1000);
/// assert_eq!(amount.to_major_floor(), 1000);
/// assert_eq!(HUF::CODE, "HUF");
/// assert_eq!(HUF::SYMBOL, "Ft");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HUF;

impl Currency for HUF {
    const DECIMALS: u8 = 0; // Hungarian Forint typically doesn't use decimal places
    const CODE: &'static str = "HUF";
    const SYMBOL: &'static str = "Ft";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_huf_currency_properties() {
        assert_eq!(HUF::DECIMALS, 0);
        assert_eq!(HUF::CODE, "HUF");
        assert_eq!(HUF::SYMBOL, "Ft");
    }

    #[test]
    fn test_huf_amount_creation() {
        let amount = Amount::<HUF>::from_major(1000);
        assert_eq!(amount.to_major_floor(), 1000);
    }

    #[test]
    fn test_huf_amount_with_minor() {
        let amount = Amount::<HUF>::from_minor(1000); // 1000 HUF (no decimals)
        assert_eq!(amount.to_major_floor(), 1000);
        assert_eq!(amount.to_minor(), 1000);
    }
}
