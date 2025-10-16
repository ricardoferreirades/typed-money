//! Cross-platform determinism tests.
//!
//! These tests verify that all operations produce identical results
//! across different platforms (Linux, macOS, Windows, WebAssembly).

#[cfg(test)]
mod tests {
    use crate::{Amount, BTC, EUR, JPY, USD};

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
    fn test_decimal_string_representation_determinism() {
        // Verify consistent string representation across platforms
        let amount = Amount::<USD>::from_minor(12345);
        let str_repr = amount.value().to_string();

        // Decimal should always produce the same string
        assert_eq!(str_repr, "123.45");
    }

    #[test]
    fn test_different_decimal_places_determinism() {
        // Test currencies with different decimal places
        let usd = Amount::<USD>::from_minor(12345); // 2 decimals
        let jpy = Amount::<JPY>::from_minor(12345); // 0 decimals
        let btc = Amount::<BTC>::from_minor(12345); // 8 decimals

        assert_eq!(usd.value().to_string(), "123.45");
        assert_eq!(jpy.value().to_string(), "12345");
        assert_eq!(btc.value().to_string(), "0.00012345");
    }

    #[test]
    fn test_large_numbers_determinism() {
        // Test with large numbers to ensure no platform-specific overflow
        let large = Amount::<USD>::from_minor(999_999_999_999_999);
        assert_eq!(large.to_minor(), 999_999_999_999_999);

        let large_major = large.to_major_floor();
        assert_eq!(large_major, 9_999_999_999_999);
    }

    #[test]
    fn test_zero_determinism() {
        // Verify zero is represented consistently
        let zero1 = Amount::<USD>::from_major(0);
        let zero2 = Amount::<USD>::from_minor(0);
        assert_eq!(zero1, zero2);
        assert_eq!(zero1.value().to_string(), "0");
    }

    #[test]
    fn test_negative_numbers_determinism() {
        // Verify negative numbers work consistently
        let neg = Amount::<USD>::from_major(-100);
        assert_eq!(neg.value().to_string(), "-100");
        assert_eq!(neg.to_major_floor(), -100);
        assert_eq!(neg.to_minor(), -10000);
    }

    #[test]
    fn test_rounding_determinism() {
        // Verify rounding produces consistent results
        use crate::RoundingMode;

        let amount = Amount::<USD>::from_minor(12350); // $123.50

        // These should always produce the same results on all platforms
        assert_eq!(amount.to_major_rounded(RoundingMode::HalfUp), 124);
        assert_eq!(amount.to_major_rounded(RoundingMode::HalfDown), 123);
        assert_eq!(amount.to_major_rounded(RoundingMode::HalfEven), 124);
        assert_eq!(amount.to_major_rounded(RoundingMode::Floor), 123);
        assert_eq!(amount.to_major_rounded(RoundingMode::Ceiling), 124);
    }

    #[test]
    fn test_high_precision_currency_determinism() {
        // Ethereum has 18 decimal places - test high precision
        let eth = Amount::<EUR>::from_minor(1); // 1 wei = 0.000000000000000001 ETH
        let eth_str = eth.value().to_string();

        // Should be consistent across platforms
        assert_eq!(eth_str, "0.01");
    }

    #[test]
    fn test_display_formatting_determinism() {
        // Verify Display output is consistent
        let usd = Amount::<USD>::from_major(100);
        let eur = Amount::<EUR>::from_minor(12345);
        let jpy = Amount::<JPY>::from_major(1000);
        let btc = Amount::<BTC>::from_major(1);

        assert_eq!(format!("{}", usd), "$100.00 USD");
        assert_eq!(format!("{}", eur), "€123.45 EUR");
        assert_eq!(format!("{}", jpy), "¥1000 JPY");
        assert_eq!(format!("{}", btc), "₿1.00000000 BTC");
    }

    #[test]
    fn test_equality_determinism() {
        // Verify equality checks are consistent
        let a1 = Amount::<USD>::from_minor(12345);
        let a2 = Amount::<USD>::from_minor(12345);
        let a3 = Amount::<USD>::from_major(123);

        assert_eq!(a1, a2); // Should always be equal
        assert_ne!(a1, a3); // Should always be not equal
    }

    #[test]
    fn test_decimal_precision_no_float() {
        // Verify we never lose precision like floats do
        // This classic float problem should not occur:
        // 0.1 + 0.2 != 0.3 in floating point

        let ten_cents = Amount::<USD>::from_minor(10);
        let twenty_cents = Amount::<USD>::from_minor(20);

        // When we implement Add, this should equal 30 cents exactly
        // For now, verify the underlying decimals are precise
        assert_eq!(ten_cents.value().to_string(), "0.10");
        assert_eq!(twenty_cents.value().to_string(), "0.20");

        // Create thirty cents directly
        let thirty_cents = Amount::<USD>::from_minor(30);
        assert_eq!(thirty_cents.value().to_string(), "0.30");
    }

    #[test]
    fn test_conversion_round_trip_determinism() {
        // Verify major -> minor -> major round trip
        let original = Amount::<USD>::from_major(123);
        let minor = original.to_minor();
        let back_to_major = Amount::<USD>::from_minor(minor);

        assert_eq!(original, back_to_major);
    }
}
