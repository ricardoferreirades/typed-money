use crate::Currency;

/// Chinese Yuan (CNY)
///
/// The renminbi is the official currency of the People's Republic of China.
/// The yuan is the basic unit of the renminbi, but is also used to refer to the Chinese currency generally.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, CNY};
///
/// let amount = Amount::<CNY>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(CNY::CODE, "CNY");
/// assert_eq!(CNY::SYMBOL, "¥");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CNY;

impl Currency for CNY {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "CNY";
    const SYMBOL: &'static str = "¥";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_cny_currency_properties() {
        assert_eq!(CNY::DECIMALS, 2);
        assert_eq!(CNY::CODE, "CNY");
        assert_eq!(CNY::SYMBOL, "¥");
    }

    #[test]
    fn test_cny_amount_creation() {
        let amount = Amount::<CNY>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_cny_amount_with_cents() {
        let amount = Amount::<CNY>::from_minor(10050); // 100.50 CNY
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
