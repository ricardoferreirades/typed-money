//! Rounding modes for monetary calculations.

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
