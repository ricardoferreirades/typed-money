//! Error types for monetary operations.
//!
//! This module provides comprehensive error handling for all fallible operations
//! in the typed-money library. All errors include rich context for debugging
//! and recovery suggestions.
//!
//! # Error Types
//!
//! The main error type is [`MoneyError`], which covers:
//! - Currency mismatches
//! - Precision errors
//! - Invalid rates
//! - Arithmetic overflow
//! - Parsing errors
//! - Rounding errors
//! - Serialization errors
//!
//! # Examples
//!
//! ## Handling Parse Errors
//!
//! ```
//! use typed_money::{Amount, USD, MoneyError};
//! use std::str::FromStr;
//!
//! match Amount::<USD>::from_str("invalid") {
//!     Ok(amount) => println!("Parsed: {}", amount),
//!     Err(MoneyError::ParseError { input, .. }) => {
//!         println!("Failed to parse: {}", input);
//!     }
//!     Err(e) => println!("Other error: {}", e),
//! }
//! ```
//!
//! ## Checking Precision
//!
//! ```
//! use typed_money::{Amount, USD, MoneyError};
//!
//! let amount = Amount::<USD>::from_major(10) / 3;  // Creates excess precision
//!
//! match amount.check_precision() {
//!     Ok(_) => println!("Precision OK"),
//!     Err(MoneyError::PrecisionError { expected, actual, suggestion, .. }) => {
//!         println!("Expected {} decimals, got {}. {}", expected, actual, suggestion);
//!         // Can recover by normalizing
//!         let fixed = amount.normalize();
//!         assert!(fixed.check_precision().is_ok());
//!     }
//!     Err(e) => println!("Other error: {}", e),
//! }
//! ```
//!
//! ## Creating Invalid Rates
//!
//! ```
//! use typed_money::{Rate, USD, EUR, MoneyError};
//!
//! // Rates must be positive
//! match Rate::<USD, EUR>::try_new(-1.0) {
//!     Ok(_) => unreachable!(),
//!     Err(MoneyError::InvalidRate { reason, .. }) => {
//!         assert!(reason.contains("positive"));
//!     }
//!     Err(e) => panic!("Unexpected error: {}", e),
//! }
//! ```

use rust_decimal::Decimal;

#[cfg(not(feature = "std"))]
use crate::inner_prelude::*;
use core::fmt;

/// Result type alias for money operations.
///
/// This is a convenience alias for `Result<T, MoneyError>`.
///
/// # Examples
///
/// ```
/// use typed_money::{MoneyResult, Amount, USD};
///
/// fn parse_amount(s: &str) -> MoneyResult<Amount<USD>> {
///     // Parsing logic would go here
///     Ok(Amount::<USD>::from_major(100))
/// }
/// ```
pub type MoneyResult<T> = Result<T, MoneyError>;

/// Errors that can occur during monetary operations.
///
/// All error variants include context to help diagnose and fix issues.
#[derive(Debug, Clone, PartialEq)]
pub enum MoneyError {
    /// Attempted to perform an operation between incompatible currencies.
    ///
    /// This error is primarily for runtime checks. The type system usually
    /// prevents this at compile time.
    CurrencyMismatch {
        /// The expected currency code
        expected: &'static str,
        /// The actual currency code found
        found: &'static str,
        /// Additional context about the operation
        context: String,
    },

    /// No conversion rate available for the requested currency pair.
    ConversionRateMissing {
        /// The source currency code
        from: &'static str,
        /// The target currency code
        to: &'static str,
    },

    /// Precision would be lost in the operation.
    ///
    /// This warning indicates that an amount has more decimal places
    /// than the currency supports.
    PrecisionError {
        /// The currency code
        currency: &'static str,
        /// Expected precision (number of decimal places)
        expected: u8,
        /// Actual precision found
        actual: u32,
        /// Suggestion for fixing the error
        suggestion: &'static str,
    },

    /// Invalid amount value (NaN, Infinity, or other invalid state).
    InvalidAmount {
        /// Description of what makes the amount invalid
        reason: &'static str,
        /// The currency code if available
        currency: Option<&'static str>,
    },

    /// Failed to parse a string into a monetary amount.
    ParseError {
        /// The input string that failed to parse
        input: String,
        /// The expected currency code
        expected_currency: Option<&'static str>,
        /// Description of why parsing failed
        reason: String,
    },

