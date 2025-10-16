//! Currency conversion methods for Amount.
//!
//! Provides explicit currency conversion using exchange rates.

use super::type_def::Amount;
use crate::{Currency, Rate};
use std::marker::PhantomData;

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
    /// ```compile_fail
    /// use typed_money::{Amount, Rate, USD, EUR, GBP};
    ///
    /// let usd = Amount::<USD>::from_major(100);
    /// let wrong_rate = Rate::<EUR, GBP>::new(0.88);
    ///
    /// // This won't compile!
    /// let invalid = usd.convert(&wrong_rate);  // Error: type mismatch
    /// ```
    pub fn convert<To: Currency>(&self, rate: &Rate<C, To>) -> Amount<To> {
        Amount {
            value: self.value * rate.value(),
            _currency: PhantomData,
        }
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
}
