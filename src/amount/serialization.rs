//! Serde serialization support for Amount.
//!
//! This module provides Serialize and Deserialize implementations when
//! the `serde_support` feature is enabled.

#[cfg(feature = "serde_support")]
use super::type_def::Amount;

#[cfg(feature = "serde_support")]
use crate::Currency;

#[cfg(feature = "serde_support")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "serde_support")]
use std::marker::PhantomData;

#[cfg(feature = "serde_support")]
use std::str::FromStr;

#[cfg(all(feature = "use_rust_decimal", not(feature = "use_bigdecimal")))]
use rust_decimal::Decimal;

#[cfg(all(feature = "use_bigdecimal", not(feature = "use_rust_decimal")))]
use bigdecimal::BigDecimal as Decimal;

/// Serialization format for Amount.
///
/// This struct is used for JSON serialization with both value and currency code.
#[cfg(feature = "serde_support")]
#[derive(Serialize, Deserialize)]
struct AmountSerde {
    value: String,
    currency: String,
}

#[cfg(feature = "serde_support")]
impl<C: Currency> Serialize for Amount<C> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let amount_serde = AmountSerde {
            value: self.value.to_string(),
            currency: C::CODE.to_string(),
        };
        amount_serde.serialize(serializer)
    }
}

#[cfg(feature = "serde_support")]
impl<'de, C: Currency> Deserialize<'de> for Amount<C> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let amount_serde = AmountSerde::deserialize(deserializer)?;

        // Validate currency code matches
        if amount_serde.currency != C::CODE {
            return Err(serde::de::Error::custom(format!(
                "Currency mismatch: expected {}, found {}",
                C::CODE,
                amount_serde.currency
            )));
        }

        // Parse the decimal value
        let decimal_value = Decimal::from_str(&amount_serde.value)
            .map_err(|_| serde::de::Error::custom("Invalid decimal value"))?;

        Ok(Self {
            value: decimal_value,
            _currency: PhantomData,
        })
    }
}

#[cfg(all(test, feature = "serde_support"))]
mod tests {
    use super::*;
    use crate::{EUR, JPY, USD};

    #[test]
    fn test_serialize_usd() {
        let amount = Amount::<USD>::from_major(100);
        let json = serde_json::to_string(&amount).unwrap();

        assert!(json.contains("100"));
        assert!(json.contains("USD"));
    }

    #[test]
    fn test_deserialize_usd() {
        let json = r#"{"value":"100.00","currency":"USD"}"#;
        let amount: Amount<USD> = serde_json::from_str(json).unwrap();

        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_round_trip_usd() {
        let original = Amount::<USD>::from_minor(12345);
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Amount<USD> = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_serialize_eur() {
        let amount = Amount::<EUR>::from_minor(9999);
        let json = serde_json::to_string(&amount).unwrap();

        assert!(json.contains("99.99"));
        assert!(json.contains("EUR"));
    }

    #[test]
    fn test_serialize_jpy() {
        let amount = Amount::<JPY>::from_major(1000);
        let json = serde_json::to_string(&amount).unwrap();

        assert!(json.contains("1000"));
        assert!(json.contains("JPY"));
    }

    #[test]
    fn test_deserialize_currency_mismatch() {
        let json = r#"{"value":"100.00","currency":"EUR"}"#;
        let result: Result<Amount<USD>, _> = serde_json::from_str(json);

        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Currency mismatch"));
        }
    }

    #[test]
    fn test_deserialize_invalid_value() {
        let json = r#"{"value":"not a number","currency":"USD"}"#;
        let result: Result<Amount<USD>, _> = serde_json::from_str(json);

        assert!(result.is_err());
    }

    #[test]
    fn test_round_trip_negative() {
        let original = Amount::<USD>::from_major(-50);
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Amount<USD> = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_round_trip_zero() {
        let original = Amount::<USD>::from_major(0);
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: Amount<USD> = serde_json::from_str(&json).unwrap();

        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_serialize_pretty() {
        let amount = Amount::<USD>::from_major(100);
        let json = serde_json::to_string_pretty(&amount).unwrap();

        assert!(json.contains("100"));
        assert!(json.contains("USD"));
    }

    #[test]
    fn test_deserialize_different_formats() {
        // Test various valid JSON formats
        let jsons = vec![
            r#"{"value":"100","currency":"USD"}"#,
            r#"{"value":"100.00","currency":"USD"}"#,
            r#"{"value":"100.0","currency":"USD"}"#,
        ];

        for json in jsons {
            let amount: Amount<USD> = serde_json::from_str(json).unwrap();
            assert_eq!(amount.to_major_floor(), 100);
        }
    }
}
