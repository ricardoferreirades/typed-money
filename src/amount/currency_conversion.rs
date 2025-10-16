//! Currency conversion methods for Amount.
//!
//! Provides explicit currency conversion using exchange rates.

use super::type_def::Amount;
use crate::{Currency, Rate};
use std::marker::PhantomData;

#[cfg(feature = "conversion_tracking")]
use crate::conversion_tracking::{ConversionEvent, ConversionTracker};

impl<C: Currency> Amount<C> {
    /// Converts this amount to another currency using an explicit exchange rate.
    ///
    /// This is the only way to convert between currencies. Implicit conversions
    /// are not allowed, ensuring all conversions are explicit and auditable.
    ///
    /// # Type Safety
    ///
    /// The type system ensures that the rate matches the currencies being converted.
    ///
    /// # Examples
    ///
    /// ```
    /// use typed_money::{Amount, Rate, USD, EUR};
    ///
    /// let usd = Amount::<USD>::from_major(100);
    /// let rate = Rate::<USD, EUR>::new(0.85);
    /// let eur = usd.convert(&rate);
    ///
    /// assert_eq!(eur.to_major_floor(), 85);
    /// ```
    ///
    /// # Compile-Time Safety
    ///
    /// Attempting to use a rate with mismatched currencies won't compile:
    ///
    /// ```compile_fail
    /// use typed_money::{Amount, Rate, USD, EUR, GBP};
    ///
    /// let usd = Amount::<USD>::from_major(100);
    /// let wrong_rate = Rate::<EUR, GBP>::new(0.88);
    ///
    /// // This won't compile - type mismatch!
    /// let invalid = usd.convert(&wrong_rate);
    /// ```
    ///
    /// Implicit conversions between different currencies are prevented:
    ///
    /// ```compile_fail
    /// use typed_money::{Amount, USD, EUR};
    ///
    /// let usd = Amount::<USD>::from_major(100);
    /// let eur = Amount::<EUR>::from_major(85);
    ///
    /// // This won't compile - can't add different currencies!
    /// let invalid = usd + eur;
    /// ```
    pub fn convert<To: Currency>(&self, rate: &Rate<C, To>) -> Amount<To> {
        Amount {
            value: self.value * rate.value(),
            _currency: PhantomData,
        }
    }

    /// Converts this amount to another currency using an explicit exchange rate,
    /// with optional conversion tracking.
    ///
    /// This method is only available when the `conversion_tracking` feature is enabled.
    /// It performs the same conversion as `convert()` but also calls the provided
    /// tracker to log/record the conversion event.
    ///
    /// # Type Safety
    ///
    /// The type system ensures that the rate matches the currencies being converted.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(feature = "conversion_tracking")]
    /// # {
    /// use typed_money::{Amount, Rate, USD, EUR};
    /// use typed_money::conversion_tracking::NoOpTracker;
    ///
    /// let usd = Amount::<USD>::from_major(100);
    /// let rate = Rate::<USD, EUR>::new(0.85);
    /// let tracker = NoOpTracker;
    /// let eur = usd.convert_with_tracking(&rate, &tracker);
    ///
    /// assert_eq!(eur.to_major_floor(), 85);
    /// # }
    /// ```
    #[cfg(feature = "conversion_tracking")]
    pub fn convert_with_tracking<To: Currency, T: ConversionTracker>(
        &self,
        rate: &Rate<C, To>,
        tracker: &T,
    ) -> Amount<To> {
        let result = Amount {
            value: self.value * rate.value(),
            _currency: PhantomData,
        };

        // Create and track the conversion event
        let event = ConversionEvent::<C, To>::new(
            self.value,
            result.value,
            *rate.value(),
            rate.timestamp_unix_secs(),
            rate.source(),
        );

        tracker.track(&event);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BTC, EUR, GBP, JPY, USD};

    #[test]
    fn test_convert_usd_to_eur() {
        let usd = Amount::<USD>::from_major(100);
        let rate = Rate::<USD, EUR>::new(0.85);
        let eur = usd.convert(&rate);

        // 100 * 0.85 = 85.00 (but floating point conversion may have precision issues)
        assert!(eur.to_major_floor() >= 84 && eur.to_major_floor() <= 85);
        assert_eq!(eur.to_minor(), 8500); // More precise check
    }

    #[test]
    fn test_convert_with_decimals() {
        let usd = Amount::<USD>::from_minor(12345); // $123.45
        let rate = Rate::<USD, EUR>::new(0.85);
        let eur = usd.convert(&rate);

        // 123.45 * 0.85 = 104.9325
        assert_eq!(eur.to_minor(), 10493); // €104.93
    }

    #[test]
    fn test_convert_chain() {
        // Convert USD -> EUR -> GBP
        let usd = Amount::<USD>::from_major(100);
        let usd_eur_rate = Rate::<USD, EUR>::new(0.85);
        let eur_gbp_rate = Rate::<EUR, GBP>::new(0.88);

        let eur = usd.convert(&usd_eur_rate);
        let gbp = eur.convert(&eur_gbp_rate);

        // 100 * 0.85 * 0.88 = 74.8
        assert_eq!(gbp.to_major_floor(), 74);
    }

    #[test]
    fn test_convert_using_inverse_rate() {
        let usd = Amount::<USD>::from_major(100);
        let usd_eur_rate = Rate::<USD, EUR>::new(0.85);

        // Convert USD -> EUR
        let eur = usd.convert(&usd_eur_rate);

        // Convert back EUR -> USD using inverse rate
        let eur_usd_rate = usd_eur_rate.inverse();
        let usd_back = eur.convert(&eur_usd_rate);

        // Should get approximately back to 100 (within precision)
        assert_eq!(usd_back.to_major_floor(), 100);
    }

