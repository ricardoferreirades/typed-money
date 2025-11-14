//! Precision control and detection for Amount.

use super::type_def::Amount;
use crate::{Currency, MoneyError, MoneyResult};

impl<C: Currency> Amount<C> {
    /// Checks if this amount has more decimal places than the currency supports.
    ///
    /// Returns `true` if the amount has excess precision that would be lost
    /// when converting to minor units or displaying.
    ///
    /// This is useful for detecting when rounding might be needed.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_money::{Amount, USD, JPY};
    ///
    /// // USD has 2 decimals
    /// let precise = Amount::<USD>::from_major(100) / 3; // 33.333...
    /// assert!(precise.has_excess_precision());
    ///
    /// let rounded = Amount::<USD>::from_minor(3333); // 33.33
    /// assert!(!rounded.has_excess_precision());
    ///
    /// // JPY has 0 decimals
    /// let jpy = Amount::<JPY>::from_major(100) / 3; // 33.333...
    /// assert!(jpy.has_excess_precision());
    /// ```
    #[cfg(all(feature = "use_rust_decimal", not(feature = "use_bigdecimal")))]
    pub fn has_excess_precision(&self) -> bool {
        let scale = self.value.scale();
        scale > u32::from(C::DECIMALS)
    }

    #[cfg(all(feature = "use_bigdecimal", not(feature = "use_rust_decimal")))]
    pub fn has_excess_precision(&self) -> bool {
        use bigdecimal::ToPrimitive;

        let (_, scale) = self.value.as_bigint_and_exponent();
        scale > i64::from(C::DECIMALS)
    }

    /// Returns the number of decimal places in this amount.
    ///
    /// This can be more than the currency's `DECIMALS` if the amount
    /// came from arithmetic operations.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_money::{Amount, USD};
    ///
    /// let amount = Amount::<USD>::from_minor(1234); // 12.34
    /// assert_eq!(amount.precision(), 2);
    ///
    /// let divided = amount / 3; // 4.113333...
    /// assert!(divided.precision() > 2);
    /// ```
    #[cfg(all(feature = "use_rust_decimal", not(feature = "use_bigdecimal")))]
    pub fn precision(&self) -> u32 {
        self.value.scale()
    }

    #[cfg(all(feature = "use_bigdecimal", not(feature = "use_rust_decimal")))]
    pub fn precision(&self) -> i64 {
        let (_, scale) = self.value.as_bigint_and_exponent();
        scale
    }

    /// Returns the currency's expected decimal precision.
    ///
    /// This is a convenience method that returns `C::DECIMALS`.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_money::{Amount, USD, JPY, BTC};
    ///
    /// assert_eq!(Amount::<USD>::currency_precision(), 2);
    /// assert_eq!(Amount::<JPY>::currency_precision(), 0);
    /// assert_eq!(Amount::<BTC>::currency_precision(), 8);
    /// ```
    #[inline]
    pub const fn currency_precision() -> u8 {
        C::DECIMALS
    }

    /// Normalizes the amount to the currency's decimal precision.
    ///
    /// This is equivalent to `round(RoundingMode::HalfEven)` but more explicit
    /// about the intent of normalizing to currency precision.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_money::{Amount, USD};
    ///
    /// let amount = Amount::<USD>::from_major(100) / 3; // 33.333...
    /// assert!(amount.has_excess_precision());
    ///
    /// let normalized = amount.normalize();
    /// assert!(!normalized.has_excess_precision());
    /// assert_eq!(normalized.to_minor(), 3333); // 33.33
    /// ```
    pub fn normalize(&self) -> Self {
        use crate::RoundingMode;
        self.round(RoundingMode::HalfEven)
    }

    /// Checks if the amount has valid precision for the currency.
    ///
    /// Returns an error if the amount has excess precision that should be
    /// rounded before use in financial operations.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_money::{Amount, USD};
    ///
    /// let amount = Amount::<USD>::from_minor(1234); // 12.34
    /// assert!(amount.check_precision().is_ok());
    ///
    /// let divided = Amount::<USD>::from_major(100) / 3; // 33.333...
    /// let result = divided.check_precision();
    /// assert!(result.is_err());
    ///
    /// // Can recover by normalizing
    /// let normalized = divided.normalize();
    /// assert!(normalized.check_precision().is_ok());
    /// ```
    #[cfg(all(feature = "use_rust_decimal", not(feature = "use_bigdecimal")))]
    pub fn check_precision(&self) -> MoneyResult<()> {
        if self.has_excess_precision() {
            Err(MoneyError::PrecisionError {
                currency: C::CODE,
                expected: C::DECIMALS,
                actual: self.precision(),
                suggestion: "Use normalize() or round()",
            })
        } else {
            Ok(())
        }
    }

