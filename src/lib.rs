//! # Typed Money
//!
//! A type-safe money library for Rust that prevents currency mixing bugs at compile time.
//!
//! ## Features
//!
//! - **Type-safe** - Currency mixing prevented at compile time
//! - **Zero-cost abstractions** - O(1) operations, no runtime overhead
//! - **100% Safe Rust** - No unsafe code
//! - **Deterministic** - Uses `rust_decimal` for precise arithmetic
//! - **Comprehensive** - Full arithmetic, conversions, rounding, and formatting
//! - **Flexible** - Optional serde support and conversion tracking
//!
//! ## Quick Start
//!
//! ```
//! use typed_money::{Amount, USD, EUR, Rate, RoundingMode};
//!
//! // Create amounts
//! let price = Amount::<USD>::from_major(100);  // $100.00
//! let tax = Amount::<USD>::from_minor(850);     // $8.50
//!
//! // Arithmetic operations
//! let total = price + tax;  // $108.50
//! assert_eq!(total.to_minor(), 10850);
//!
//! // Currency conversion
//! let rate = Rate::<USD, EUR>::new(0.85);
//! let eur_price = price.convert(&rate);  // €85.00
//!
//! // Rounding
//! let divided = Amount::<USD>::from_major(10) / 3;  // $3.333...
//! let rounded = divided.round(RoundingMode::HalfUp);  // $3.33
//! ```
//!
//! ## Type Safety
//!
//! The library prevents currency mixing at compile time:
//!
//! ```compile_fail
//! use typed_money::{Amount, USD, EUR};
//!
//! let usd = Amount::<USD>::from_major(100);
//! let eur = Amount::<EUR>::from_major(85);
//!
//! // This won't compile - type mismatch!
//! let invalid = usd + eur;
//! ```
//!
//! ## Supported Currencies
//!
//! Built-in currencies include:
//! - **USD** - United States Dollar (2 decimals)
//! - **EUR** - Euro (2 decimals)
//! - **GBP** - British Pound Sterling (2 decimals)
//! - **JPY** - Japanese Yen (0 decimals)
//! - **BTC** - Bitcoin (8 decimals)
//! - **ETH** - Ethereum (18 decimals)
//!
//! ## Feature Flags
//!
//! - `use_rust_decimal` (default) - Use rust_decimal backend
//! - `use_bigdecimal` - Use bigdecimal backend (alternative)
//! - `serde_support` - Enable serde serialization
//! - `conversion_tracking` - Enable conversion tracking/logging

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::all)]

// Ensure exactly one decimal backend is enabled
#[cfg(not(any(feature = "use_rust_decimal", feature = "use_bigdecimal")))]
compile_error!("Either 'use_rust_decimal' or 'use_bigdecimal' feature must be enabled");

#[cfg(all(feature = "use_rust_decimal", feature = "use_bigdecimal"))]
compile_error!("Only one decimal backend can be enabled at a time");

mod amount;
mod currency;
mod error;
mod rate;
mod rounding;

#[cfg(feature = "conversion_tracking")]
pub mod conversion_tracking;

pub use amount::Amount;
pub use currency::{Currency, BTC, ETH, EUR, GBP, JPY, USD};
pub use error::{MoneyError, MoneyResult};
pub use rate::Rate;
pub use rounding::RoundingMode;
