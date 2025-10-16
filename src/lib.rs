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
//!
//! ## Example
//!
//! ```
//! use typed_money::{Amount, USD};
//!
//! let price = Amount::<USD>::from_major(100);  // $100.00
//! println!("{}", price);  // Displays: $100.00 USD
//!
//! // Type-safe: different currencies can't be mixed
//! let eur_price = typed_money::Amount::<typed_money::EUR>::from_major(85);
//! // This won't compile: price + eur_price  // Error: type mismatch!
//! ```

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
mod rounding;

pub use amount::Amount;
pub use currency::{Currency, BTC, ETH, EUR, GBP, JPY, USD};
pub use rounding::RoundingMode;
