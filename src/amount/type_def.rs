//! Amount type definition.

use crate::Currency;
use std::marker::PhantomData;

#[cfg(all(feature = "use_rust_decimal", not(feature = "use_bigdecimal")))]
use rust_decimal::Decimal;

#[cfg(all(feature = "use_bigdecimal", not(feature = "use_rust_decimal")))]
use bigdecimal::BigDecimal as Decimal;

/// A monetary amount in a specific currency.
///
/// This type uses phantom types to track currency at compile time, preventing
/// accidental mixing of different currencies. All operations are immutable and
/// follow functional programming principles.
///
/// # Type Parameters
///
/// * `C` - The currency type, which must implement the `Currency` trait
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, USD};
///
/// // Create from major units (dollars)
/// let price = Amount::<USD>::from_major(100);  // $100.00
///
/// // Create from minor units (cents)
/// let tax = Amount::<USD>::from_minor(550);    // $5.50
///
/// println!("{}", price);  // Displays: $100.00 USD
/// println!("{}", tax);    // Displays: $5.50 USD
/// ```
///
/// # Compile-Time Safety
///
/// ```compile_fail
/// use typed_money::{Amount, USD, EUR};
///
/// let usd = Amount::<USD>::from_major(100);
/// let eur = Amount::<EUR>::from_major(85);
///
/// // This won't compile!
/// let invalid = usd + eur;  // Error: type mismatch
/// ```
#[cfg(all(feature = "use_rust_decimal", not(feature = "use_bigdecimal")))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Amount<C: Currency> {
    /// Internal value stored as a Decimal for precision
    pub(super) value: Decimal,
    /// Phantom data to track currency type at compile time (zero runtime cost)
    pub(super) _currency: PhantomData<C>,
}

#[cfg(all(feature = "use_bigdecimal", not(feature = "use_rust_decimal")))]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Amount<C: Currency> {
    /// Internal value stored as a Decimal for precision
    pub(super) value: Decimal,
    /// Phantom data to track currency type at compile time (zero runtime cost)
    pub(super) _currency: PhantomData<C>,
}

impl<C: Currency> Amount<C> {
    /// Returns the raw `Decimal` value.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_money::{Amount, USD};
    /// use rust_decimal::Decimal;
    ///
    /// let amount = Amount::<USD>::from_major(100);
    /// assert_eq!(amount.value(), &Decimal::from(100));
    /// ```
    #[inline]
    pub const fn value(&self) -> &Decimal {
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::USD;
    use rust_decimal::Decimal;

    #[test]
    fn test_copy_clone() {
        let amount1 = Amount::<USD>::from_major(100);
        let amount2 = amount1; // Copy

        // Verify Copy trait works (amount1 still usable after copy)
        assert_eq!(amount1, amount2);

        // Test Clone trait explicitly
        #[allow(clippy::clone_on_copy)]
        let amount3 = amount1.clone(); // Explicitly test Clone trait
        assert_eq!(amount1, amount3);
    }

    #[test]
    fn test_phantom_data_zero_cost() {
        use std::mem;

        // Amount<C> should be the same size as Decimal (PhantomData is zero-sized)
        assert_eq!(mem::size_of::<Amount<USD>>(), mem::size_of::<Decimal>());
    }

    #[test]
    fn test_value_accessor() {
        let amount = Amount::<USD>::from_major(100);
        assert_eq!(*amount.value(), Decimal::from(100));
    }

    // ========================================================================
    // Comparison Tests (PartialEq, Eq, PartialOrd, Ord)
    // ========================================================================

    #[test]
    fn test_equality_same_values() {
        let a = Amount::<USD>::from_major(100);
        let b = Amount::<USD>::from_major(100);

        assert_eq!(a, b);
        assert!(a == b);
    }

    #[test]
    fn test_equality_different_values() {
        let a = Amount::<USD>::from_major(100);
        let b = Amount::<USD>::from_major(50);

        assert_ne!(a, b);
        assert!(a != b);
    }

    #[test]
    fn test_less_than() {
        let small = Amount::<USD>::from_major(50);
        let large = Amount::<USD>::from_major(100);

        assert!(small < large);
        assert!(large >= small);
    }

    #[test]
    fn test_greater_than() {
        let small = Amount::<USD>::from_major(50);
        let large = Amount::<USD>::from_major(100);

        assert!(large > small);
        assert!(small <= large);
    }

    #[test]
    fn test_sorting() {
        let mut amounts = [
            Amount::<USD>::from_major(100),
            Amount::<USD>::from_major(25),
            Amount::<USD>::from_major(75),
            Amount::<USD>::from_major(50),
        ];

        amounts.sort();

        assert_eq!(amounts[0].to_major_floor(), 25);
        assert_eq!(amounts[1].to_major_floor(), 50);
        assert_eq!(amounts[2].to_major_floor(), 75);
        assert_eq!(amounts[3].to_major_floor(), 100);
    }

    #[test]
    fn test_min_max() {
        use std::cmp::{max, min};

        let a = Amount::<USD>::from_major(100);
        let b = Amount::<USD>::from_major(50);

        assert_eq!(min(a, b), b);
        assert_eq!(max(a, b), a);
    }

    #[test]
    fn test_ordering_with_zero() {
        let zero = Amount::<USD>::from_major(0);
        let positive = Amount::<USD>::from_major(1);
        let negative = Amount::<USD>::from_major(-1);

        assert!(negative < zero);
        assert!(zero < positive);
        assert!(positive > zero);
    }

    #[test]
    fn test_ordering_negative_values() {
        let neg_large = Amount::<USD>::from_major(-100);
        let neg_small = Amount::<USD>::from_major(-50);

        assert!(neg_large < neg_small);
        assert!(neg_small > neg_large);
    }

    // ========================================================================
    // Determinism Tests
    // ========================================================================

    #[test]
    fn test_decimal_precision_no_float() {
        // Verify we never lose precision like floats do
        // This classic float problem should not occur: 0.1 + 0.2 != 0.3 in floating point
        let ten_cents = Amount::<USD>::from_minor(10);
        let twenty_cents = Amount::<USD>::from_minor(20);

        // Verify the underlying decimals are precise
        assert_eq!(ten_cents.value().to_string(), "0.10");
        assert_eq!(twenty_cents.value().to_string(), "0.20");

        let thirty_cents = Amount::<USD>::from_minor(30);
        assert_eq!(thirty_cents.value().to_string(), "0.30");
    }
}
