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
//! ## Regional Currencies
//!
//! ### European Regional
//! - **RON** - Romanian Leu (2 decimal places)
//! - **BGN** - Bulgarian Lev (2 decimal places)
//! - **HRK** - Croatian Kuna (2 decimal places)
//! - **RSD** - Serbian Dinar (2 decimal places)
//! - **UAH** - Ukrainian Hryvnia (2 decimal places)
//!
//! ### Asian Regional
//! - **THB** - Thai Baht (2 decimal places)
//! - **MYR** - Malaysian Ringgit (2 decimal places)
//! - **IDR** - Indonesian Rupiah (0 decimal places)
//! - **PHP** - Philippine Peso (2 decimal places)
//! - **VND** - Vietnamese Dong (0 decimal places)
//!
//! ### American Regional
//! - **COP** - Colombian Peso (2 decimal places)
//! - **PEN** - Peruvian Sol (2 decimal places)
//! - **UYU** - Uruguayan Peso (2 decimal places)
//! - **BOB** - Bolivian Boliviano (2 decimal places)
//! - **PYG** - Paraguayan Guarani (0 decimal places)
//!
//! ### African Regional
//! - **NGN** - Nigerian Naira (2 decimal places)
//! - **KES** - Kenyan Shilling (2 decimal places)
//! - **GHS** - Ghanaian Cedi (2 decimal places)
//! - **MAD** - Moroccan Dirham (2 decimal places)
//! - **TND** - Tunisian Dinar (3 decimal places)
//!
//! ### Middle Eastern Regional
//! - **QAR** - Qatari Riyal (2 decimal places)
//! - **KWD** - Kuwaiti Dinar (3 decimal places)
//! - **BHD** - Bahraini Dinar (3 decimal places)
//! - **OMR** - Omani Rial (3 decimal places)
//! - **JOD** - Jordanian Dinar (3 decimal places)
//!
//! ## Cryptocurrencies
//! - **BTC** - Bitcoin (8 decimal places - satoshis)
//! - **ETH** - Ethereum (18 decimal places - wei)
//! - **LTC** - Litecoin (8 decimal places - litoshis)
//! - **BCH** - Bitcoin Cash (8 decimal places - satoshis)
//! - **XRP** - XRP (6 decimal places - drops)
//! - **ADA** - Cardano (6 decimal places - lovelace)
//! - **DOT** - Polkadot (10 decimal places - planck)
//! - **LINK** - Chainlink (18 decimal places - wei)
//! - **UNI** - Uniswap (18 decimal places - wei)
//! - **AAVE** - Aave (18 decimal places - wei)
//!
//! ## Stablecoins
//! - **USDT** - Tether (6 decimal places - micro)
//! - **USDC** - USD Coin (6 decimal places - micro)
//! - **DAI** - Dai (18 decimal places - wei)
//! - **BUSD** - Binance USD (18 decimal places - wei)
//!
//! ## DeFi Tokens
//! - **SUSHI** - SushiSwap (18 decimal places - wei)
//! - **COMP** - Compound (18 decimal places - wei)
//! - **MKR** - Maker (18 decimal places - wei)
//! - **YFI** - Yearn Finance (18 decimal places - wei)
//!
//! ## Precious Metals
//! - **XAU** - Gold (4 decimal places - troy ounces)
//! - **XAG** - Silver (4 decimal places - troy ounces)
//! - **XPT** - Platinum (4 decimal places - troy ounces)
//! - **XPD** - Palladium (4 decimal places - troy ounces)
//! - **XDI** - Diamond (4 decimal places - carats)
//!
//! ## Base Metals
//! - **XCU** - Copper (4 decimal places - metric tons)
//! - **XAL** - Aluminum (4 decimal places - metric tons)
//! - **XZN** - Zinc (4 decimal places - metric tons)
//! - **XNI** - Nickel (4 decimal places - metric tons)
//!
//! # Examples
//!
//! ## Using Built-in Currencies
//!
//! ```
//! use typed_money::{Amount, Currency, USD, JPY, BTC, ETH, LTC, ADA, CAD, CNY, BRL, THB, NGN, QAR, XAU, XAG, XCU};
//!
//! let dollars = Amount::<USD>::from_major(100);  // $100.00
//! let yen = Amount::<JPY>::from_minor(1000);     // ¥1000 (no decimals)
//! let bitcoin = Amount::<BTC>::from_minor(100_000_000);  // 1.00000000 BTC
//! let ethereum = Amount::<ETH>::from_minor(1_000_000_000_000_000_000);  // 1.000000000000000000 ETH
//! let litecoin = Amount::<LTC>::from_major(1);   // Ł1.00000000
//! let cardano = Amount::<ADA>::from_major(1000); // ₳1000.000000
//! let cad = Amount::<CAD>::from_major(50);       // C$50.00
//! let yuan = Amount::<CNY>::from_major(200);     // ¥200.00
//! let real = Amount::<BRL>::from_major(300);     // R$300.00
//! let baht = Amount::<THB>::from_major(150);     // ฿150.00
//! let naira = Amount::<NGN>::from_major(400);    // ₦400.00
//! let riyal = Amount::<QAR>::from_major(250);    // ﷼250.00
//! let gold = Amount::<XAU>::from_major(1);       // Au1.0000
//! let silver = Amount::<XAG>::from_major(10);    // Ag10.0000
//! let copper = Amount::<XCU>::from_major(5);     // Cu5.0000
//!
//! // Each currency respects its own decimal precision
//! assert_eq!(USD::DECIMALS, 2);
//! assert_eq!(JPY::DECIMALS, 0);
//! assert_eq!(BTC::DECIMALS, 8);
//! assert_eq!(CAD::DECIMALS, 2);
//! assert_eq!(CNY::DECIMALS, 2);
//! assert_eq!(BRL::DECIMALS, 2);
//! assert_eq!(THB::DECIMALS, 2);
//! assert_eq!(NGN::DECIMALS, 2);
//! assert_eq!(QAR::DECIMALS, 2);
//! assert_eq!(XAU::DECIMALS, 4);
//! assert_eq!(XAG::DECIMALS, 4);
//! assert_eq!(XCU::DECIMALS, 4);
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

