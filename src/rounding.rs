//! Rounding modes for monetary calculations.
//!
//! This module provides various rounding strategies for handling decimal precision
//! in monetary amounts. Different rounding modes are appropriate for different
//! use cases and regulatory requirements.
//!
//! # Choosing a Rounding Mode
//!
//! - **HalfEven** (Banker's Rounding) - Recommended for most financial applications
//!   as it minimizes cumulative rounding errors
//! - **HalfUp** - Common in retail and consumer applications
//! - **Floor/Ceiling** - Useful for conservative estimates (always round down/up)
//! - **Down** (Truncate) - Fast but can accumulate errors
//! - **Up** - Conservative rounding away from zero
//!
//! # Examples
//!
//! ```
//! use typed_money::{Amount, USD, RoundingMode};
//!
//! let amount = Amount::<USD>::from_major(10) / 3;  // $3.333...
//!
//! // Different rounding modes produce different results
//! assert_eq!(amount.round(RoundingMode::HalfUp).to_minor(), 333);
//! assert_eq!(amount.round(RoundingMode::HalfDown).to_minor(), 333);
//! assert_eq!(amount.round(RoundingMode::Floor).to_minor(), 333);
//! assert_eq!(amount.round(RoundingMode::Ceiling).to_minor(), 334);
//! ```
//!
//! # Edge Cases
//!
//! ## Negative Numbers
//!
//! ```
//! use typed_money::{Amount, USD, RoundingMode};
//!
//! let negative = Amount::<USD>::from_major(-10) / 3;  // -$3.333...
//!
//! // Floor and Ceiling behave differently with negatives
//! assert_eq!(negative.round(RoundingMode::Floor).to_minor(), -334);   // More negative
//! assert_eq!(negative.round(RoundingMode::Ceiling).to_minor(), -333); // Less negative
//! ```
//!
//! ## Zero-Decimal Currencies
//!
//! ```
//! use typed_money::{Amount, JPY, RoundingMode};
//!
//! // JPY has no decimal places
//! let yen = Amount::<JPY>::from_minor(1000);
//! let divided = yen / 3;  // 333.333...
//!
//! // Rounding respects currency precision
//! assert_eq!(divided.round(RoundingMode::HalfUp).to_minor(), 333);
//! ```

/// Rounding modes for decimal operations.
///
/// Different rounding strategies are appropriate for different use cases.
/// Financial applications often use `HalfEven` (banker's rounding) to minimize bias.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoundingMode {
    /// Round half towards positive infinity.
    ///
    /// Also known as "round half up" or "arithmetic rounding".
    ///
    /// # Examples
    /// - 2.5 → 3
    /// - 2.4 → 2
    /// - -2.5 → -2
    HalfUp,

    /// Round half towards zero.
    ///
    /// # Examples
    /// - 2.5 → 2
    /// - 2.6 → 3
    /// - -2.5 → -2
    HalfDown,

    /// Round half to nearest even number.
    ///
    /// Also known as "banker's rounding". This mode minimizes
    /// cumulative rounding errors in repeated operations.
    ///
    /// # Examples
    /// - 2.5 → 2 (even)
    /// - 3.5 → 4 (even)
    /// - 4.5 → 4 (even)
    HalfEven,

    /// Round towards positive infinity (always round up).
    ///
    /// Always rounds away from zero, regardless of sign.
    ///
    /// # Examples
    /// - 2.1 → 3
    /// - 2.9 → 3
    /// - -2.1 → -3
    /// - -2.9 → -3
    Up,

    /// Round towards zero (truncate).
    ///
    /// Always rounds towards zero, regardless of sign.
    ///
    /// # Examples
    /// - 2.1 → 2
    /// - 2.9 → 2
    /// - -2.1 → -2
    /// - -2.9 → -2
    Down,

    /// Round towards negative infinity (floor).
    ///
    /// Always rounds down to the nearest integer.
    ///
    /// # Examples
    /// - 2.1 → 2
    /// - 2.9 → 2
    /// - -2.1 → -3
    /// - -2.9 → -3
    Floor,

    /// Round towards positive infinity (ceiling).
    ///
    /// Always rounds up to the nearest integer.
    ///
    /// # Examples
    /// - 2.1 → 3
    /// - 2.9 → 3
    /// - -2.1 → -2
    /// - -2.9 → -2
    Ceiling,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rounding_mode_copy() {
        let mode1 = RoundingMode::HalfUp;
        let mode2 = mode1;
        assert_eq!(mode1, mode2);
    }

    #[test]
    fn test_rounding_mode_equality() {
        assert_eq!(RoundingMode::HalfUp, RoundingMode::HalfUp);
        assert_ne!(RoundingMode::HalfUp, RoundingMode::HalfDown);
    }
}
