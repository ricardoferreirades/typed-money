//! Amount type for type-safe monetary values.
//!
//! This module provides the `Amount<C>` type which represents a monetary amount
//! in a specific currency. The currency is tracked at compile time using phantom types,
//! enabling zero-cost type safety.

mod arithmetic;
mod constructors;
mod conversions;
mod currency_conversion;
mod display;
mod metadata;
mod parsing;
mod precision;
mod rounding;
#[cfg(feature = "serde_support")]
mod serialization;
mod type_def;

pub use metadata::CurrencyMetadata;
pub use type_def::Amount;