// Major Cryptocurrencies
mod aave;
mod ada;
mod bch;
mod dot;
mod link;
mod ltc;
mod uni;
mod xrp;

// Stablecoins
mod busd;
mod dai;
mod usdc;
mod usdt;

// DeFi Tokens
mod comp;
mod mkr;
mod sushi;
mod yfi;

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

// European Regional Currencies
mod bgn;
mod hrk;
mod ron;
mod rsd;
mod uah;

// Asian Regional Currencies
mod idr;
mod myr;
mod php;
mod thb;
mod vnd;

// American Regional Currencies
mod bob;
mod cop;
mod pen;
mod pyg;
mod uyu;

// African Regional Currencies
mod ghs;
mod kes;
mod mad;
mod ngn;
mod tnd;

// Middle Eastern Regional Currencies
mod bhd;
mod jod;
mod kwd;
mod omr;
mod qar;

// Precious Metals
mod xau;
mod xag;
mod xpt;
mod xpd;
mod xdi;

// Base Metals
mod xcu;
mod xal;
mod xzn;
mod xni;

pub use trait_def::Currency;

// Core currencies
pub use btc::BTC;
pub use eth::ETH;
pub use eur::EUR;
pub use gbp::GBP;
pub use jpy::JPY;
pub use usd::USD;

// Major Cryptocurrencies
pub use aave::AAVE;
pub use ada::ADA;
pub use bch::BCH;
pub use dot::DOT;
pub use link::LINK;
pub use ltc::LTC;
pub use uni::UNI;
pub use xrp::XRP;

// Stablecoins
pub use busd::BUSD;
pub use dai::DAI;
pub use usdc::USDC;
pub use usdt::USDT;

// DeFi Tokens
pub use comp::COMP;
pub use mkr::MKR;
pub use sushi::SUSHI;
pub use yfi::YFI;

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

// European Regional Currencies
pub use bgn::BGN;
pub use hrk::HRK;
pub use ron::RON;
pub use rsd::RSD;
pub use uah::UAH;

// Asian Regional Currencies
pub use idr::IDR;
pub use myr::MYR;
pub use php::PHP;
pub use thb::THB;
pub use vnd::VND;

// American Regional Currencies
pub use bob::BOB;
pub use cop::COP;
pub use pen::PEN;
pub use pyg::PYG;
pub use uyu::UYU;

// African Regional Currencies
pub use ghs::GHS;
pub use kes::KES;
pub use mad::MAD;
pub use ngn::NGN;
pub use tnd::TND;

// Middle Eastern Regional Currencies
pub use bhd::BHD;
pub use jod::JOD;
pub use kwd::KWD;
pub use omr::OMR;
pub use qar::QAR;

// Precious Metals
pub use xau::XAU;
pub use xag::XAG;
pub use xpt::XPT;
pub use xpd::XPD;
pub use xdi::XDI;

// Base Metals
pub use xcu::XCU;
pub use xal::XAL;
pub use xzn::XZN;
pub use xni::XNI;
