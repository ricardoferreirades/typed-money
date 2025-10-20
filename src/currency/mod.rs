//! Currency trait and built-in currency types.
//!
//! This module defines the `Currency` trait which enables compile-time type-safe
//! currency operations. The library includes common fiat and cryptocurrencies,
//! and you can define custom currencies by implementing the trait.
//!
//! # Built-in Currencies
//!
//! ## Major Fiat Currencies
//! - **USD** - United States Dollar (2 decimal places)
//! - **EUR** - Euro (2 decimal places)
//! - **GBP** - British Pound Sterling (2 decimal places)
//! - **JPY** - Japanese Yen (0 decimal places)
//! - **CAD** - Canadian Dollar (2 decimal places)
//! - **CHF** - Swiss Franc (2 decimal places)
//! - **AUD** - Australian Dollar (2 decimal places)
//! - **NZD** - New Zealand Dollar (2 decimal places)
//!
//! ## Asian Currencies
//! - **CNY** - Chinese Yuan (2 decimal places)
//! - **KRW** - South Korean Won (0 decimal places)
//! - **SGD** - Singapore Dollar (2 decimal places)
//! - **HKD** - Hong Kong Dollar (2 decimal places)
//! - **TWD** - New Taiwan Dollar (2 decimal places)
//! - **INR** - Indian Rupee (2 decimal places)
//!
//! ## European Currencies
//! - **SEK** - Swedish Krona (2 decimal places)
//! - **NOK** - Norwegian Krone (2 decimal places)
//! - **DKK** - Danish Krone (2 decimal places)
//! - **PLN** - Polish Złoty (2 decimal places)
//! - **CZK** - Czech Koruna (2 decimal places)
//! - **HUF** - Hungarian Forint (0 decimal places)
//!
//! ## American Currencies
//! - **BRL** - Brazilian Real (2 decimal places)
//! - **MXN** - Mexican Peso (2 decimal places)
//! - **ARS** - Argentine Peso (2 decimal places)
//! - **CLP** - Chilean Peso (0 decimal places)
//!
//! ## African/Middle Eastern Currencies
//! - **ZAR** - South African Rand (2 decimal places)
//! - **EGP** - Egyptian Pound (2 decimal places)
//! - **AED** - United Arab Emirates Dirham (2 decimal places)
//! - **SAR** - Saudi Riyal (2 decimal places)
//! - **ILS** - Israeli Shekel (2 decimal places)
//! - **TRY** - Turkish Lira (2 decimal places)
//!
//! ## Cryptocurrencies
//! - **BTC** - Bitcoin (8 decimal places - satoshis)
//! - **ETH** - Ethereum (18 decimal places - wei)
//!
//! # Examples
//!
//! ## Using Built-in Currencies
//!
//! ```
//! use typed_money::{Amount, Currency, USD, JPY, BTC, CAD, CNY, BRL};
//!
//! let dollars = Amount::<USD>::from_major(100);  // $100.00
//! let yen = Amount::<JPY>::from_minor(1000);     // ¥1000 (no decimals)
//! let bitcoin = Amount::<BTC>::from_minor(100_000_000);  // 1.00000000 BTC
//! let cad = Amount::<CAD>::from_major(50);       // C$50.00
//! let yuan = Amount::<CNY>::from_major(200);     // ¥200.00
//! let real = Amount::<BRL>::from_major(300);     // R$300.00
//!
//! // Each currency respects its own decimal precision
//! assert_eq!(USD::DECIMALS, 2);
//! assert_eq!(JPY::DECIMALS, 0);
//! assert_eq!(BTC::DECIMALS, 8);
//! assert_eq!(CAD::DECIMALS, 2);
//! assert_eq!(CNY::DECIMALS, 2);
//! assert_eq!(BRL::DECIMALS, 2);
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

// Major Fiat Currencies
mod aud;
mod cad;
mod chf;
mod nzd;

// Asian Currencies
mod cny;
mod hkd;
mod inr;
mod krw;
mod sgd;
mod twd;

// European Currencies
mod czk;
mod dkk;
mod huf;
mod nok;
mod pln;
mod sek;

// American Currencies
mod ars;
mod brl;
mod clp;
mod mxn;

// African/Middle Eastern Currencies
mod aed;
mod egp;
mod ils;
mod sar;
mod try_currency;
mod zar;

pub use trait_def::Currency;

// Core currencies
pub use btc::BTC;
pub use eth::ETH;
pub use eur::EUR;
pub use gbp::GBP;
pub use jpy::JPY;
pub use usd::USD;

// Major Fiat Currencies
pub use aud::AUD;
pub use cad::CAD;
pub use chf::CHF;
pub use nzd::NZD;

// Asian Currencies
pub use cny::CNY;
pub use hkd::HKD;
pub use inr::INR;
pub use krw::KRW;
pub use sgd::SGD;
pub use twd::TWD;

// European Currencies
pub use czk::CZK;
pub use dkk::DKK;
pub use huf::HUF;
pub use nok::NOK;
pub use pln::PLN;
pub use sek::SEK;

// American Currencies
pub use ars::ARS;
pub use brl::BRL;
pub use clp::CLP;
pub use mxn::MXN;

// African/Middle Eastern Currencies
pub use aed::AED;
pub use egp::EGP;
pub use ils::ILS;
pub use sar::SAR;
pub use try_currency::TRY;
pub use zar::ZAR;
