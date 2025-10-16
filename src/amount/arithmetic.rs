//! Arithmetic operations for Amount.
//!
//! All operations are pure functions that create new values (immutable).
//! Cross-currency operations are prevented at compile time by the type system.

use super::type_def::Amount;
use crate::Currency;
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Sub};

#[cfg(all(feature = "use_rust_decimal", not(feature = "use_bigdecimal")))]
use rust_decimal::Decimal;

#[cfg(all(feature = "use_bigdecimal", not(feature = "use_rust_decimal")))]
use bigdecimal::BigDecimal as Decimal;

// ============================================================================
// Addition
// ============================================================================

/// Add two amounts of the same currency.
///
/// # Type Safety
///
/// The type system prevents adding amounts of different currencies at compile time.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, USD};
///
/// let a = Amount::<USD>::from_major(100);
/// let b = Amount::<USD>::from_major(50);
/// let total = a + b;
///
/// assert_eq!(total.to_major_floor(), 150);
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
impl<C: Currency> Add for Amount<C> {
    type Output = Self;

    #[inline]
    fn add(self, other: Self) -> Self {
        Self {
            value: self.value + other.value,
            _currency: PhantomData,
        }
    }
}

// ============================================================================
// Subtraction
// ============================================================================

/// Subtract one amount from another (same currency).
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, USD};
///
/// let total = Amount::<USD>::from_major(100);
/// let discount = Amount::<USD>::from_major(15);
/// let final_price = total - discount;
///
/// assert_eq!(final_price.to_major_floor(), 85);
/// ```
///
/// # Compile-Time Safety
///
/// ```compile_fail
/// use typed_money::{Amount, USD, EUR};
///
/// let usd = Amount::<USD>::from_major(100);
/// let eur = Amount::<EUR>::from_major(15);
///
/// // This won't compile!
/// let invalid = usd - eur;  // Error: type mismatch
/// ```
impl<C: Currency> Sub for Amount<C> {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {
            value: self.value - other.value,
            _currency: PhantomData,
        }
    }
}

// ============================================================================
// Scalar Multiplication
// ============================================================================

/// Multiply an amount by a scalar integer.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, USD};
///
/// let price = Amount::<USD>::from_major(50);
/// let total = price * 3;
///
/// assert_eq!(total.to_major_floor(), 150);
/// ```
impl<C: Currency> Mul<i64> for Amount<C> {
    type Output = Self;

    #[inline]
    fn mul(self, scalar: i64) -> Self {
        Self {
            value: self.value * Decimal::from(scalar),
            _currency: PhantomData,
        }
    }
}

/// Multiply an amount by a scalar (commutative).
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, USD};
///
/// let price = Amount::<USD>::from_major(50);
/// let total = 3 * price;  // Commutative: 3 * price = price * 3
///
/// assert_eq!(total.to_major_floor(), 150);
/// ```
impl<C: Currency> Mul<Amount<C>> for i64 {
    type Output = Amount<C>;

    #[inline]
    fn mul(self, amount: Amount<C>) -> Amount<C> {
        amount * self
    }
}

// ============================================================================
// Scalar Division
// ============================================================================

/// Divide an amount by a scalar integer.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, USD};
///
/// let total = Amount::<USD>::from_major(100);
/// let per_person = total / 4;
///
/// assert_eq!(per_person.to_major_floor(), 25);
/// ```
///
/// # Panics
///
/// Panics if dividing by zero.
impl<C: Currency> Div<i64> for Amount<C> {
    type Output = Self;