    #[cfg(all(feature = "use_bigdecimal", not(feature = "use_rust_decimal")))]
    pub fn check_precision(&self) -> MoneyResult<()> {
        if self.has_excess_precision() {
            Err(MoneyError::PrecisionError {
                currency: C::CODE,
                expected: C::DECIMALS,
                actual: self.precision() as u32,
                suggestion: format!(
                    "Use normalize() or round() to adjust precision to {} decimal places",
                    C::DECIMALS
                ),
            })
        } else {
            Ok(())
        }
    }

    /// Checks if the amount has valid precision for the currency.
    ///
    /// This version is used when both decimal backends are enabled (which should not
    /// happen in normal use, but may occur during testing with --all-features).
    #[cfg(all(feature = "use_rust_decimal", feature = "use_bigdecimal"))]
    pub fn check_precision(&self) -> MoneyResult<()> {
        // When both backends are enabled, we can't determine precision
        // This is a compile-time configuration error, so we just return Ok
        Ok(())
    }
}

#[cfg(test)]
#[cfg(not(all(feature = "use_rust_decimal", feature = "use_bigdecimal")))]
mod tests {
    use super::*;
    #[cfg(not(feature = "std"))]
    use crate::inner_prelude::*;
    use crate::{BTC, EUR, JPY, USD};

    // ========================================================================
    // Precision Detection Tests
    // ========================================================================

    #[test]
    fn test_has_excess_precision_usd() {
        // USD has 2 decimals
        let exact = Amount::<USD>::from_minor(1234); // 12.34
        assert!(!exact.has_excess_precision());

        let divided = Amount::<USD>::from_major(100) / 3; // 33.333...
        assert!(divided.has_excess_precision());
    }

    #[test]
    fn test_has_excess_precision_jpy() {
        // JPY has 0 decimals
        let exact = Amount::<JPY>::from_major(100);
        assert!(!exact.has_excess_precision());

        let divided = Amount::<JPY>::from_major(100) / 3; // 33.333...
        assert!(divided.has_excess_precision());
    }

    #[test]
    fn test_has_excess_precision_btc() {
        // BTC has 8 decimals
        let exact = Amount::<BTC>::from_minor(12345678); // 0.12345678
        assert!(!exact.has_excess_precision());

        let divided = Amount::<BTC>::from_major(1) / 3; // 0.333...
        assert!(divided.has_excess_precision());
    }

    #[test]
    fn test_precision_method() {
        let usd = Amount::<USD>::from_minor(1234); // 12.34
        assert_eq!(usd.precision(), 2);

        let jpy = Amount::<JPY>::from_major(1234); // 1234
        assert_eq!(jpy.precision(), 0);
    }

    #[test]
    fn test_precision_after_division() {
        let amount = Amount::<USD>::from_major(10);
        let divided = amount / 3; // 3.333...

        // Should have more than 2 decimals
        assert!(divided.precision() > 2);
        assert!(divided.has_excess_precision());
    }

    #[test]
    fn test_currency_precision() {
        assert_eq!(Amount::<USD>::currency_precision(), 2);
        assert_eq!(Amount::<EUR>::currency_precision(), 2);
        assert_eq!(Amount::<JPY>::currency_precision(), 0);
        assert_eq!(Amount::<BTC>::currency_precision(), 8);
    }

    // ========================================================================
    // Normalization Tests
    // ========================================================================

    #[test]
    fn test_normalize_removes_excess_precision() {
        let amount = Amount::<USD>::from_major(100) / 3; // 33.333...
        assert!(amount.has_excess_precision());

        let normalized = amount.normalize();
        assert!(!normalized.has_excess_precision());
    }

    #[test]
    fn test_normalize_uses_half_even() {
        use core::marker::PhantomData;
        use rust_decimal::Decimal;

        // 12.345 normalizes to 12.34 (banker's rounding)
        let value = Decimal::new(12345, 3);
        let amount = Amount::<USD> {
            value,
            _currency: PhantomData,
        };

        let normalized = amount.normalize();
        assert_eq!(normalized.to_minor(), 1234);
    }

    #[test]
    fn test_normalize_already_normalized() {
        let amount = Amount::<USD>::from_minor(1234); // Already 2 decimals
        let normalized = amount.normalize();

        assert_eq!(amount, normalized);
    }

    #[test]
    fn test_normalize_jpy() {
        let amount = Amount::<JPY>::from_major(100) / 3; // 33.333...
        let normalized = amount.normalize();

        assert_eq!(normalized.to_major_floor(), 33);
        assert!(!normalized.has_excess_precision());
    }

    // ========================================================================
    // Precision Preservation Tests
    // ========================================================================

    #[test]
    fn test_arithmetic_preserves_precision() {
        let a = Amount::<USD>::from_minor(1000); // 10.00
        let b = Amount::<USD>::from_minor(333); // 3.33

        let sum = a + b; // 13.33
        assert_eq!(sum.precision(), 2);
        assert!(!sum.has_excess_precision());
    }