    /// Rounding operation failed.
    RoundingError {
        /// The currency code
        currency: &'static str,
        /// Description of what went wrong
        reason: &'static str,
    },

    /// Failed to convert rate value to decimals.
    InvalidRateConversion {
        /// The rate value that was invalid
        value: f64,
        /// Description of why the rate is invalid
        reason: &'static str,
    },

    /// Invalid exchange rate value.
    InvalidRate {
        /// The rate value that was invalid
        value: Decimal,
        /// Description of why the rate is invalid
        reason: &'static str,
    },

    /// Arithmetic overflow occurred.
    Overflow {
        /// The operation that caused overflow
        operation: String,
        /// The currency code
        currency: &'static str,
    },

    /// Arithmetic underflow occurred.
    Underflow {
        /// The operation that caused underflow
        operation: String,
        /// The currency code
        currency: &'static str,
    },
}

impl MoneyError {
    /// Returns a suggestion for how to fix this error.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_money::MoneyError;
    ///
    /// let error = MoneyError::InvalidRate {
    ///     value: "0.0".to_string(),
    ///     reason: "Rate must be positive".to_string(),
    /// };
    ///
    /// println!("{}", error.suggestion());
    /// ```
    pub fn suggestion(&self) -> &str {
        match self {
            MoneyError::CurrencyMismatch { .. } => {
                "Ensure both amounts use the same currency, or use explicit conversion with a Rate"
            }
            MoneyError::ConversionRateMissing { .. } => {
                "Provide a Rate instance for the currency conversion"
            }
            MoneyError::PrecisionError { suggestion, .. } => suggestion,
            MoneyError::InvalidAmount { .. } => "Check that the amount is a valid finite number",
            MoneyError::ParseError { .. } => {
                "Ensure the input string is in a valid format (e.g., '12.34' or '$12.34 USD')"
            }
            MoneyError::RoundingError { .. } => {
                "Try a different rounding mode or check the amount precision"
            }
            MoneyError::InvalidRate { .. } | MoneyError::InvalidRateConversion { .. } => {
                "Exchange rates must be positive, finite numbers"
            }
            MoneyError::Overflow { .. } => {
                "Use smaller values or check for logical errors in calculations"
            }
            MoneyError::Underflow { .. } => {
                "Use larger values or check for logical errors in calculations"
            }
        }
    }

    /// Returns the currency code associated with this error, if any.
    pub fn currency(&self) -> Option<&'static str> {
        match self {
            MoneyError::CurrencyMismatch { expected, .. } => Some(expected),
            MoneyError::ConversionRateMissing { from, .. } => Some(from),
            MoneyError::PrecisionError { currency, .. } => Some(currency),
            MoneyError::InvalidAmount { currency, .. } => *currency,
            MoneyError::ParseError {
                expected_currency, ..
            } => *expected_currency,
            MoneyError::RoundingError { currency, .. } => Some(currency),
            MoneyError::InvalidRate { .. } | MoneyError::InvalidRateConversion { .. } => None,
            MoneyError::Overflow { currency, .. } => Some(currency),
            MoneyError::Underflow { currency, .. } => Some(currency),
        }
    }
}

impl fmt::Display for MoneyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MoneyError::CurrencyMismatch {
                expected,
                found,
                context,
            } => {
                write!(
                    f,
                    "Currency mismatch: expected {}, found {} ({})",
                    expected, found, context
                )
            }
            MoneyError::ConversionRateMissing { from, to } => {
                write!(f, "No conversion rate available from {} to {}", from, to)
            }
            MoneyError::PrecisionError {
                currency,
                expected,
                actual,
                ..
            } => {
                write!(
                    f,
                    "Precision error for {}: expected {} decimal places, found {}",
                    currency, expected, actual
                )
            }
            MoneyError::InvalidAmount { reason, currency } => {
                if let Some(curr) = currency {
                    write!(f, "Invalid amount for {}: {}", curr, reason)
                } else {
                    write!(f, "Invalid amount: {}", reason)
                }
            }
            MoneyError::ParseError {
                input,
                expected_currency,
                reason,
            } => {
                if let Some(curr) = expected_currency {
                    write!(f, "Failed to parse '{}' as {}: {}", input, curr, reason)
                } else {
                    write!(f, "Failed to parse '{}': {}", input, reason)
                }
            }
            MoneyError::RoundingError { currency, reason } => {
                write!(f, "Rounding error for {}: {}", currency, reason)
            }
            MoneyError::InvalidRate { value, reason } => {
                write!(f, "Invalid exchange rate '{value:.1}': {reason}")
            }
            MoneyError::Overflow {
                operation,
                currency,
            } => {
                write!(
                    f,
                    "Arithmetic overflow in {} operation for {}",
                    operation, currency
                )
            }
            MoneyError::Underflow {
                operation,
                currency,
            } => {
                write!(
                    f,
                    "Arithmetic underflow in {} operation for {}",
                    operation, currency
                )
            }
            MoneyError::InvalidRateConversion { value, reason } => write!(
                f,
                "Invalid exchange rate conversion from f64 '{}': {}",
                value, reason
            ),
        }
    }
}

