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

impl<C: Currency> Amount<C> {
    /// Formats the amount as a string with symbol and currency code.
    ///
    /// This is equivalent to calling `.to_string()` via the Display trait.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_money::{Amount, USD};
    ///
    /// let amount = Amount::<USD>::from_major(100);
    /// assert_eq!(amount.format_full(), "$100.00 USD");
    /// ```
    pub fn format_full(&self) -> String {
        format!("{}", self)
    }

    /// Formats the amount with symbol only (no currency code).
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_money::{Amount, USD, EUR};
    ///
    /// let usd = Amount::<USD>::from_major(100);
    /// assert_eq!(usd.format_symbol(), "$100.00");
    ///
    /// let eur = Amount::<EUR>::from_minor(12345);
    /// assert_eq!(eur.format_symbol(), "€123.45");
    /// ```
    pub fn format_symbol(&self) -> String {
        let formatted_value = if C::DECIMALS == 0 {
            format!("{}", self.value.trunc())
        } else {
            format!("{:.prec$}", self.value, prec = C::DECIMALS as usize)
        };

        format!("{}{}", C::SYMBOL, formatted_value)
    }

    /// Formats the amount with currency code only (no symbol).
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_money::{Amount, USD};
    ///
    /// let amount = Amount::<USD>::from_major(100);
    /// assert_eq!(amount.format_code(), "100.00 USD");
    /// ```
    pub fn format_code(&self) -> String {
        let formatted_value = if C::DECIMALS == 0 {
            format!("{}", self.value.trunc())
        } else {
            format!("{:.prec$}", self.value, prec = C::DECIMALS as usize)
        };

        format!("{} {}", formatted_value, C::CODE)
    }

    /// Formats the amount as a plain number (no symbol or code).
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_money::{Amount, USD};
    ///
    /// let amount = Amount::<USD>::from_major(100);
    /// assert_eq!(amount.format_plain(), "100.00");
    /// ```
    pub fn format_plain(&self) -> String {
        if C::DECIMALS == 0 {
            format!("{}", self.value.trunc())
        } else {
            format!("{:.prec$}", self.value, prec = C::DECIMALS as usize)
        }
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

    // Determinism tests
    #[test]
    fn test_display_formatting_determinism() {
        use crate::JPY;
        // Verify Display output is consistent across platforms
        let usd = Amount::<USD>::from_major(100);
        let eur = Amount::<EUR>::from_minor(12345);
        let jpy = Amount::<JPY>::from_major(1000);
        let btc = Amount::<BTC>::from_major(1);

        assert_eq!(format!("{}", usd), "$100.00 USD");
        assert_eq!(format!("{}", eur), "€123.45 EUR");
        assert_eq!(format!("{}", jpy), "¥1000 JPY");
        assert_eq!(format!("{}", btc), "₿1.00000000 BTC");
    }

    // ========================================================================
    // Custom Formatting Tests (Section 5.1)
    // ========================================================================

    #[test]
    fn test_format_full() {
        let amount = Amount::<USD>::from_major(100);
        assert_eq!(amount.format_full(), "$100.00 USD");
    }

    #[test]
    fn test_format_symbol() {
        let usd = Amount::<USD>::from_major(100);
        assert_eq!(usd.format_symbol(), "$100.00");

        let eur = Amount::<EUR>::from_minor(12345);
        assert_eq!(eur.format_symbol(), "€123.45");

        let jpy = Amount::<JPY>::from_major(1000);
        assert_eq!(jpy.format_symbol(), "¥1000");
    }

    #[test]
    fn test_format_code() {
        let usd = Amount::<USD>::from_major(100);
        assert_eq!(usd.format_code(), "100.00 USD");

        let jpy = Amount::<JPY>::from_major(1000);
        assert_eq!(jpy.format_code(), "1000 JPY");
    }

    #[test]
    fn test_format_plain() {
        let usd = Amount::<USD>::from_major(100);
        assert_eq!(usd.format_plain(), "100.00");

        let jpy = Amount::<JPY>::from_major(1000);
        assert_eq!(jpy.format_plain(), "1000");

        let btc = Amount::<BTC>::from_major(1);
        assert_eq!(btc.format_plain(), "1.00000000");
    }

    #[test]
    fn test_format_negative() {
        let amount = Amount::<USD>::from_major(-50);
        assert_eq!(amount.format_full(), "$-50.00 USD");
        assert_eq!(amount.format_symbol(), "$-50.00");
        assert_eq!(amount.format_plain(), "-50.00");
    }

    #[test]
    fn test_format_zero() {
        let zero = Amount::<USD>::from_major(0);
        assert_eq!(zero.format_full(), "$0.00 USD");
        assert_eq!(zero.format_symbol(), "$0.00");
        assert_eq!(zero.format_plain(), "0.00");
    }
}
