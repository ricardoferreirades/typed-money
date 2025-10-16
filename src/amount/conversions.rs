//! Conversion methods for Amount.

use super::type_def::Amount;
use crate::{Currency, RoundingMode};

#[cfg(all(feature = "use_rust_decimal", not(feature = "use_bigdecimal")))]
use rust_decimal::Decimal;

#[cfg(all(feature = "use_bigdecimal", not(feature = "use_rust_decimal")))]
use bigdecimal::BigDecimal as Decimal;

impl<C: Currency> Amount<C> {
    /// Returns the amount in major units, rounding according to the specified mode.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_money::{Amount, USD, RoundingMode};
    ///
    /// let amount = Amount::<USD>::from_minor(12345);  // $123.45
    /// assert_eq!(amount.to_major_rounded(RoundingMode::Floor), 123);
    /// assert_eq!(amount.to_major_rounded(RoundingMode::HalfUp), 123);
    ///
    /// let amount2 = Amount::<USD>::from_minor(12399);  // $123.99
    /// assert_eq!(amount2.to_major_rounded(RoundingMode::Floor), 123);
    /// assert_eq!(amount2.to_major_rounded(RoundingMode::HalfUp), 124);
    /// ```
    #[cfg(feature = "use_rust_decimal")]
    pub fn to_major_rounded(&self, mode: RoundingMode) -> i64 {
        let rounded = match mode {
            RoundingMode::HalfUp => self.value.round_dp(0),
            RoundingMode::HalfDown => {
                // Round half down: round to nearest, ties toward zero
                let abs_value = self.value.abs();
                let rounded_abs = abs_value
                    .round_dp_with_strategy(0, rust_decimal::RoundingStrategy::MidpointTowardZero);
                if self.value.is_sign_negative() {
                    -rounded_abs
                } else {
                    rounded_abs
                }
            }
            RoundingMode::HalfEven => {
                // Banker's rounding
                self.value
                    .round_dp_with_strategy(0, rust_decimal::RoundingStrategy::MidpointNearestEven)
            }
            RoundingMode::Floor => self.value.trunc(),
            RoundingMode::Ceiling => self.value.ceil(),
        };

        rounded.to_string().parse().unwrap_or(0)
    }

    /// Returns the amount in major units, truncating (flooring) any decimals.
    ///
    /// ⚠️ **Warning**: This discards fractional amounts without rounding.
    /// For proper rounding, use `to_major_rounded()` or the specific rounding methods.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_money::{Amount, USD};
    ///
    /// let amount = Amount::<USD>::from_minor(12399);  // $123.99
    /// assert_eq!(amount.to_major_floor(), 123);  // Lost $0.99!
    /// ```
    #[cfg(feature = "use_rust_decimal")]
    pub fn to_major_floor(&self) -> i64 {
        self.to_major_rounded(RoundingMode::Floor)
    }

    /// Returns the amount in major units, rounding half up.
    ///
    /// This is the most common rounding mode: 0.5 rounds up to 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_money::{Amount, USD};
    ///
    /// let amount1 = Amount::<USD>::from_minor(12350);  // $123.50
    /// assert_eq!(amount1.to_major_half_up(), 124);     // 0.50 rounds up
    ///
    /// let amount2 = Amount::<USD>::from_minor(12349);  // $123.49
    /// assert_eq!(amount2.to_major_half_up(), 123);     // 0.49 rounds down
    /// ```
    #[cfg(feature = "use_rust_decimal")]
    pub fn to_major_half_up(&self) -> i64 {
        self.to_major_rounded(RoundingMode::HalfUp)
    }

    /// Returns the amount in major units, rounding half down.
    ///
    /// When exactly halfway, rounds towards zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_money::{Amount, USD};
    ///
    /// let amount1 = Amount::<USD>::from_minor(12350);  // $123.50
    /// assert_eq!(amount1.to_major_half_down(), 123);   // 0.50 rounds down
    ///
    /// let amount2 = Amount::<USD>::from_minor(12351);  // $123.51
    /// assert_eq!(amount2.to_major_half_down(), 124);   // 0.51 rounds up
    /// ```
    #[cfg(feature = "use_rust_decimal")]
    pub fn to_major_half_down(&self) -> i64 {
        self.to_major_rounded(RoundingMode::HalfDown)
    }

