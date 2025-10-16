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
    #[cfg(all(feature = "use_rust_decimal", not(feature = "use_bigdecimal")))]
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

    #[cfg(all(feature = "use_bigdecimal", not(feature = "use_rust_decimal")))]
    pub fn from_minor(amount: i64) -> Self {
        use bigdecimal::BigInt;
        use bigdecimal::ToPrimitive;

        let value = if C::DECIMALS == 0 {
            Decimal::from(amount)
        } else {
            Decimal::new(BigInt::from(amount), C::DECIMALS.into())
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

    // Determinism tests
    #[test]
    fn test_decimal_creation_determinism() {
        // Verify that creating decimals produces consistent results
        let amount1 = Amount::<USD>::from_major(100);
        let amount2 = Amount::<USD>::from_major(100);
        assert_eq!(amount1.value(), amount2.value());

        let amount3 = Amount::<USD>::from_minor(12345);
        let amount4 = Amount::<USD>::from_minor(12345);
        assert_eq!(amount3.value(), amount4.value());
    }

    #[test]
    fn test_different_decimal_places_determinism() {
        use crate::{BTC, JPY};
        // Test currencies with different decimal places
        let usd = Amount::<USD>::from_minor(12345); // 2 decimals
        let jpy = Amount::<JPY>::from_minor(12345); // 0 decimals
        let btc = Amount::<BTC>::from_minor(12345); // 8 decimals

        assert_eq!(usd.value().to_string(), "123.45");
        assert_eq!(jpy.value().to_string(), "12345");
        assert_eq!(btc.value().to_string(), "0.00012345");
    }

    #[test]
    fn test_zero_determinism() {
        // Verify zero is represented consistently
        let zero1 = Amount::<USD>::from_major(0);
        let zero2 = Amount::<USD>::from_minor(0);
        assert_eq!(zero1, zero2);
        assert_eq!(zero1.value().to_string(), "0");
    }
}