    #[inline]
    fn div(self, scalar: i64) -> Self {
        assert!(scalar != 0, "Cannot divide amount by zero");

        Self {
            value: self.value / Decimal::from(scalar),
            _currency: PhantomData,
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EUR, USD};

    // Addition tests
    #[test]
    fn test_add_same_currency() {
        let a = Amount::<USD>::from_major(100);
        let b = Amount::<USD>::from_major(50);
        let result = a + b;

        assert_eq!(result.to_major_floor(), 150);
    }

    #[test]
    fn test_add_with_decimals() {
        let a = Amount::<USD>::from_minor(12345); // $123.45
        let b = Amount::<USD>::from_minor(6789); // $67.89
        let result = a + b;

        assert_eq!(result.to_minor(), 19134); // $191.34
    }

    #[test]
    fn test_add_zero() {
        let a = Amount::<USD>::from_major(100);
        let zero = Amount::<USD>::from_major(0);
        let result = a + zero;

        assert_eq!(result, a);
    }

    // Subtraction tests
    #[test]
    fn test_sub_same_currency() {
        let a = Amount::<USD>::from_major(100);
        let b = Amount::<USD>::from_major(30);
        let result = a - b;

        assert_eq!(result.to_major_floor(), 70);
    }

    #[test]
    fn test_sub_with_decimals() {
        let a = Amount::<USD>::from_minor(12345); // $123.45
        let b = Amount::<USD>::from_minor(6789); // $67.89
        let result = a - b;

        assert_eq!(result.to_minor(), 5556); // $55.56
    }

    #[test]
    fn test_sub_to_negative() {
        let a = Amount::<USD>::from_major(50);
        let b = Amount::<USD>::from_major(100);
        let result = a - b;

        assert_eq!(result.to_major_floor(), -50);
    }

    #[test]
    fn test_sub_zero() {
        let a = Amount::<USD>::from_major(100);
        let zero = Amount::<USD>::from_major(0);
        let result = a - zero;

        assert_eq!(result, a);
    }

    // Multiplication tests
    #[test]
    fn test_mul_by_scalar() {
        let price = Amount::<USD>::from_major(50);
        let total = price * 3;

        assert_eq!(total.to_major_floor(), 150);
    }

    #[test]
    fn test_mul_commutative() {
        let price = Amount::<USD>::from_major(50);
        let result1 = price * 3;
        let result2 = 3 * price;

        assert_eq!(result1, result2);
    }

    #[test]
    fn test_mul_by_zero() {
        let price = Amount::<USD>::from_major(100);
        #[allow(clippy::erasing_op)]
        let result = price * 0;

        assert_eq!(result.to_major_floor(), 0);
    }

    #[test]
    fn test_mul_by_one() {
        let price = Amount::<USD>::from_major(100);
        let result = price * 1;

        assert_eq!(result, price);
    }

    #[test]
    fn test_mul_with_decimals() {
        let price = Amount::<USD>::from_minor(1250); // $12.50
        let result = price * 4;

        assert_eq!(result.to_minor(), 5000); // $50.00
    }

    #[test]
    fn test_mul_negative() {
        let price = Amount::<USD>::from_major(50);
        let result = price * -2;

        assert_eq!(result.to_major_floor(), -100);
    }

    // Division tests
    #[test]
    fn test_div_by_scalar() {
        let total = Amount::<USD>::from_major(100);
        let per_person = total / 4;

        assert_eq!(per_person.to_major_floor(), 25);
    }

    #[test]
    fn test_div_with_remainder() {
        let total = Amount::<USD>::from_major(100);
        let result = total / 3;

        // 100 / 3 = 33.333...
        assert_eq!(result.to_major_floor(), 33);
        assert_eq!(result.to_major_ceiling(), 34);
    }

    #[test]
    fn test_div_by_one() {
        let amount = Amount::<USD>::from_major(100);
        let result = amount / 1;

        assert_eq!(result, amount);
    }

    #[test]
    fn test_div_with_decimals() {
        let total = Amount::<USD>::from_minor(10000); // $100.00
        let result = total / 4;

        assert_eq!(result.to_minor(), 2500); // $25.00
    }

    #[test]
    #[should_panic(expected = "Cannot divide amount by zero")]
    fn test_div_by_zero_panics() {
        let amount = Amount::<USD>::from_major(100);
        let _ = amount / 0; // Should panic
    }

    // Combined operations
    #[test]
    fn test_combined_operations() {
        let price = Amount::<USD>::from_major(100);
        let discount = Amount::<USD>::from_major(10);
        let quantity = 3;

        // (price - discount) * quantity
        let total = (price - discount) * quantity;

        assert_eq!(total.to_major_floor(), 270); // ($100 - $10) * 3 = $270
    }

    #[test]
    fn test_complex_calculation() {
        // Calculate: (base + fee) * qty / people
        let base = Amount::<USD>::from_minor(9999); // $99.99
        let fee = Amount::<USD>::from_minor(500); // $5.00
        let total = (base + fee) * 10; // $1049.90
        let per_person = total / 5; // $209.98

        assert_eq!(per_person.to_minor(), 20998);
    }

    // Type safety tests (these should fail to compile if uncommented)
    #[test]
    fn test_type_safety_enforced() {
        let usd = Amount::<USD>::from_major(100);
        let _eur = Amount::<EUR>::from_major(85);

        // The following would not compile (type mismatch):
        // let _ = usd + eur;  // Error!
        // let _ = usd - eur;  // Error!

        // Only same-currency operations compile
        let usd2 = Amount::<USD>::from_major(50);
        let _ = usd + usd2; // This works!
    }

    // Immutability test
    #[test]
    fn test_immutability() {
        let original = Amount::<USD>::from_major(100);
        let doubled = original * 2;

        // Original is unchanged (functional programming principle)
        assert_eq!(original.to_major_floor(), 100);
        assert_eq!(doubled.to_major_floor(), 200);
    }
}
