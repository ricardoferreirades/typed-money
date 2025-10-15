//! Display implementation for Amount.

use super::type_def::Amount;
use crate::Currency;
use std::fmt;

impl<C: Currency> fmt::Display for Amount<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format: {symbol}{amount} {code}
        // e.g., "$100.00 USD" or "€85.50 EUR"
        let formatted_value = if C::DECIMALS == 0 {
            format!("{}", self.value.trunc())
        } else {
            format!("{:.prec$}", self.value, prec = C::DECIMALS as usize)
        };

        write!(f, "{}{} {}", C::SYMBOL, formatted_value, C::CODE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BTC, EUR, JPY, USD};

    #[test]
    fn test_display_usd() {
        let amount = Amount::<USD>::from_major(100);
        assert_eq!(format!("{}", amount), "$100.00 USD");
    }

    #[test]
    fn test_display_eur() {
        let amount = Amount::<EUR>::from_minor(12345);
        assert_eq!(format!("{}", amount), "€123.45 EUR");
    }

    #[test]
    fn test_display_jpy() {
        let amount = Amount::<JPY>::from_major(1000);
        assert_eq!(format!("{}", amount), "¥1000 JPY");
    }

    #[test]
    fn test_display_btc() {
        let amount = Amount::<BTC>::from_major(1);
        assert_eq!(format!("{}", amount), "₿1.00000000 BTC");
    }
}
