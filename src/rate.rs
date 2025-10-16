//! Exchange rate types for currency conversion.
//!
//! This module provides type-safe exchange rates that enable explicit,
//! auditable currency conversions while preventing implicit conversions.

use crate::Currency;
use std::marker::PhantomData;

#[cfg(all(feature = "use_rust_decimal", not(feature = "use_bigdecimal")))]
use rust_decimal::Decimal;

#[cfg(all(feature = "use_bigdecimal", not(feature = "use_rust_decimal")))]
use bigdecimal::BigDecimal as Decimal;

/// An exchange rate from one currency to another.
///
/// Exchange rates are immutable after construction and use phantom types
/// to ensure type-safe conversions at compile time.
///
/// # Type Parameters
///
/// * `From` - The source currency
/// * `To` - The target currency
///
/// # Examples
///
/// ```
/// use typed_money::{Rate, USD, EUR};
///
/// // Create a rate: 1 USD = 0.85 EUR
/// let rate = Rate::<USD, EUR>::new(0.85);
/// ```
///
/// # Immutability
///
/// Rates are immutable after creation to ensure auditability and prevent
/// accidental modifications that could lead to financial errors.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rate<From: Currency, To: Currency> {
    /// The exchange rate value (always positive)
    rate: Decimal,
    /// Optional UNIX timestamp (seconds) representing when the rate was observed
    ///
    /// Kept optional to avoid forcing callers to provide a timestamp. Using a
    /// primitive preserves `Copy` and avoids allocations.
    metadata_timestamp_unix_secs: Option<u64>,
    /// Optional static source identifier for auditability (e.g., "ECB", "Manual")
    ///
    /// Using `&'static str` preserves `Copy`. Callers can pass string literals
    /// for simple source tagging without allocations.
    metadata_source: Option<&'static str>,
    /// Phantom data for source currency (zero runtime cost)
    _from: PhantomData<From>,
    /// Phantom data for target currency (zero runtime cost)
    _to: PhantomData<To>,
}

impl<From: Currency, To: Currency> Rate<From, To> {
    /// Creates a new exchange rate from a floating-point value.
    ///
    /// # Panics
    ///
    /// Panics if the rate is zero, negative, NaN, or infinite.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_money::{Rate, USD, EUR};
    ///
    /// let rate = Rate::<USD, EUR>::new(0.85);  // 1 USD = 0.85 EUR
    /// ```
    ///
    /// # Panics Examples
    ///
    /// ```should_panic
    /// use typed_money::{Rate, USD, EUR};
    ///
    /// let rate = Rate::<USD, EUR>::new(0.0);  // Panics: rate must be positive
    /// ```
    pub fn new(rate: f64) -> Self {
        assert!(rate.is_finite(), "Exchange rate must be a finite number");
        assert!(rate > 0.0, "Exchange rate must be positive and non-zero");

        let decimal_rate = Decimal::try_from(rate).expect("Failed to convert rate to Decimal");

        Self {
            rate: decimal_rate,
            metadata_timestamp_unix_secs: None,
            metadata_source: None,
            _from: PhantomData,
            _to: PhantomData,
        }
    }

    /// Creates a new exchange rate from a `Decimal` value.
    ///
    /// This is useful when you already have a precise decimal rate.
    ///
    /// # Panics
    ///
    /// Panics if the rate is zero or negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_money::{Rate, USD, EUR};
    /// use rust_decimal::Decimal;
    ///
    /// let decimal_rate = Decimal::new(85, 2);  // 0.85
    /// let rate = Rate::<USD, EUR>::from_decimal(decimal_rate);
    /// ```
    pub fn from_decimal(rate: Decimal) -> Self {
        assert!(
            rate > Decimal::ZERO,
            "Exchange rate must be positive and non-zero"
        );

        Self {
            rate,
            metadata_timestamp_unix_secs: None,
            metadata_source: None,
            _from: PhantomData,
            _to: PhantomData,
        }
    }

    /// Returns the exchange rate value.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_money::{Rate, USD, EUR};
    ///
    /// let rate = Rate::<USD, EUR>::new(0.85);
    /// println!("Rate: {}", rate.value());
    /// ```
    #[inline]
    pub const fn value(&self) -> &Decimal {
        &self.rate
    }

    /// Returns the optional UNIX timestamp (seconds) metadata.
    #[inline]
    pub const fn timestamp_unix_secs(&self) -> Option<u64> {
        self.metadata_timestamp_unix_secs
    }

