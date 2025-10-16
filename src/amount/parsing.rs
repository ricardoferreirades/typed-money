//! String parsing for Amount.

use super::type_def::Amount;
use crate::{Currency, MoneyError, MoneyResult};
use std::marker::PhantomData;
use std::str::FromStr;

#[cfg(all(feature = "use_rust_decimal", not(feature = "use_bigdecimal")))]
use rust_decimal::Decimal;

#[cfg(all(feature = "use_bigdecimal", not(feature = "use_rust_decimal")))]
use bigdecimal::BigDecimal as Decimal;

impl<C: Currency> Amount<C> {
    /// Parses a string into an Amount.
    ///
    /// Supports multiple formats:
    /// - `"12.34"` - numeric only
    /// - `"$12.34"` - with symbol (validates currency matches)
    /// - `"12.34 USD"` - with currency code (validates currency matches)
    /// - `"USD 12.34"` - alternative format (validates currency matches)
    /// - `"$12.34 USD"` - with both symbol and code
    ///
    /// Whitespace is trimmed automatically.
    ///
    /// # Security
    ///
    /// This parser rejects ambiguous or potentially malicious inputs:
    /// - Multiple decimal points
    /// - Non-numeric characters (except currency symbols/codes)
    /// - Excessively long strings
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_money::{Amount, USD};
    ///
    /// let amount = Amount::<USD>::parse("12.34")?;
    /// assert_eq!(amount.to_minor(), 1234);
    ///
    /// let amount2 = Amount::<USD>::parse("$12.34")?;
    /// assert_eq!(amount2.to_minor(), 1234);
    ///
    /// let amount3 = Amount::<USD>::parse("12.34 USD")?;
    /// assert_eq!(amount3.to_minor(), 1234);
    ///
    /// // Mismatched currency returns error
    /// assert!(Amount::<USD>::parse("€12.34").is_err());
    /// # Ok::<(), typed_money::MoneyError>(())
    /// ```
    pub fn parse(input: &str) -> MoneyResult<Self> {
        let trimmed = input.trim();

        if trimmed.is_empty() {
            return Err(MoneyError::ParseError {
                input: input.to_string(),
                expected_currency: Some(C::CODE),
                reason: "Empty string".to_string(),
            });
        }

        // Check for excessively long input (security)
        if trimmed.len() > 100 {
            return Err(MoneyError::ParseError {
                input: input.to_string(),
                expected_currency: Some(C::CODE),
                reason: "Input too long (max 100 characters)".to_string(),
            });
        }

        // Remove currency symbol if present and validate
        let mut working = trimmed;
        if working.starts_with(C::SYMBOL) {
            working = &working[C::SYMBOL.len()..];
        } else {
            // Check if it starts with a different currency symbol
            let other_symbols = ["$", "€", "£", "¥", "₿", "Ξ"];
            for symbol in &other_symbols {
                if working.starts_with(symbol) && *symbol != C::SYMBOL {
                    return Err(MoneyError::ParseError {
                        input: input.to_string(),
                        expected_currency: Some(C::CODE),
                        reason: format!(
                            "Currency symbol mismatch: found {}, expected {}",
                            symbol,
                            C::SYMBOL
                        ),
                    });
                }
            }
        }

        // Remove currency code if present (at start or end) and validate
        working = working.trim();

        // Check at the end first (more common: "12.34 USD")
        if working.ends_with(C::CODE) {
            working = working[..working.len() - C::CODE.len()].trim();
        } else if working.starts_with(C::CODE) {
            // Alternative format: "USD 12.34"
            working = working[C::CODE.len()..].trim();
        } else {
            // Check if it contains a different currency code
            let codes = ["USD", "EUR", "GBP", "JPY", "BTC", "ETH"];
            for code in &codes {
                if (working.ends_with(code) || working.starts_with(code)) && *code != C::CODE {
                    return Err(MoneyError::ParseError {
                        input: input.to_string(),
                        expected_currency: Some(C::CODE),
                        reason: format!(
                            "Currency code mismatch: found {}, expected {}",
                            code,
                            C::CODE
                        ),
                    });
                }
            }
        }

        working = working.trim();

        // Parse the numeric value
        let decimal_value = Decimal::from_str(working).map_err(|_| MoneyError::ParseError {
            input: input.to_string(),
            expected_currency: Some(C::CODE),
            reason: format!("Invalid numeric value: '{}'", working),
        })?;

        Ok(Self {
            value: decimal_value,
            _currency: PhantomData,
        })
    }
}

impl<C: Currency> FromStr for Amount<C> {
    type Err = MoneyError;

    /// Parses a string into an Amount using the FromStr trait.
    ///
    /// See [`Amount::parse`] for supported formats and examples.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EUR, GBP, JPY, USD};

    // ========================================================================
    // Parsing Tests - Numeric Only
    // ========================================================================

    #[test]
    fn test_parse_numeric_only() {
        let amount = Amount::<USD>::parse("12.34").unwrap();
        assert_eq!(amount.to_minor(), 1234);
    }

