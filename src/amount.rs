//! Amount type for type-safe monetary values.
//!
//! This module provides the `Amount<C>` type which represents a monetary amount
//! in a specific currency. The currency is tracked at compile time using phantom types,
//! enabling zero-cost type safety.

use crate::Currency;
use rust_decimal::Decimal;
use std::fmt;
use std::marker::PhantomData;

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
    value: Decimal,
    /// Phantom data to track currency type at compile time (zero runtime cost)
    _currency: PhantomData<C>,
}

impl<C: Currency> Amount<C> {
    /// Creates a new `Amount` from a raw `Decimal` value.
    ///
    /// This is a low-level constructor. Consider using `from_major` or `from_minor` instead.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_money::{Amount, USD};
    /// use rust_decimal::Decimal;
    ///
    /// let amount = Amount::<USD>::new(Decimal::new(10050, 2));
    /// ```
    #[inline]
    pub const fn new(value: Decimal) -> Self {
        Self {
            value,
            _currency: PhantomData,
        }
    }

    /// Creates an `Amount` from major currency units (e.g., dollars, euros).
    ///
    /// This is the most common way to create monetary amounts.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_money::{Amount, USD, JPY};
    ///
    /// let usd = Amount::<USD>::from_major(100);  // $100.00
    /// let jpy = Amount::<JPY>::from_major(1000); // ¥1000 (no decimals)
    /// ```
    pub fn from_major(amount: i64) -> Self {
        Self {
            value: Decimal::from(amount),
            _currency: PhantomData,
        }
    }

    /// Creates an `Amount` from minor currency units (e.g., cents, pence).
    ///
    /// The minor units are automatically converted to the proper decimal representation
    /// based on the currency's `DECIMALS` constant.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_money::{Amount, USD, JPY};
    ///
    /// let usd = Amount::<USD>::from_minor(12345);  // $123.45 (from cents)
    /// let jpy = Amount::<JPY>::from_minor(1000);   // ¥1000 (JPY has no minor units)
    /// ```
    pub fn from_minor(amount: i64) -> Self {
        let value = if C::DECIMALS == 0 {
            Decimal::from(amount)
        } else {
            Decimal::new(amount, C::DECIMALS.into())
        };

        Self {
            value,
            _currency: PhantomData,
        }
    }

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

    /// Returns the amount in major units as an integer (truncating decimals).
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_money::{Amount, USD};
    ///
    /// let amount = Amount::<USD>::from_minor(12345);  // $123.45
    /// assert_eq!(amount.to_major(), 123);
    /// ```
    pub fn to_major(&self) -> i64 {
        self.value.trunc().to_string().parse().unwrap_or(0)
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

// ============================================================================
// Display Implementation
// ============================================================================

impl<C: Currency> fmt::Display for Amount<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format: {symbol}{amount} {code}
        // e.g., "$100.00 USD" or "€85.50 EUR"
        let formatted_value = if C::DECIMALS == 0 {
            format!("{}", self.value.trunc())
        } else {
            format!("{:.prec$}", self.value, prec = C::DECIMALS as usize)
        };

        write!(f, "{}{} {}", C::SYMBOL, formatted_value, C::CODE)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BTC, EUR, JPY, USD};

    #[test]
    fn test_from_major() {
        let amount = Amount::<USD>::from_major(100);
        assert_eq!(amount.to_major(), 100);
    }

    #[test]
    fn test_from_minor() {
        let amount = Amount::<USD>::from_minor(12345);
        assert_eq!(amount.to_major(), 123);
        assert_eq!(amount.to_minor(), 12345);
    }

    #[test]
    fn test_from_minor_zero_decimals() {
        let amount = Amount::<JPY>::from_minor(1000);
        assert_eq!(amount.to_major(), 1000);
        assert_eq!(amount.to_minor(), 1000);
    }

    #[test]
    fn test_display_usd() {
        let amount = Amount::<USD>::from_major(100);
        assert_eq!(format!("{}", amount), "$100.00 USD");
    }

    #[test]
    fn test_display_eur() {
        let amount = Amount::<EUR>::from_minor(12345);
        assert_eq!(format!("{}", amount), "€123.45 EUR");
    }

    #[test]
    fn test_display_jpy() {
        let amount = Amount::<JPY>::from_major(1000);
        assert_eq!(format!("{}", amount), "¥1000 JPY");
    }

    #[test]
    fn test_display_btc() {
        let amount = Amount::<BTC>::from_major(1);
        assert_eq!(format!("{}", amount), "₿1.00000000 BTC");
    }

    #[test]
    fn test_copy_clone() {
        let amount1 = Amount::<USD>::from_major(100);
        let amount2 = amount1; // Copy
        let amount3 = amount1.clone(); // Clone

        // All should be equal
        assert_eq!(amount1, amount2);
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