    /// Returns the optional static source identifier metadata.
    #[inline]
    pub const fn source(&self) -> Option<&'static str> {
        self.metadata_source
    }

    /// Returns a new `Rate` with the given UNIX timestamp (seconds) metadata set.
    ///
    /// Existing metadata values not provided by this method are preserved.
    #[inline]
    pub const fn with_timestamp_unix_secs(mut self, timestamp_unix_secs: u64) -> Self {
        self.metadata_timestamp_unix_secs = Some(timestamp_unix_secs);
        self
    }

    /// Returns a new `Rate` with the given static source identifier set.
    ///
    /// Existing metadata values not provided by this method are preserved.
    #[inline]
    pub const fn with_source(mut self, source: &'static str) -> Self {
        self.metadata_source = Some(source);
        self
    }

    /// Convenience method to set both timestamp and source metadata at once.
    #[inline]
    pub const fn with_metadata(self, timestamp_unix_secs: u64, source: &'static str) -> Self {
        self.with_timestamp_unix_secs(timestamp_unix_secs).with_source(source)
    }

    /// Returns the inverse rate (To -> From).
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_money::{Rate, USD, EUR};
    ///
    /// let usd_to_eur = Rate::<USD, EUR>::new(0.85);
    /// let eur_to_usd = usd_to_eur.inverse();
    ///
    /// // Inverse of 0.85 is approximately 1.176
    /// ```
    pub fn inverse(&self) -> Rate<To, From> {
        Rate {
            rate: Decimal::ONE / self.rate,
            metadata_timestamp_unix_secs: self.metadata_timestamp_unix_secs,
            metadata_source: self.metadata_source,
            _from: PhantomData,
            _to: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EUR, GBP, USD};

    #[test]
    fn test_rate_creation() {
        let rate = Rate::<USD, EUR>::new(0.85);
        assert!(rate.value() > &Decimal::ZERO);
    }

    #[test]
    fn test_rate_from_decimal() {
        let decimal_rate = Decimal::new(85, 2); // 0.85
        let rate = Rate::<USD, EUR>::from_decimal(decimal_rate);
        assert_eq!(rate.value(), &decimal_rate);
    }

    #[test]
    #[should_panic(expected = "Exchange rate must be positive and non-zero")]
    fn test_rate_zero_panics() {
        let _ = Rate::<USD, EUR>::new(0.0);
    }

    #[test]
    #[should_panic(expected = "Exchange rate must be positive and non-zero")]
    fn test_rate_negative_panics() {
        let _ = Rate::<USD, EUR>::new(-1.5);
    }

    #[test]
    #[should_panic(expected = "Exchange rate must be a finite number")]
    fn test_rate_nan_panics() {
        let _ = Rate::<USD, EUR>::new(f64::NAN);
    }

    #[test]
    #[should_panic(expected = "Exchange rate must be a finite number")]
    fn test_rate_infinity_panics() {
        let _ = Rate::<USD, EUR>::new(f64::INFINITY);
    }

    #[test]
    fn test_rate_inverse() {
        let usd_to_eur = Rate::<USD, EUR>::new(0.85);
        let eur_to_usd = usd_to_eur.inverse();

        // Inverse of 0.85 should be approximately 1.176
        let inverse_value = eur_to_usd.value().to_string();
        assert!(inverse_value.starts_with("1.17"));
    }

    #[test]
    fn test_rate_double_inverse() {
        let original = Rate::<USD, EUR>::new(0.85);
        let inverse = original.inverse();
        let back = inverse.inverse();

        // Double inverse should get back to original (within precision)
        let diff = (original.value() - back.value()).abs();
        assert!(diff < Decimal::new(1, 10)); // Less than 0.0000000001
    }

    #[test]
    fn test_rate_immutability() {
        let rate = Rate::<USD, EUR>::new(0.85);
        let rate_copy = rate;

        // Both should have the same value (proving immutability)
        assert_eq!(rate.value(), rate_copy.value());
    }

    #[test]
    fn test_rate_type_safety() {
        let _usd_eur = Rate::<USD, EUR>::new(0.85);
        let _eur_gbp = Rate::<EUR, GBP>::new(0.88);

        // These are different types at compile time
        // let _ = usd_eur == eur_gbp;  // Won't compile: type mismatch!
    }

    #[test]
    fn test_rate_clone() {
        let rate1 = Rate::<USD, EUR>::new(0.85);
        #[allow(clippy::clone_on_copy)]
        let rate2 = rate1.clone();

        assert_eq!(rate1.value(), rate2.value());
    }

    #[test]
    fn test_rate_copy() {
        let rate1 = Rate::<USD, EUR>::new(0.85);
        let rate2 = rate1; // Copy

        // Both should still be usable (proving Copy works)
        assert_eq!(rate1.value(), rate2.value());
    }

    #[test]
    fn test_rate_metadata_defaults_and_setters() {
        let rate = Rate::<USD, EUR>::new(0.85);
        assert_eq!(rate.timestamp_unix_secs(), None);
        assert_eq!(rate.source(), None);

        let rate = rate.with_timestamp_unix_secs(1_700_000_000).with_source("Manual");
        assert_eq!(rate.timestamp_unix_secs(), Some(1_700_000_000));
        assert_eq!(rate.source(), Some("Manual"));
    }

    #[test]
    fn test_rate_inverse_preserves_metadata() {
        let rate = Rate::<USD, EUR>::new(0.85).with_metadata(1_700_000_000, "ECB");
        let inverse = rate.inverse();
        assert_eq!(inverse.timestamp_unix_secs(), Some(1_700_000_000));
        assert_eq!(inverse.source(), Some("ECB"));
    }
}