    #[test]
    fn test_convert_zero_amount() {
        let zero = Amount::<USD>::from_major(0);
        let rate = Rate::<USD, EUR>::new(0.85);
        let eur_zero = zero.convert(&rate);

        assert_eq!(eur_zero.to_major_floor(), 0);
    }

    #[test]
    fn test_convert_preserves_original() {
        // Verify immutability - original amount is unchanged
        let original = Amount::<USD>::from_major(100);
        let rate = Rate::<USD, EUR>::new(0.85);
        let _converted = original.convert(&rate);

        // Original should be unchanged
        assert_eq!(original.to_major_floor(), 100);
    }

    #[test]
    fn test_convert_jpy_to_usd() {
        // Test with zero-decimal currency
        let jpy = Amount::<JPY>::from_major(10000);
        let rate = Rate::<JPY, USD>::new(0.0067); // ~150 JPY per USD

        let usd = jpy.convert(&rate);
        assert_eq!(usd.to_major_floor(), 67);
    }

    #[test]
    fn test_convert_btc_to_usd() {
        // Test with high-precision currency
        let btc = Amount::<BTC>::from_minor(10000000); // 0.1 BTC
        let rate = Rate::<BTC, USD>::new(45000.0); // $45,000 per BTC

        let usd = btc.convert(&rate);
        assert_eq!(usd.to_major_floor(), 4500); // $4,500
    }

    #[test]
    fn test_convert_same_rate_produces_same_result() {
        // Determinism: same rate always produces same result
        let amount = Amount::<USD>::from_major(100);
        let rate = Rate::<USD, EUR>::new(0.85);

        let result1 = amount.convert(&rate);
        let result2 = amount.convert(&rate);

        assert_eq!(result1, result2);
    }

    #[test]
    fn test_rate_copy_semantics() {
        let rate1 = Rate::<USD, EUR>::new(0.85);
        let rate2 = rate1; // Copy

        let amount = Amount::<USD>::from_major(100);

        // Both rates should work independently
        let eur1 = amount.convert(&rate1);
        let eur2 = amount.convert(&rate2);

        assert_eq!(eur1, eur2);
    }

    // ========================================================================
    // Conversion Safety Tests (Section 3.2)
    // ========================================================================

    #[test]
    fn test_explicit_rate_required() {
        // This test verifies that a Rate instance is always required
        let usd = Amount::<USD>::from_major(100);
        let rate = Rate::<USD, EUR>::new(0.85);

        // Conversion requires explicit rate - this is the ONLY way to convert
        let _eur = usd.convert(&rate);

        // There's no .to_eur() or implicit conversion method available
    }

    #[test]
    fn test_rate_validation_enforced() {
        // All these should pass because Rate validation is enforced at construction
        let _rate1 = Rate::<USD, EUR>::new(0.85);
        let _rate2 = Rate::<USD, EUR>::new(1.0);
        let _rate3 = Rate::<USD, EUR>::new(1000.0);

        // Invalid rates are caught at Rate construction (see rate.rs tests for panics)
    }

    #[cfg(feature = "conversion_tracking")]
    mod tracking_tests {
        use super::*;
        use crate::conversion_tracking::{ConversionEvent, ConversionTracker, NoOpTracker};
        use std::cell::RefCell;

        struct TestTracker {
            events: RefCell<Vec<(String, String, String)>>,
        }

        impl ConversionTracker for TestTracker {
            fn track<From: Currency, To: Currency>(&self, event: &ConversionEvent<From, To>) {
                self.events.borrow_mut().push((
                    event.from_amount.to_string(),
                    event.to_amount.to_string(),
                    event.from_currency_code.to_string(),
                ));
            }
        }

        #[test]
        fn test_convert_with_tracking_noop() {
            let usd = Amount::<USD>::from_major(100);
            let rate = Rate::<USD, EUR>::new(0.85);
            let tracker = NoOpTracker;

            let eur = usd.convert_with_tracking(&rate, &tracker);
            assert_eq!(eur.to_major_floor(), 85);
        }

        #[test]
        fn test_convert_with_tracking_custom() {
            let tracker = TestTracker {
                events: RefCell::new(Vec::new()),
            };

            let usd = Amount::<USD>::from_major(100);
            let rate = Rate::<USD, EUR>::new(0.85);

            let _eur = usd.convert_with_tracking(&rate, &tracker);

            let events = tracker.events.borrow();
            assert_eq!(events.len(), 1);
            assert_eq!(events[0].2, "USD");
        }

        #[test]
        fn test_tracking_captures_metadata() {
            struct MetadataTracker {
                last_timestamp: RefCell<Option<u64>>,
                last_source: RefCell<Option<&'static str>>,
            }

            impl ConversionTracker for MetadataTracker {
                fn track<From: Currency, To: Currency>(&self, event: &ConversionEvent<From, To>) {
                    *self.last_timestamp.borrow_mut() = event.timestamp_unix_secs;
                    *self.last_source.borrow_mut() = event.rate_source;
                }
            }

            let tracker = MetadataTracker {
                last_timestamp: RefCell::new(None),
                last_source: RefCell::new(None),
            };

            let usd = Amount::<USD>::from_major(100);
            let rate = Rate::<USD, EUR>::new(0.85).with_metadata(1_700_000_000, "ECB");

            let _eur = usd.convert_with_tracking(&rate, &tracker);

            assert_eq!(*tracker.last_timestamp.borrow(), Some(1_700_000_000));
            assert_eq!(*tracker.last_source.borrow(), Some("ECB"));
        }
    }
}
