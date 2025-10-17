//! Currency trait and built-in currency types.
//!
//! This module defines the `Currency` trait which enables compile-time type-safe
//! currency operations. The library includes common fiat and cryptocurrencies,
//! and you can define custom currencies by implementing the trait.
//!
//! # Built-in Currencies
//!
//! - **USD** - United States Dollar (2 decimal places)
//! - **EUR** - Euro (2 decimal places)
//! - **GBP** - British Pound Sterling (2 decimal places)
//! - **JPY** - Japanese Yen (0 decimal places)
//! - **BTC** - Bitcoin (8 decimal places - satoshis)
//! - **ETH** - Ethereum (18 decimal places - wei)
//!
//! # Examples
//!
//! ## Using Built-in Currencies
//!
//! ```
//! use typed_money::{Amount, Currency, USD, JPY, BTC};
//!
//! let dollars = Amount::<USD>::from_major(100);  // $100.00
//! let yen = Amount::<JPY>::from_minor(1000);     // ¥1000 (no decimals)
//! let bitcoin = Amount::<BTC>::from_minor(100_000_000);  // 1.00000000 BTC
//!
//! // Each currency respects its own decimal precision
//! assert_eq!(USD::DECIMALS, 2);
//! assert_eq!(JPY::DECIMALS, 0);
//! assert_eq!(BTC::DECIMALS, 8);
//! ```
//!
//! ## Defining Custom Currencies
//!
//! ```
//! use typed_money::{Currency, Amount};
//!
//! #[derive(Debug, Copy, Clone)]
//! struct CAD;  // Canadian Dollar
//!
//! impl Currency for CAD {
//!     const DECIMALS: u8 = 2;
//!     const CODE: &'static str = "CAD";
//!     const SYMBOL: &'static str = "C$";
//! }
//!
//! let cad = Amount::<CAD>::from_major(50);  // C$50.00
//! assert_eq!(cad.to_minor(), 5000);
//! ```
//!
//! ## Currency Properties
//!
//! ```
//! use typed_money::{Currency, EUR, GBP};
//!
//! // Access currency metadata at compile time
//! println!("EUR code: {}", EUR::CODE);      // "EUR"
//! println!("EUR symbol: {}", EUR::SYMBOL);  // "€"
//! println!("GBP decimals: {}", GBP::DECIMALS);  // 2
//! ```

mod trait_def;

// Built-in currencies
mod btc;
mod eth;
mod eur;
mod gbp;
mod jpy;
mod usd;

pub use trait_def::Currency;

pub use btc::BTC;
pub use eth::ETH;
pub use eur::EUR;
pub use gbp::GBP;
pub use jpy::JPY;
pub use usd::USD;