impl core::error::Error for MoneyError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        // None of our errors wrap other errors currently
        None
    }
}

#[cfg(test)]
mod tests {
    #[cfg(not(feature = "std"))]
    use crate::inner_prelude::*;

    use super::*;

    #[test]
    fn test_currency_mismatch_display() {
        let error = MoneyError::CurrencyMismatch {
            expected: "USD",
            found: "EUR",
            context: "addition".to_string(),
        };

        assert_eq!(
            &error.to_string(),
            "Currency mismatch: expected USD, found EUR (addition)"
        );
    }

    #[test]
    fn test_conversion_rate_missing_display() {
        let error = MoneyError::ConversionRateMissing {
            from: "USD",
            to: "JPY",
        };

        assert_eq!(
            &error.to_string(),
            "No conversion rate available from USD to JPY"
        );
    }

    #[test]
    fn test_precision_error_display() {
        let error = MoneyError::PrecisionError {
            currency: "USD",
            expected: 2,
            actual: 5,
            suggestion: "Use normalize() or round()",
        };

        assert_eq!(
            &error.to_string(),
            "Precision error for USD: expected 2 decimal places, found 5"
        );
    }

    #[test]
    fn test_invalid_amount_display() {
        let error = MoneyError::InvalidAmount {
            reason: "Value is NaN",
            currency: Some("EUR"),
        };

        assert_eq!(&error.to_string(), "Invalid amount for EUR: Value is NaN");
    }

    #[test]
    fn test_parse_error_display() {
        let error = MoneyError::ParseError {
            input: "not a number".to_string(),
            expected_currency: Some("USD"),
            reason: "Contains non-numeric characters".to_string(),
        };

        assert_eq!(
            &error.to_string(),
            "Failed to parse 'not a number' as USD: Contains non-numeric characters"
        );
    }

    #[test]
    fn test_invalid_rate_display() {
        let error = MoneyError::InvalidRate {
            value: Decimal::ZERO,
            reason: "Rate must be positive",
        };

        assert_eq!(
            &error.to_string(),
            "Invalid exchange rate '0.0': Rate must be positive"
        );
    }

    #[test]
    fn test_overflow_display() {
        let error = MoneyError::Overflow {
            operation: "multiplication".to_string(),
            currency: "BTC",
        };

        assert_eq!(
            &error.to_string(),
            "Arithmetic overflow in multiplication operation for BTC"
        );
    }

    #[test]
    fn test_suggestion() {
        let error = MoneyError::CurrencyMismatch {
            expected: "USD",
            found: "EUR",
            context: "test".to_string(),
        };

        assert!(error.suggestion().contains("same currency"));
    }

    #[test]
    fn test_currency_extraction() {
        let error = MoneyError::PrecisionError {
            currency: "USD",
            expected: 2,
            actual: 5,
            suggestion: "test",
        };

        assert_eq!(error.currency(), Some("USD"));
    }

    #[test]
    fn test_error_trait_implementation() {
        let error = MoneyError::InvalidAmount {
            reason: "test",
            currency: None,
        };

        // Should implement Error trait
        let _: &dyn core::error::Error = &error;
    }

    #[test]
    fn test_money_result_alias() {
        fn example() -> MoneyResult<i32> {
            Ok(42)
        }

        assert_eq!(example().unwrap(), 42);
    }

    #[test]
    fn test_error_clone() {
        let error = MoneyError::InvalidRate {
            value: Decimal::ZERO,
            reason: "test",
        };

        let cloned = error.clone();
        assert_eq!(error, cloned);
    }

    #[test]
    fn test_error_debug() {
        let error = MoneyError::InvalidAmount {
            reason: "test",
            currency: Some("USD"),
        };

        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("InvalidAmount"));
    }
}