    /// Returns the amount in major units using banker's rounding (round half to even).
    ///
    /// This minimizes cumulative rounding errors by rounding to the nearest even number
    /// when exactly halfway between two integers.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_money::{Amount, USD};
    ///
    /// let amount1 = Amount::<USD>::from_minor(12350);  // $123.50
    /// assert_eq!(amount1.to_major_half_even(), 124);   // Rounds to even (124)
    ///
    /// let amount2 = Amount::<USD>::from_minor(12250);  // $122.50
    /// assert_eq!(amount2.to_major_half_even(), 122);   // Rounds to even (122)
    /// ```
    #[cfg(feature = "use_rust_decimal")]
    pub fn to_major_half_even(&self) -> i64 {
        self.to_major_rounded(RoundingMode::HalfEven)
    }

    /// Returns the amount in major units, rounding towards positive infinity (ceiling).
    ///
    /// Always rounds up if there are any decimal places.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_money::{Amount, USD};
    ///
    /// let amount1 = Amount::<USD>::from_minor(12301);  // $123.01
    /// assert_eq!(amount1.to_major_ceiling(), 124);     // Any decimals round up
    ///
    /// let amount2 = Amount::<USD>::from_minor(12399);  // $123.99
    /// assert_eq!(amount2.to_major_ceiling(), 124);     // Rounds up
    ///
    /// let amount3 = Amount::<USD>::from_minor(12300);  // $123.00
    /// assert_eq!(amount3.to_major_ceiling(), 123);     // No decimals, stays same
    /// ```
    #[cfg(feature = "use_rust_decimal")]
    pub fn to_major_ceiling(&self) -> i64 {
        self.to_major_rounded(RoundingMode::Ceiling)
    }

