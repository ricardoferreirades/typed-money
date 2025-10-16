//! Conversion tracking for auditing currency conversions.
//!
//! This module provides optional tracking capabilities for monitoring and logging
//! currency conversions when the `conversion_tracking` feature is enabled.

use crate::Currency;
use std::marker::PhantomData;

#[cfg(all(feature = "use_rust_decimal", not(feature = "use_bigdecimal")))]
use rust_decimal::Decimal;

#[cfg(all(feature = "use_bigdecimal", not(feature = "use_rust_decimal")))]
use bigdecimal::BigDecimal as Decimal;

/// A record of a currency conversion event.
///
/// This struct captures all relevant information about a conversion for
/// auditing and logging purposes.
///
/// # Type Parameters
///
/// * `From` - The source currency type
/// * `To` - The target currency type
#[derive(Debug, Clone, PartialEq)]
pub struct ConversionEvent<From: Currency, To: Currency> {
    /// The source amount value
    pub from_amount: Decimal,
    /// The target amount value after conversion
    pub to_amount: Decimal,
    /// The exchange rate used
    pub rate: Decimal,
    /// Optional timestamp when the conversion occurred (UNIX seconds)
    pub timestamp_unix_secs: Option<u64>,
    /// Optional source of the exchange rate
    pub rate_source: Option<&'static str>,
    /// Source currency code (for runtime inspection)
    pub from_currency_code: &'static str,
    /// Target currency code (for runtime inspection)
    pub to_currency_code: &'static str,
    /// Phantom data for type safety (zero runtime cost)
    _phantom: PhantomData<(From, To)>,
}

impl<From: Currency, To: Currency> ConversionEvent<From, To> {
    /// Creates a new conversion event.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        from_amount: Decimal,
        to_amount: Decimal,
        rate: Decimal,
        timestamp_unix_secs: Option<u64>,
        rate_source: Option<&'static str>,
    ) -> Self {
        Self {
            from_amount,
            to_amount,
            rate,
            timestamp_unix_secs,
            rate_source,
            from_currency_code: From::CODE,
            to_currency_code: To::CODE,
            _phantom: PhantomData,
        }
    }
}

/// Trait for implementing custom conversion tracking/logging.
///
/// Implement this trait to define custom behavior for tracking conversions,
/// such as logging to a file, sending to a monitoring service, or storing
/// in a database.
///
/// # Examples
///
/// ```
/// use typed_money::conversion_tracking::{ConversionEvent, ConversionTracker};
/// use typed_money::{Currency, USD, EUR};
///
/// struct SimpleLogger;
///
/// impl ConversionTracker for SimpleLogger {
///     fn track<From: Currency, To: Currency>(&self, event: &ConversionEvent<From, To>) {
///         println!("Converted {} {} to {} {}",
///             event.from_amount, event.from_currency_code,
///             event.to_amount, event.to_currency_code);
///     }
/// }
/// ```
pub trait ConversionTracker {
    /// Called when a conversion occurs.
    fn track<From: Currency, To: Currency>(&self, event: &ConversionEvent<From, To>);
}

/// A no-op tracker that does nothing.
///
/// This is useful as a default or for disabling tracking.
#[derive(Debug, Clone, Copy)]
pub struct NoOpTracker;

impl ConversionTracker for NoOpTracker {
    fn track<From: Currency, To: Currency>(&self, _event: &ConversionEvent<From, To>) {
        // Do nothing
    }
}

#[cfg(test)]
#[cfg(not(all(feature = "use_rust_decimal", feature = "use_bigdecimal")))]
mod tests {
    use super::*;
    use crate::{EUR, USD};

    #[cfg(feature = "use_rust_decimal")]
    use rust_decimal::Decimal;

    #[cfg(feature = "use_bigdecimal")]
    use bigdecimal::BigDecimal as Decimal;

    #[test]
    fn test_conversion_event_creation() {
        let event = ConversionEvent::<USD, EUR>::new(
            Decimal::from(100),
            Decimal::new(85, 0),
            Decimal::new(85, 2),
            Some(1_700_000_000),
            Some("ECB"),
        );

        assert_eq!(event.from_amount, Decimal::from(100));
        assert_eq!(event.to_amount, Decimal::new(85, 0));
        assert_eq!(event.rate, Decimal::new(85, 2));
        assert_eq!(event.timestamp_unix_secs, Some(1_700_000_000));
        assert_eq!(event.rate_source, Some("ECB"));
        assert_eq!(event.from_currency_code, "USD");
        assert_eq!(event.to_currency_code, "EUR");
    }

    #[test]
    fn test_noop_tracker() {
        let tracker = NoOpTracker;
        let event = ConversionEvent::<USD, EUR>::new(
            Decimal::from(100),
            Decimal::new(85, 0),
            Decimal::new(85, 2),
            None,
            None,
        );

        // Should not panic or do anything
        tracker.track(&event);
    }

    struct CountingTracker {
        count: std::cell::RefCell<usize>,
    }

    impl ConversionTracker for CountingTracker {
        fn track<From: Currency, To: Currency>(&self, _event: &ConversionEvent<From, To>) {
            *self.count.borrow_mut() += 1;
        }
    }

    #[test]
    fn test_custom_tracker() {
        let tracker = CountingTracker {
            count: std::cell::RefCell::new(0),
        };

        let event = ConversionEvent::<USD, EUR>::new(
            Decimal::from(100),
            Decimal::new(85, 0),
            Decimal::new(85, 2),
            None,
            None,
        );

        tracker.track(&event);
        tracker.track(&event);

        assert_eq!(*tracker.count.borrow(), 2);
    }
}