    #[test]
    fn test_parse_integer() {
        let amount = Amount::<USD>::parse("100").unwrap();
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_parse_negative() {
        let amount = Amount::<USD>::parse("-50.25").unwrap();
        assert_eq!(amount.to_minor(), -5025);
    }

    #[test]
    fn test_parse_zero() {
        let amount = Amount::<USD>::parse("0").unwrap();
        assert_eq!(amount.to_major_floor(), 0);
    }

    // ========================================================================
    // Parsing Tests - With Symbol
    // ========================================================================

    #[test]
    fn test_parse_with_symbol() {
        let amount = Amount::<USD>::parse("$12.34").unwrap();
        assert_eq!(amount.to_minor(), 1234);
    }

    #[test]
    fn test_parse_eur_symbol() {
        let amount = Amount::<EUR>::parse("€123.45").unwrap();
        assert_eq!(amount.to_minor(), 12345);
    }

    #[test]
    fn test_parse_gbp_symbol() {
        let amount = Amount::<GBP>::parse("£99.99").unwrap();
        assert_eq!(amount.to_minor(), 9999);
    }

    #[test]
    fn test_parse_jpy_symbol() {
        let amount = Amount::<JPY>::parse("¥1000").unwrap();
        assert_eq!(amount.to_major_floor(), 1000);
    }

    #[test]
    fn test_parse_symbol_mismatch() {
        // USD parser with EUR symbol should fail
        let result = Amount::<USD>::parse("€12.34");
        assert!(result.is_err());

        if let Err(e) = result {
            assert!(e.to_string().contains("symbol mismatch"));
        }
    }

    // ========================================================================
    // Parsing Tests - With Currency Code
    // ========================================================================

    #[test]
    fn test_parse_with_code_suffix() {
        let amount = Amount::<USD>::parse("12.34 USD").unwrap();
        assert_eq!(amount.to_minor(), 1234);
    }

    #[test]
    fn test_parse_with_code_prefix() {
        let amount = Amount::<USD>::parse("USD 12.34").unwrap();
        assert_eq!(amount.to_minor(), 1234);
    }

    #[test]
    fn test_parse_code_mismatch() {
        // USD parser with EUR code should fail
        let result = Amount::<USD>::parse("12.34 EUR");
        assert!(result.is_err());

        if let Err(e) = result {
            assert!(e.to_string().contains("code mismatch"));
        }
    }

    // ========================================================================
    // Parsing Tests - Combined Format
    // ========================================================================

    #[test]
    fn test_parse_symbol_and_code() {
        let amount = Amount::<USD>::parse("$12.34 USD").unwrap();
        assert_eq!(amount.to_minor(), 1234);
    }

    #[test]
    fn test_parse_with_whitespace() {
        let amount = Amount::<USD>::parse("  $12.34   USD  ").unwrap();
        assert_eq!(amount.to_minor(), 1234);
    }

    // ========================================================================
    // FromStr Trait Tests
    // ========================================================================

    #[test]
    fn test_fromstr_trait() {
        let amount: Amount<USD> = "12.34".parse().unwrap();
        assert_eq!(amount.to_minor(), 1234);
    }

    #[test]
    fn test_fromstr_with_symbol() {
        let amount: Amount<EUR> = "€99.99".parse().unwrap();
        assert_eq!(amount.to_minor(), 9999);
    }

    // ========================================================================
    // Error Handling Tests
    // ========================================================================

    #[test]
    fn test_parse_empty_string() {
        let result = Amount::<USD>::parse("");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_invalid_number() {
        let result = Amount::<USD>::parse("not a number");
        assert!(result.is_err());

        if let Err(e) = result {
            assert!(matches!(e, MoneyError::ParseError { .. }));
        }
    }

    #[test]
    fn test_parse_multiple_decimals() {
        let result = Amount::<USD>::parse("12.34.56");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_excessive_length() {
        let long_string = "1".repeat(101);
        let result = Amount::<USD>::parse(&long_string);
        assert!(result.is_err());

        if let Err(e) = result {
            assert!(e.to_string().contains("too long"));
        }
    }

    #[test]
    fn test_parse_special_characters() {
        let result = Amount::<USD>::parse("12.34<script>");
        assert!(result.is_err());
    }

    // ========================================================================
    // Determinism Tests
    // ========================================================================

    #[test]
    fn test_parse_deterministic() {
        let amount1 = Amount::<USD>::parse("12.34").unwrap();
        let amount2 = Amount::<USD>::parse("12.34").unwrap();
        assert_eq!(amount1, amount2);
    }

    #[test]
    fn test_parse_different_formats_same_result() {
        let a1 = Amount::<USD>::parse("12.34").unwrap();
        let a2 = Amount::<USD>::parse("$12.34").unwrap();
        let a3 = Amount::<USD>::parse("12.34 USD").unwrap();
        let a4 = Amount::<USD>::parse("$12.34 USD").unwrap();

        assert_eq!(a1, a2);
        assert_eq!(a2, a3);
        assert_eq!(a3, a4);
    }
}
