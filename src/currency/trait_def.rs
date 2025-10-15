//! Currency trait definition.

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
