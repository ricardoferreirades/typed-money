//! Constructor methods for Amount.

use super::type_def::Amount;
use crate::Currency;
use std::marker::PhantomData;

#[cfg(all(feature = "use_rust_decimal", not(feature = "use_bigdecimal")))]
use rust_decimal::Decimal;

#[cfg(all(feature = "use_bigdecimal", not(feature = "use_rust_decimal")))]
use bigdecimal::BigDecimal as Decimal;

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::USD;

    #[test]
    fn test_from_major() {
        let amount = Amount::<USD>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_from_minor() {
        let amount = Amount::<USD>::from_minor(12345);
        assert_eq!(amount.to_major_floor(), 123);
        assert_eq!(amount.to_minor(), 12345);
    }

    #[test]
    fn test_from_minor_zero_decimals() {
        use crate::JPY;
        let amount = Amount::<JPY>::from_minor(1000);
        assert_eq!(amount.to_major_floor(), 1000);
        assert_eq!(amount.to_minor(), 1000);
    }
}