    /// Returns the amount in minor units as an integer.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_money::{Amount, USD};
    ///
    /// let amount = Amount::<USD>::from_major(123);  // $123.00
    /// assert_eq!(amount.to_minor(), 12300);  // 12300 cents
    /// ```
    pub fn to_minor(&self) -> i64 {
        if C::DECIMALS == 0 {
            self.value.to_string().parse().unwrap_or(0)
        } else {
            let scaled = self.value * Decimal::from(10_i64.pow(C::DECIMALS.into()));
            scaled.trunc().to_string().parse().unwrap_or(0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RoundingMode, USD};

    #[test]
    fn test_to_major_floor() {
        let amount = Amount::<USD>::from_minor(12345); // $123.45
        assert_eq!(amount.to_major_floor(), 123);

        let amount2 = Amount::<USD>::from_minor(12399); // $123.99
        assert_eq!(amount2.to_major_floor(), 123); // Truncates, not rounds
    }

    #[test]
    fn test_to_major_half_up() {
        let amount1 = Amount::<USD>::from_minor(12350); // $123.50
        assert_eq!(amount1.to_major_half_up(), 124); // 0.50 rounds up

        let amount2 = Amount::<USD>::from_minor(12349); // $123.49
        assert_eq!(amount2.to_major_half_up(), 123); // 0.49 rounds down

        let amount3 = Amount::<USD>::from_minor(12351); // $123.51
        assert_eq!(amount3.to_major_half_up(), 124); // 0.51 rounds up
    }

    #[test]
    fn test_to_major_half_down() {
        let amount1 = Amount::<USD>::from_minor(12350); // $123.50
        assert_eq!(amount1.to_major_half_down(), 123); // 0.50 rounds down

        let amount2 = Amount::<USD>::from_minor(12351); // $123.51
        assert_eq!(amount2.to_major_half_down(), 124); // 0.51 rounds up

        let amount3 = Amount::<USD>::from_minor(12349); // $123.49
        assert_eq!(amount3.to_major_half_down(), 123); // 0.49 rounds down
    }

    #[test]
    fn test_to_major_half_even() {
        // Banker's rounding rounds to nearest even
        let amount1 = Amount::<USD>::from_minor(12350); // $123.50
        assert_eq!(amount1.to_major_half_even(), 124); // Rounds to even (124)

        let amount2 = Amount::<USD>::from_minor(12250); // $122.50
        assert_eq!(amount2.to_major_half_even(), 122); // Rounds to even (122)

        let amount3 = Amount::<USD>::from_minor(12450); // $124.50
        assert_eq!(amount3.to_major_half_even(), 124); // Rounds to even (124)

        let amount4 = Amount::<USD>::from_minor(12549); // $125.49
        assert_eq!(amount4.to_major_half_even(), 125); // Not halfway, rounds down
    }

    #[test]
    fn test_to_major_ceiling() {
        let amount1 = Amount::<USD>::from_minor(12301); // $123.01
        assert_eq!(amount1.to_major_ceiling(), 124); // Any decimals round up

        let amount2 = Amount::<USD>::from_minor(12399); // $123.99
        assert_eq!(amount2.to_major_ceiling(), 124); // Rounds up

        let amount3 = Amount::<USD>::from_minor(12300); // $123.00
        assert_eq!(amount3.to_major_ceiling(), 123); // No decimals, stays same

        let amount4 = Amount::<USD>::from_minor(12350); // $123.50
        assert_eq!(amount4.to_major_ceiling(), 124); // Half rounds up
    }

    #[test]
    fn test_to_major_rounded_with_mode() {
        let amount = Amount::<USD>::from_minor(12350); // $123.50

        assert_eq!(amount.to_major_rounded(RoundingMode::HalfUp), 124);
        assert_eq!(amount.to_major_rounded(RoundingMode::HalfDown), 123);
        assert_eq!(amount.to_major_rounded(RoundingMode::HalfEven), 124);
        assert_eq!(amount.to_major_rounded(RoundingMode::Floor), 123);
        assert_eq!(amount.to_major_rounded(RoundingMode::Ceiling), 124);
    }

    #[test]
    fn test_to_minor() {
        let amount = Amount::<USD>::from_major(123); // $123.00
        assert_eq!(amount.to_minor(), 12300); // 12300 cents
    }

    // Determinism tests
    #[test]
    fn test_large_numbers_determinism() {
        // Test with large numbers to ensure no platform-specific overflow
        let large = Amount::<USD>::from_minor(999_999_999_999_999);
        assert_eq!(large.to_minor(), 999_999_999_999_999);

        let large_major = large.to_major_floor();
        assert_eq!(large_major, 9_999_999_999_999);
    }

    #[test]
    fn test_negative_numbers_determinism() {
        // Verify negative numbers work consistently
        let neg = Amount::<USD>::from_major(-100);
        assert_eq!(neg.value().to_string(), "-100");
        assert_eq!(neg.to_major_floor(), -100);
        assert_eq!(neg.to_minor(), -10000);
    }

    #[test]
    fn test_rounding_determinism() {
        // Verify rounding produces consistent results
        let amount = Amount::<USD>::from_minor(12350); // $123.50

        // These should always produce the same results on all platforms
        assert_eq!(amount.to_major_rounded(RoundingMode::HalfUp), 124);
        assert_eq!(amount.to_major_rounded(RoundingMode::HalfDown), 123);
        assert_eq!(amount.to_major_rounded(RoundingMode::HalfEven), 124);
        assert_eq!(amount.to_major_rounded(RoundingMode::Floor), 123);
        assert_eq!(amount.to_major_rounded(RoundingMode::Ceiling), 124);
    }

    #[test]
    fn test_conversion_round_trip_determinism() {
        // Verify major -> minor -> major round trip
        let original = Amount::<USD>::from_major(123);
        let minor = original.to_minor();
        let back_to_major = Amount::<USD>::from_minor(minor);

        assert_eq!(original, back_to_major);
    }

    #[test]
    fn test_decimal_string_representation_determinism() {
        // Verify consistent string representation across platforms
        let amount = Amount::<USD>::from_minor(12345);
        let str_repr = amount.value().to_string();

        // Decimal should always produce the same string
        assert_eq!(str_repr, "123.45");
    }
}
