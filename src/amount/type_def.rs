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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
}
