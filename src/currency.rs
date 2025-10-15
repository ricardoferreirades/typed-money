//! Currency trait and built-in currency types.
//!
//! This module defines the `Currency` trait which allows compile-time type-safe
//! currency operations. Built-in currencies include USD, EUR, GBP, JPY, BTC, and ETH.

use std::fmt;

/// Trait representing a currency type.
///
/// This trait uses associated constants to define currency properties at compile time,
/// enabling zero-cost abstractions for type-safe monetary operations.
///
/// # Example
///
/// ```
/// use typed_money::Currency;
///
/// #[derive(Debug, Copy, Clone)]
/// struct MyCustomCurrency;
///
/// impl Currency for MyCustomCurrency {
///     const DECIMALS: u8 = 2;
///     const CODE: &'static str = "XXX";
///     const SYMBOL: &'static str = "¤";
/// }
/// ```
pub trait Currency: Copy + Clone + fmt::Debug + 'static {
    /// Number of decimal places for this currency (e.g., 2 for USD, 0 for JPY)
    const DECIMALS: u8;

    /// ISO 4217 currency code (e.g., "USD", "EUR")
    const CODE: &'static str;

    /// Currency symbol (e.g., "$", "€", "£")
    const SYMBOL: &'static str;
}

// ============================================================================
// Built-in Currencies
// ============================================================================

/// United States Dollar
///
/// # Example
///
/// ```
/// use typed_money::{Amount, USD};
///
/// let amount = Amount::<USD>::from_major(100);
/// println!("{}", amount);  // Displays: $100.00 USD
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct USD;

impl Currency for USD {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "USD";
    const SYMBOL: &'static str = "$";
}

/// Euro
///
/// # Example
///
/// ```
/// use typed_money::{Amount, EUR};
///
/// let amount = Amount::<EUR>::from_major(100);
/// println!("{}", amount);  // Displays: €100.00 EUR
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct EUR;

impl Currency for EUR {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "EUR";
    const SYMBOL: &'static str = "€";
}

/// British Pound Sterling
///
/// # Example
///
/// ```
/// use typed_money::{Amount, GBP};
///
/// let amount = Amount::<GBP>::from_major(100);
/// println!("{}", amount);  // Displays: £100.00 GBP
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct GBP;

impl Currency for GBP {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "GBP";
    const SYMBOL: &'static str = "£";
}

/// Japanese Yen
///
/// # Example
///
/// ```
/// use typed_money::{Amount, JPY};
///
/// let amount = Amount::<JPY>::from_major(1000);
/// println!("{}", amount);  // Displays: ¥1000 JPY
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct JPY;

impl Currency for JPY {
    const DECIMALS: u8 = 0;
    const CODE: &'static str = "JPY";
    const SYMBOL: &'static str = "¥";
}

/// Bitcoin
///
/// # Example
///
/// ```
/// use typed_money::{Amount, BTC};
///
/// let amount = Amount::<BTC>::from_major(1);
/// println!("{}", amount);  // Displays: ₿1.00000000 BTC
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct BTC;

impl Currency for BTC {
    const DECIMALS: u8 = 8;
    const CODE: &'static str = "BTC";
    const SYMBOL: &'static str = "₿";
}

/// Ethereum
///
/// # Example
///
/// ```
/// use typed_money::{Amount, ETH};
///
/// let amount = Amount::<ETH>::from_major(1);
/// println!("{}", amount);  // Displays: Ξ1.000000000000000000 ETH
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ETH;

impl Currency for ETH {
    const DECIMALS: u8 = 18;
    const CODE: &'static str = "ETH";
    const SYMBOL: &'static str = "Ξ";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_usd_constants() {
        assert_eq!(USD::DECIMALS, 2);
        assert_eq!(USD::CODE, "USD");
        assert_eq!(USD::SYMBOL, "$");
    }

    #[test]
    fn test_eur_constants() {
        assert_eq!(EUR::DECIMALS, 2);
        assert_eq!(EUR::CODE, "EUR");
        assert_eq!(EUR::SYMBOL, "€");
    }

    #[test]
    fn test_gbp_constants() {
        assert_eq!(GBP::DECIMALS, 2);
        assert_eq!(GBP::CODE, "GBP");
        assert_eq!(GBP::SYMBOL, "£");
    }

    #[test]
    fn test_jpy_constants() {
        assert_eq!(JPY::DECIMALS, 0);
        assert_eq!(JPY::CODE, "JPY");
        assert_eq!(JPY::SYMBOL, "¥");
    }

    #[test]
    fn test_btc_constants() {
        assert_eq!(BTC::DECIMALS, 8);
        assert_eq!(BTC::CODE, "BTC");
        assert_eq!(BTC::SYMBOL, "₿");
    }

    #[test]
    fn test_eth_constants() {
        assert_eq!(ETH::DECIMALS, 18);
        assert_eq!(ETH::CODE, "ETH");
        assert_eq!(ETH::SYMBOL, "Ξ");
    }

    #[test]
    fn test_currency_trait_properties() {
        // Verify currencies are Copy and Clone
        let usd1 = USD;
        let usd2 = usd1;
        let _usd3 = usd1.clone();

        // Both should still be usable (proving Copy works)
        assert_eq!(USD::CODE, "USD");
        let _ = (usd1, usd2); // Use both to prove they're independent
    }
}
