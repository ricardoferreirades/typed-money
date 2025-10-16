//! Rounding methods for Amount.

use super::type_def::Amount;
use crate::{Currency, RoundingMode};
use std::marker::PhantomData;

impl<C: Currency> Amount<C> {
    /// Rounds the amount to the currency's decimal precision using the specified rounding mode.
    ///
    /// The rounding is applied based on the currency's `DECIMALS` constant. For example:
    /// - USD (2 decimals): $12.345 → $12.35
    /// - JPY (0 decimals): ¥12.7 → ¥13
    /// - BTC (8 decimals): rounds to 8 decimal places
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_money::{Amount, USD, JPY, RoundingMode};
    ///
    /// // USD has 2 decimal places - round 12.3456 to 12.35
    /// let amount = Amount::<USD>::from_minor(12345) + Amount::<USD>::from_minor(1);
    /// let divided = amount / 10; // Creates 1234.6 cents = $12.346
    /// let rounded = divided.round(RoundingMode::HalfUp);
    /// // Note: Since we can't create arbitrary decimals via public API easily,
    /// // this example shows the concept. In practice, rounding is used after
    /// // arithmetic operations that may introduce extra decimal places.
    ///
    /// // JPY has 0 decimal places
    /// let jpy = Amount::<JPY>::from_major(127) / 10; // 12.7
    /// let rounded = jpy.round(RoundingMode::HalfEven);
    /// assert_eq!(rounded.to_major_floor(), 13); // ¥13 (rounds to even)
    /// ```
    ///
    /// # Rounding Modes
    ///
    /// See [`RoundingMode`] for detailed documentation on each mode.
    pub fn round(&self, mode: RoundingMode) -> Self {
        let scale = u32::from(C::DECIMALS);

        #[cfg(feature = "use_rust_decimal")]
        let rounded_value = {
            use rust_decimal::prelude::*;

            match mode {
                RoundingMode::HalfUp => self
                    .value
                    .round_dp_with_strategy(scale, RoundingStrategy::MidpointAwayFromZero),
                RoundingMode::HalfDown => self
                    .value
                    .round_dp_with_strategy(scale, RoundingStrategy::MidpointTowardZero),
                RoundingMode::HalfEven => self
                    .value
                    .round_dp_with_strategy(scale, RoundingStrategy::MidpointNearestEven),
                RoundingMode::Up => self
                    .value
                    .round_dp_with_strategy(scale, RoundingStrategy::AwayFromZero),
                RoundingMode::Down => self
                    .value
                    .round_dp_with_strategy(scale, RoundingStrategy::ToZero),
                RoundingMode::Floor => self
                    .value
                    .round_dp_with_strategy(scale, RoundingStrategy::ToNegativeInfinity),
                RoundingMode::Ceiling => self
                    .value
                    .round_dp_with_strategy(scale, RoundingStrategy::ToPositiveInfinity),
            }
        };

        #[cfg(feature = "use_bigdecimal")]
        let rounded_value = {
            use bigdecimal::RoundingMode as BigDecimalRoundingMode;

            let bigdecimal_mode = match mode {
                RoundingMode::HalfUp => BigDecimalRoundingMode::HalfUp,
                RoundingMode::HalfDown => BigDecimalRoundingMode::HalfDown,
                RoundingMode::HalfEven => BigDecimalRoundingMode::HalfEven,
                RoundingMode::Up => BigDecimalRoundingMode::Up,
                RoundingMode::Down => BigDecimalRoundingMode::Down,
                RoundingMode::Floor => BigDecimalRoundingMode::Floor,
                RoundingMode::Ceiling => BigDecimalRoundingMode::Ceiling,
            };

            self.value.with_scale_round(scale.into(), bigdecimal_mode)
        };

        Self {
            value: rounded_value,
            _currency: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BTC, EUR, GBP, JPY, USD};

    // ========================================================================
    // HalfUp Rounding Tests
    // ========================================================================

    #[test]
    fn test_round_half_up_positive() {
        use rust_decimal::Decimal;
        // Create 12.345 USD (3 decimal places)
        let value = Decimal::new(12345, 3);
        let amount = Amount::<USD> {
            value,
            _currency: PhantomData,
        };

        // Round to 2 decimals: 12.345 → 12.35
        let rounded = amount.round(RoundingMode::HalfUp);
        assert_eq!(rounded.to_minor(), 1235);
    }

    #[test]
    fn test_round_half_up_exactly_half() {
        use rust_decimal::Decimal;
        // Create 12.345 with .345 → rounds to .35
        let value = Decimal::new(12345, 3);
        let amount = Amount::<USD> {
            value,
            _currency: PhantomData,
        };

        let rounded = amount.round(RoundingMode::HalfUp);
        assert_eq!(rounded.to_minor(), 1235); // 12.35
    }

    #[test]
    fn test_round_half_up_negative() {
        use rust_decimal::Decimal;
        // Create -12.345
        let value = Decimal::new(-12345, 3);
        let amount = Amount::<USD> {
            value,
            _currency: PhantomData,
        };

        let rounded = amount.round(RoundingMode::HalfUp);
        assert_eq!(rounded.to_minor(), -1235); // -12.35
    }

    // ========================================================================
    // HalfDown Rounding Tests
    // ========================================================================

    #[test]
    fn test_round_half_down_positive() {
        use rust_decimal::Decimal;
        // Create 12.345 - rounds to 12.34 (half down)
        let value = Decimal::new(12345, 3);
        let amount = Amount::<USD> {
            value,
            _currency: PhantomData,
        };

        let rounded = amount.round(RoundingMode::HalfDown);
        assert_eq!(rounded.to_minor(), 1234);
    }

    #[test]
    fn test_round_half_down_negative() {
        use rust_decimal::Decimal;
        // Create -12.345 - rounds to -12.34 (half down)
        let value = Decimal::new(-12345, 3);
        let amount = Amount::<USD> {
            value,
            _currency: PhantomData,
        };

        let rounded = amount.round(RoundingMode::HalfDown);
        assert_eq!(rounded.to_minor(), -1234);
    }

    // ========================================================================
    // HalfEven (Banker's Rounding) Tests
    // ========================================================================

    #[test]
    fn test_round_half_even_to_even() {
        use rust_decimal::Decimal;
        // Create 12.345 - rounds to 12.34 (4 is even)
        let value = Decimal::new(12345, 3);
        let amount = Amount::<USD> {
            value,
            _currency: PhantomData,
        };

        let rounded = amount.round(RoundingMode::HalfEven);
        assert_eq!(rounded.to_minor(), 1234); // 12.34 (even)

        // Create 12.355 - rounds to 12.36 (6 is even)
        let value2 = Decimal::new(12355, 3);
        let amount2 = Amount::<USD> {
            value: value2,
            _currency: PhantomData,
        };

        let rounded2 = amount2.round(RoundingMode::HalfEven);
        assert_eq!(rounded2.to_minor(), 1236); // 12.36 (even)
    }

    #[test]
    fn test_round_half_even_minimizes_bias() {
        use rust_decimal::Decimal;
        // Test that banker's rounding minimizes cumulative error
        let amounts = [
            Decimal::new(10345, 3), // 10.345 → 10.34
            Decimal::new(11345, 3), // 11.345 → 11.34
            Decimal::new(12345, 3), // 12.345 → 12.34
            Decimal::new(13355, 3), // 13.355 → 13.36
        ];

        let total_rounded: i64 = amounts
            .iter()
            .map(|&v| {
                let amt = Amount::<USD> {
                    value: v,
                    _currency: PhantomData,
                };
                amt.round(RoundingMode::HalfEven).to_minor()
            })
            .sum();

        // Total should be: 10.34 + 11.34 + 12.34 + 13.36 = 47.38
        assert_eq!(total_rounded, 4738);
    }

    // ========================================================================
    // Up Rounding Tests
    // ========================================================================

    #[test]
    fn test_round_up_positive() {
        use rust_decimal::Decimal;
        // 12.341 rounds up to 12.35 (away from zero)
        let value = Decimal::new(12341, 3);
        let amount = Amount::<USD> {
            value,
            _currency: PhantomData,
        };

        let rounded = amount.round(RoundingMode::Up);
        assert_eq!(rounded.to_minor(), 1235);
    }

    #[test]
    fn test_round_up_negative() {
        use rust_decimal::Decimal;
        // -12.341 rounds to -12.35 (away from zero)
        let value = Decimal::new(-12341, 3);
        let amount = Amount::<USD> {
            value,
            _currency: PhantomData,
        };

        let rounded = amount.round(RoundingMode::Up);
        assert_eq!(rounded.to_minor(), -1235);
    }

    #[test]
    fn test_round_up_no_fraction() {
        // Exact value doesn't change
        let amount = Amount::<USD>::from_major(12);
        let rounded = amount.round(RoundingMode::Up);
        assert_eq!(rounded.to_major_floor(), 12);
    }

    // ========================================================================
    // Down Rounding Tests
    // ========================================================================

    #[test]
    fn test_round_down_positive() {
        use rust_decimal::Decimal;
        // 12.349 rounds down to 12.34 (towards zero)
        let value = Decimal::new(12349, 3);
        let amount = Amount::<USD> {
            value,
            _currency: PhantomData,
        };

        let rounded = amount.round(RoundingMode::Down);
        assert_eq!(rounded.to_minor(), 1234);
    }

    #[test]
    fn test_round_down_negative() {
        use rust_decimal::Decimal;
        // -12.349 rounds to -12.34 (towards zero)
        let value = Decimal::new(-12349, 3);
        let amount = Amount::<USD> {
            value,
            _currency: PhantomData,
        };

        let rounded = amount.round(RoundingMode::Down);
        assert_eq!(rounded.to_minor(), -1234);
    }

    // ========================================================================
    // Floor Rounding Tests
    // ========================================================================

    #[test]
    fn test_round_floor_positive() {
        use rust_decimal::Decimal;
        // 12.349 floors to 12.34
        let value = Decimal::new(12349, 3);
        let amount = Amount::<USD> {
            value,
            _currency: PhantomData,
        };

        let rounded = amount.round(RoundingMode::Floor);
        assert_eq!(rounded.to_minor(), 1234);
    }

    #[test]
    fn test_round_floor_negative() {
        use rust_decimal::Decimal;
        // -12.341 floors to -12.35 (more negative)
        let value = Decimal::new(-12341, 3);
        let amount = Amount::<USD> {
            value,
            _currency: PhantomData,
        };

        let rounded = amount.round(RoundingMode::Floor);
        assert_eq!(rounded.to_minor(), -1235);
    }

    // ========================================================================
    // Ceiling Rounding Tests
    // ========================================================================

    #[test]
    fn test_round_ceiling_positive() {
        use rust_decimal::Decimal;
        // 12.341 ceilings to 12.35
        let value = Decimal::new(12341, 3);
        let amount = Amount::<USD> {
            value,
            _currency: PhantomData,
        };

        let rounded = amount.round(RoundingMode::Ceiling);
        assert_eq!(rounded.to_minor(), 1235);
    }

    #[test]
    fn test_round_ceiling_negative() {
        use rust_decimal::Decimal;
        // -12.349 ceilings to -12.34 (less negative)
        let value = Decimal::new(-12349, 3);
        let amount = Amount::<USD> {
            value,
            _currency: PhantomData,
        };

        let rounded = amount.round(RoundingMode::Ceiling);
        assert_eq!(rounded.to_minor(), -1234);
    }

    // ========================================================================
    // Currency-Specific Decimal Tests
    // ========================================================================

    #[test]
    fn test_round_usd_respects_2_decimals() {
        use rust_decimal::Decimal;
        // USD has 2 decimals - test with 3 decimal input
        let value = Decimal::new(12341, 3); // 12.341
        let amount = Amount::<USD> {
            value,
            _currency: PhantomData,
        };

        let rounded = amount.round(RoundingMode::HalfUp);
        assert_eq!(rounded.to_minor(), 1234); // Rounds to 12.34
    }

    #[test]
    fn test_round_jpy_respects_0_decimals() {
        // JPY has 0 decimals
        let amount = Amount::<JPY>::from_major(127) / 10; // 12.7
        let rounded = amount.round(RoundingMode::HalfUp);
        assert_eq!(rounded.to_major_floor(), 13); // Rounds to 13
    }

    #[test]
    fn test_round_btc_respects_8_decimals() {
        // BTC has 8 decimals
        use rust_decimal::Decimal;

        // Create 0.123456789 BTC (9 decimals)
        let value = Decimal::new(123456789, 9);
        let amount = Amount::<BTC> {
            value,
            _currency: PhantomData,
        };

        let rounded = amount.round(RoundingMode::HalfUp);
        // Should round to 0.12345679 (8 decimals)
        assert_eq!(rounded.value().to_string(), "0.12345679");
    }

    #[test]
    fn test_round_eur_maintains_precision() {
        // Test that rounding to EUR's 2 decimals works correctly
        let amount = Amount::<EUR>::from_minor(999); // €9.99
        let rounded = amount.round(RoundingMode::HalfEven);
        assert_eq!(rounded.to_minor(), 999); // No change needed
    }

    #[test]
    fn test_round_gbp_edge_cases() {
        use rust_decimal::Decimal;
        // Test GBP with 3 decimals to 2
        let value = Decimal::new(10345, 3); // £10.345
        let amount = Amount::<GBP> {
            value,
            _currency: PhantomData,
        };

        let up = amount.round(RoundingMode::HalfUp);
        assert_eq!(up.to_minor(), 1035); // £10.35

        let down = amount.round(RoundingMode::HalfDown);
        assert_eq!(down.to_minor(), 1034); // £10.34

        let even = amount.round(RoundingMode::HalfEven);
        assert_eq!(even.to_minor(), 1034); // £10.34 (4 is even)
    }

    // ========================================================================
    // Determinism Tests
    // ========================================================================

    #[test]
    fn test_round_deterministic() {
        // Same input should always produce same output
        let amount = Amount::<USD>::from_minor(1234);

        let r1 = amount.round(RoundingMode::HalfEven);
        let r2 = amount.round(RoundingMode::HalfEven);
        let r3 = amount.round(RoundingMode::HalfEven);

        assert_eq!(r1, r2);
        assert_eq!(r2, r3);
    }

    #[test]
    fn test_round_immutable() {
        // Rounding doesn't modify original
        let original = Amount::<USD>::from_minor(1234);
        let _rounded = original.round(RoundingMode::Up);

        assert_eq!(original.to_minor(), 1234); // Unchanged
    }

    // ========================================================================
    // Zero and Edge Cases
    // ========================================================================

    #[test]
    fn test_round_zero() {
        let zero = Amount::<USD>::from_major(0);
        let rounded = zero.round(RoundingMode::HalfUp);
        assert_eq!(rounded.to_major_floor(), 0);
    }

    #[test]
    fn test_round_already_rounded() {
        // Amount already at correct precision
        let amount = Amount::<USD>::from_major(12); // Exactly $12.00
        let rounded = amount.round(RoundingMode::HalfUp);
        assert_eq!(rounded.to_major_floor(), 12);
    }

    #[test]
    fn test_round_all_modes_on_same_value() {
        // Test all rounding modes on the same value
        let amount = Amount::<USD>::from_minor(1255); // 12.55

        // All should round to either 12 or 13
        let modes = [
            RoundingMode::HalfUp,
            RoundingMode::HalfDown,
            RoundingMode::HalfEven,
            RoundingMode::Up,
            RoundingMode::Down,
            RoundingMode::Floor,
            RoundingMode::Ceiling,
        ];

        for mode in modes {
            let rounded = amount.round(mode);
            let major = rounded.to_major_floor();
            assert!((12..=13).contains(&major), "Mode {:?} gave {}", mode, major);
        }
    }
}
