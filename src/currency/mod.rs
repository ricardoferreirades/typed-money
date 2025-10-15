//! Currency trait and built-in currency types.
//!
//! This module defines the `Currency` trait which allows compile-time type-safe
//! currency operations. Built-in currencies include USD, EUR, GBP, JPY, BTC, and ETH.

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