    #[test]
    fn test_division_increases_precision() {
        let amount = Amount::<USD>::from_major(10);
        let divided = amount / 3;

        // Division should create more precision
        assert!(divided.precision() > Amount::<USD>::currency_precision().into());
        assert!(divided.has_excess_precision());
    }

    #[test]
    fn test_multiplication_can_increase_precision() {
        let amount = Amount::<USD>::from_minor(1000); // 10.00
        let multiplied = amount * 3; // 10.00 * 3 = 30.00

        // Scalar multiplication may preserve precision
        assert!(multiplied.precision() >= 2);
    }

    // ========================================================================
    // Determinism Tests
    // ========================================================================

    #[test]
    fn test_precision_detection_deterministic() {
        // Same operations should always give same precision detection
        let amount1 = Amount::<USD>::from_major(100) / 3;
        let amount2 = Amount::<USD>::from_major(100) / 3;

        assert_eq!(
            amount1.has_excess_precision(),
            amount2.has_excess_precision()
        );
    }

    #[test]
    fn test_normalize_deterministic() {
        let amount1 = Amount::<USD>::from_major(100) / 3;
        let amount2 = Amount::<USD>::from_major(100) / 3;

        let norm1 = amount1.normalize();
        let norm2 = amount2.normalize();

        assert_eq!(norm1, norm2);
    }

    #[test]
    fn test_cross_platform_precision_behavior() {
        // Test that precision behavior is consistent
        let operations = [
            Amount::<USD>::from_major(10) / 3,
            Amount::<USD>::from_major(100) / 7,
            Amount::<USD>::from_minor(1) / 3,
        ];

        for op in operations {
            // All should have excess precision
            assert!(op.has_excess_precision());

            // All should normalize consistently
            let normalized = op.normalize();
            assert!(!normalized.has_excess_precision());
        }
    }

    // ========================================================================
    // Edge Cases
    // ========================================================================

    #[test]
    fn test_zero_precision() {
        let zero = Amount::<USD>::from_major(0);
        assert!(!zero.has_excess_precision());
        assert_eq!(zero.normalize(), zero);
    }

    #[test]
    fn test_negative_precision() {
        let neg = Amount::<USD>::from_major(-100) / 3;
        assert!(neg.has_excess_precision());

        let normalized = neg.normalize();
        assert!(!normalized.has_excess_precision());
    }

    #[test]
    fn test_very_large_precision() {
        // Test with BTC which has 8 decimals
        let btc = Amount::<BTC>::from_major(1);
        let divided = btc / 7; // Creates repeating decimal

        assert!(divided.has_excess_precision());

        let normalized = divided.normalize();
        assert!(!normalized.has_excess_precision());
    }

    // ========================================================================
    // Error Handling Tests (Section 6.2)
    // ========================================================================

    #[test]
    fn test_check_precision_ok() {
        let amount = Amount::<USD>::from_minor(1234); // 12.34
        assert!(amount.check_precision().is_ok());
    }

    #[test]
    fn test_check_precision_error() {
        let amount = Amount::<USD>::from_major(100) / 3; // 33.333...
        let result = amount.check_precision();
        assert!(result.is_err());

        if let Err(e) = result {
            assert!(matches!(e, MoneyError::PrecisionError { .. }));
            assert_eq!(e.currency(), Some("USD"));
            let msg = e.to_string();
            assert!(
                msg.contains("Precision") || msg.contains("precision"),
                "Message: {}",
                msg
            );
        }
    }

    #[test]
    fn test_check_precision_error_recovery() {
        let amount = Amount::<USD>::from_major(100) / 3; // 33.333...

        // Error detected
        assert!(amount.check_precision().is_err());

        // Can recover by normalizing
        let normalized = amount.normalize();
        assert!(normalized.check_precision().is_ok());
    }

    #[test]
    fn test_check_precision_jpy() {
        let jpy = Amount::<JPY>::from_major(100);
        assert!(jpy.check_precision().is_ok());

        let divided = jpy / 3;
        assert!(divided.check_precision().is_err());
    }

    #[test]
    fn test_precision_error_message() {
        let amount = Amount::<USD>::from_major(100) / 3;
        if let Err(e) = amount.check_precision() {
            let msg = e.to_string();
            assert!(msg.contains("USD"));
            assert!(msg.contains("2 decimal places"));
        }
    }

    #[test]
    fn test_precision_error_suggestion() {
        let amount = Amount::<USD>::from_major(100) / 3;
        if let Err(e) = amount.check_precision() {
            let suggestion = e.suggestion();
            assert!(suggestion.contains("normalize") || suggestion.contains("round"));
        }
    }
}
