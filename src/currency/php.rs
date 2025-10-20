use crate::Currency;

/// Philippine Peso (PHP)
///
/// The Philippine peso is the currency of the Philippines.
/// It is subdivided into 100 centavos.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, PHP};
///
/// let amount = Amount::<PHP>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(PHP::CODE, "PHP");
/// assert_eq!(PHP::SYMBOL, "₱");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PHP;

impl Currency for PHP {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "PHP";
    const SYMBOL: &'static str = "₱";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_php_currency_properties() {
        assert_eq!(PHP::DECIMALS, 2);
        assert_eq!(PHP::CODE, "PHP");
        assert_eq!(PHP::SYMBOL, "₱");
    }

    #[test]
    fn test_php_amount_creation() {
        let amount = Amount::<PHP>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_php_amount_with_centavos() {
        let amount = Amount::<PHP>::from_minor(10050); // 100.50 PHP
        assert_eq!(amount.to_major_floor(), 100);
        assert_eq!(amount.to_minor(), 10050);
    }
}
