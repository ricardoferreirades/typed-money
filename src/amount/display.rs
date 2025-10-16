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

    /// Formats the amount with locale-specific number formatting.
    ///
    /// This is a basic implementation that supports common locale patterns:
    /// - `en_US`: `1,234.56` (comma thousands, period decimal)
    /// - `de_DE`: `1.234,56` (period thousands, comma decimal)
    /// - `fr_FR`: `1 234,56` (space thousands, comma decimal)
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_money::{Amount, USD};
    ///
    /// let amount = Amount::<USD>::from_major(1234567);
    ///
    /// // US/UK format (default)
    /// assert_eq!(amount.format_locale("en_US"), "$1,234,567.00 USD");
    ///
    /// // German format
    /// assert_eq!(amount.format_locale("de_DE"), "$1.234.567,00 USD");
    ///
    /// // French format
    /// assert_eq!(amount.format_locale("fr_FR"), "$1 234 567,00 USD");
    ///
    /// // Unknown locale defaults to en_US
    /// assert_eq!(amount.format_locale("unknown"), "$1,234,567.00 USD");
    /// ```
    pub fn format_locale(&self, locale: &str) -> String {
        let value_str = if C::DECIMALS == 0 {
            format!("{}", self.value.trunc())
        } else {
            format!("{:.prec$}", self.value, prec = C::DECIMALS as usize)
        };

        // Parse the value string to add locale-specific separators
        let formatted_value = match locale {
            "de_DE" | "de" => self.format_german_style(&value_str),
            "fr_FR" | "fr" => self.format_french_style(&value_str),
            _ => self.format_us_style(&value_str), // Default to US/UK format
        };

        format!("{}{} {}", C::SYMBOL, formatted_value, C::CODE)
    }

    fn format_us_style(&self, value: &str) -> String {
        // US format: 1,234.56 (comma thousands, period decimal)
        self.add_thousands_separator(value, ',', '.')
    }

    fn format_german_style(&self, value: &str) -> String {
        // German format: 1.234,56 (period thousands, comma decimal)
        let with_comma_decimal = value.replace('.', ",");
        self.add_thousands_separator(&with_comma_decimal, '.', ',')
    }

    fn format_french_style(&self, value: &str) -> String {
        // French format: 1 234,56 (space thousands, comma decimal)
        let with_comma_decimal = value.replace('.', ",");
        self.add_thousands_separator(&with_comma_decimal, ' ', ',')
    }

    fn add_thousands_separator(&self, value: &str, separator: char, decimal_sep: char) -> String {
        let parts: Vec<&str> = value.split(['.', ',']).collect();

        if parts.is_empty() {
            return value.to_string();
        }

        let integer_part = parts[0];
        let decimal_part = parts.get(1);

        // Handle negative sign
        let (is_negative, digits) = if let Some(stripped) = integer_part.strip_prefix('-') {
            (true, stripped)
        } else {
            (false, integer_part)
        };

        // Add thousands separators to integer part
        let mut result = String::new();
        let len = digits.len();

        for (i, ch) in digits.chars().enumerate() {
            if i > 0 && (len - i) % 3 == 0 {
                result.push(separator);
            }
            result.push(ch);
        }

        // Prepend negative sign if needed
        let formatted_integer = if is_negative {
            format!("-{}", result)
        } else {
            result
        };

        // Add decimal part if present
        if let Some(dec) = decimal_part {
            format!("{}{}{}", formatted_integer, decimal_sep, dec)
        } else {
            formatted_integer
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

    // ========================================================================
    // Locale Formatting Tests (Section 5.1)
    // ========================================================================

    #[test]
    fn test_format_locale_us() {
        let amount = Amount::<USD>::from_major(1234567);
        assert_eq!(amount.format_locale("en_US"), "$1,234,567.00 USD");
        assert_eq!(amount.format_locale("en"), "$1,234,567.00 USD");
    }

    #[test]
    fn test_format_locale_german() {
        let amount = Amount::<USD>::from_major(1234567);
        assert_eq!(amount.format_locale("de_DE"), "$1.234.567,00 USD");
        assert_eq!(amount.format_locale("de"), "$1.234.567,00 USD");
    }

    #[test]
    fn test_format_locale_french() {
        let amount = Amount::<USD>::from_major(1234567);
        assert_eq!(amount.format_locale("fr_FR"), "$1 234 567,00 USD");
        assert_eq!(amount.format_locale("fr"), "$1 234 567,00 USD");
    }

    #[test]
    fn test_format_locale_small_amount() {
        let amount = Amount::<USD>::from_major(99);
        assert_eq!(amount.format_locale("en_US"), "$99.00 USD");
        assert_eq!(amount.format_locale("de_DE"), "$99,00 USD");
        assert_eq!(amount.format_locale("fr_FR"), "$99,00 USD");
    }

    #[test]
    fn test_format_locale_thousands() {
        let amount = Amount::<USD>::from_major(1000);
        assert_eq!(amount.format_locale("en_US"), "$1,000.00 USD");
        assert_eq!(amount.format_locale("de_DE"), "$1.000,00 USD");
        assert_eq!(amount.format_locale("fr_FR"), "$1 000,00 USD");
    }

    #[test]
    fn test_format_locale_negative() {
        let amount = Amount::<USD>::from_major(-1234);
        assert_eq!(amount.format_locale("en_US"), "$-1,234.00 USD");
        assert_eq!(amount.format_locale("de_DE"), "$-1.234,00 USD");
        assert_eq!(amount.format_locale("fr_FR"), "$-1 234,00 USD");
    }

    #[test]
    fn test_format_locale_zero() {
        let amount = Amount::<USD>::from_major(0);
        assert_eq!(amount.format_locale("en_US"), "$0.00 USD");
        assert_eq!(amount.format_locale("de_DE"), "$0,00 USD");
    }

    #[test]
    fn test_format_locale_jpy() {
        let amount = Amount::<JPY>::from_major(1234567);
        // JPY has no decimals
        assert_eq!(amount.format_locale("en_US"), "¥1,234,567 JPY");
        assert_eq!(amount.format_locale("de_DE"), "¥1.234.567 JPY");
        assert_eq!(amount.format_locale("fr_FR"), "¥1 234 567 JPY");
    }

    #[test]
    fn test_format_locale_unknown_defaults_to_us() {
        let amount = Amount::<USD>::from_major(1234);
        assert_eq!(amount.format_locale("unknown"), "$1,234.00 USD");
        assert_eq!(amount.format_locale(""), "$1,234.00 USD");
    }
}
