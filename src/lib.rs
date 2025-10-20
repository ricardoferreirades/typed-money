//! # Introduction
//!
//! A type-safe money library for Rust that prevents currency mixing bugs at compile time.
//!
//! # Features
//!
//! - **Type-safe** - Currency mixing prevented at compile time
//! - **Zero-cost abstractions** - O(1) operations, no runtime overhead
//! - **100% Safe Rust** - No unsafe code
//! - **Deterministic** - Uses `rust_decimal` for precise arithmetic
//! - **Comprehensive** - Full arithmetic, conversions, rounding, and formatting
//! - **Flexible** - Optional serde support and conversion tracking
//!
//! # Quick Start
//!
//! ```
//! use typed_money::{Amount, USD, EUR, BTC, ETH, LTC, ADA, USDT, USDC, CAD, CNY, THB, NGN, Rate, RoundingMode};
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
//! // Cryptocurrency amounts
//! let bitcoin = Amount::<BTC>::from_minor(100_000_000);  // 1.00000000 BTC
//! let ethereum = Amount::<ETH>::from_major(1);           // 1.000000000000000000 ETH
//! let litecoin = Amount::<LTC>::from_major(10);          // 10.00000000 LTC
//! let cardano = Amount::<ADA>::from_major(1000);         // 1000.000000 ADA
//!
//! // Stablecoin amounts
//! let tether = Amount::<USDT>::from_major(100);          // 100.000000 USDT
//! let usd_coin = Amount::<USDC>::from_major(50);         // 50.000000 USDC
//!
//! // Multi-currency operations
//! let cad_amount = Amount::<CAD>::from_major(150);  // C$150.00
//! let cny_amount = Amount::<CNY>::from_major(700);  // ¥700.00
//!
//! // Rounding
//! let divided = Amount::<USD>::from_major(10) / 3;  // $3.333...
//! let rounded = divided.round(RoundingMode::HalfUp);  // $3.33
//! ```
//!
//! # Type Safety
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
//! # Supported Currencies
//!
//! Built-in currencies include:
//!
//! ## Major Fiat Currencies
//! - **USD** - United States Dollar (2 decimals)
//! - **EUR** - Euro (2 decimals)
//! - **GBP** - British Pound Sterling (2 decimals)
//! - **JPY** - Japanese Yen (0 decimals)
//! - **CAD** - Canadian Dollar (2 decimals)
//! - **CHF** - Swiss Franc (2 decimals)
//! - **AUD** - Australian Dollar (2 decimals)
//! - **NZD** - New Zealand Dollar (2 decimals)
//!
//! ## Asian Currencies
//! - **CNY** - Chinese Yuan (2 decimals)
//! - **KRW** - South Korean Won (0 decimals)
//! - **SGD** - Singapore Dollar (2 decimals)
//! - **HKD** - Hong Kong Dollar (2 decimals)
//! - **TWD** - New Taiwan Dollar (2 decimals)
//! - **INR** - Indian Rupee (2 decimals)
//!
//! ## European Currencies
//! - **SEK** - Swedish Krona (2 decimals)
//! - **NOK** - Norwegian Krone (2 decimals)
//! - **DKK** - Danish Krone (2 decimals)
//! - **PLN** - Polish Złoty (2 decimals)
//! - **CZK** - Czech Koruna (2 decimals)
//! - **HUF** - Hungarian Forint (0 decimals)
//!
//! ## American Currencies
//! - **BRL** - Brazilian Real (2 decimals)
//! - **MXN** - Mexican Peso (2 decimals)
//! - **ARS** - Argentine Peso (2 decimals)
//! - **CLP** - Chilean Peso (0 decimals)
//!
//! ## African/Middle Eastern Currencies
//! - **ZAR** - South African Rand (2 decimals)
//! - **EGP** - Egyptian Pound (2 decimals)
//! - **AED** - United Arab Emirates Dirham (2 decimals)
//! - **SAR** - Saudi Riyal (2 decimals)
//! - **ILS** - Israeli Shekel (2 decimals)
//! - **TRY** - Turkish Lira (2 decimals)
//!
//! ## Regional Currencies
//!
//! ### European Regional
//! - **RON** - Romanian Leu (2 decimals)
//! - **BGN** - Bulgarian Lev (2 decimals)
//! - **HRK** - Croatian Kuna (2 decimals)
//! - **RSD** - Serbian Dinar (2 decimals)
//! - **UAH** - Ukrainian Hryvnia (2 decimals)
//!
//! ### Asian Regional
//! - **THB** - Thai Baht (2 decimals)
//! - **MYR** - Malaysian Ringgit (2 decimals)
//! - **IDR** - Indonesian Rupiah (0 decimals)
//! - **PHP** - Philippine Peso (2 decimals)
//! - **VND** - Vietnamese Dong (0 decimals)
//!
//! ### American Regional
//! - **COP** - Colombian Peso (2 decimals)
//! - **PEN** - Peruvian Sol (2 decimals)
//! - **UYU** - Uruguayan Peso (2 decimals)
//! - **BOB** - Bolivian Boliviano (2 decimals)
//! - **PYG** - Paraguayan Guarani (0 decimals)
//!
//! ### African Regional
//! - **NGN** - Nigerian Naira (2 decimals)
//! - **KES** - Kenyan Shilling (2 decimals)
//! - **GHS** - Ghanaian Cedi (2 decimals)
//! - **MAD** - Moroccan Dirham (2 decimals)
//! - **TND** - Tunisian Dinar (3 decimals)
//!
//! ### Middle Eastern Regional
//! - **QAR** - Qatari Riyal (2 decimals)
//! - **KWD** - Kuwaiti Dinar (3 decimals)
//! - **BHD** - Bahraini Dinar (3 decimals)
//! - **OMR** - Omani Rial (3 decimals)
//! - **JOD** - Jordanian Dinar (3 decimals)
//!
//! ## Cryptocurrencies
//! - **BTC** - Bitcoin (8 decimals)
//! - **ETH** - Ethereum (18 decimals)
//!
//! ## Precious Metals
//! - **XAU** - Gold (4 decimals - troy ounces)
//! - **XAG** - Silver (4 decimals - troy ounces)
//! - **XPT** - Platinum (4 decimals - troy ounces)
//! - **XPD** - Palladium (4 decimals - troy ounces)
//! - **XDI** - Diamond (4 decimals - carats)
//!
//! ## Base Metals
//! - **XCU** - Copper (4 decimals - metric tons)
//! - **XAL** - Aluminum (4 decimals - metric tons)
//! - **XZN** - Zinc (4 decimals - metric tons)
//! - **XNI** - Nickel (4 decimals - metric tons)
//!
//! # Feature Flags
//!
//! - `use_rust_decimal` (default) - Use rust_decimal backend
//! - `use_bigdecimal` - Use bigdecimal backend (alternative)
//! - `serde_support` - Enable serde serialization
//! - `conversion_tracking` - Enable conversion tracking/logging
//!
//! # Examples
//!
//! The library includes comprehensive examples demonstrating various use cases.
//! Run any example with `cargo run --example <name>`:
//!
//! ### [`basic_usage`](https://github.com/ricardoferreirades/typed-money/blob/main/examples/basic_usage.rs)
//! Fundamental operations including:
//! - Creating amounts from major/minor units
//! - Arithmetic operations (add, subtract, multiply, divide)
//! - Comparisons between amounts
//! - Working with different currencies
//! - Real-world shopping cart example
//!
//! ```bash
//! cargo run --example basic_usage
//! ```
//!
//! ### [`conversions`](https://github.com/ricardoferreirades/typed-money/blob/main/examples/conversions.rs)
//! Currency conversion examples:
//! - Basic conversion with exchange rates
//! - Inverse rates
//! - Chained conversions (USD → EUR → GBP)
//! - Rate metadata for auditability
//! - International payment processing
//!
//! ```bash
//! cargo run --example conversions
//! ```
//!
//! ### [`rounding`](https://github.com/ricardoferreirades/typed-money/blob/main/examples/rounding.rs)
//! Demonstrates all 7 rounding modes:
//! - HalfUp, HalfDown, HalfEven (Banker's)
//! - Up, Down, Floor, Ceiling
//! - Edge cases with negative numbers
//! - Tax and interest calculations
//! - When to use each mode
//!
//! ```bash
//! cargo run --example rounding
//! ```
//!
//! ### [`custom_currency`](https://github.com/ricardoferreirades/typed-money/blob/main/examples/custom_currency.rs)
//! Defining custom currencies:
//! - Custom fiat currencies (CAD, CHF, AUD)
//! - Cryptocurrencies (DOGE)
//! - Game currencies (GOLD, GEMS)
//! - Loyalty points systems
//! - Multi-currency wallets
//!
//! ```bash
//! cargo run --example custom_currency
//! ```
//!
//! ### [`error_handling`](https://github.com/ricardoferreirades/typed-money/blob/main/examples/error_handling.rs)
//! Comprehensive error handling:
//! - Parse errors with recovery
//! - Precision errors and normalization
//! - Invalid rate validation
//! - Error propagation with `?`
//! - User input validation
//!
//! ```bash
//! cargo run --example error_handling
//! ```
//!
//! ### [`serialization`](https://github.com/ricardoferreirades/typed-money/blob/main/examples/serialization.rs)
//! Serde integration (requires `serde_support` feature):
//! - JSON serialization/deserialization
//! - Struct serialization with amounts
//! - Collections and multi-currency data
//! - API response handling
//! - Persistence examples
//!
//! ```bash
//! cargo run --example serialization --features serde_support
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
mod error;
mod rate;
mod rounding;

#[cfg(feature = "conversion_tracking")]
pub mod conversion_tracking;

pub use amount::{Amount, CurrencyMetadata};
pub use currency::{
    // Core currencies
    Currency,
    // Currency metadata types
    CurrencyType,
    LiquidityRating,
    SymbolPosition,
    VolatilityRating,
    AAVE,
    // Major Cryptocurrencies
    ADA,
    AED,
    ARS,
    AUD,
    BCH,
    // European Regional Currencies
    BGN,
    // Middle Eastern Regional Currencies
    BHD,
    // American Regional Currencies
    BOB,
    // American Currencies
    BRL,
    BTC,
    // Stablecoins
    BUSD,
    // Major Fiat Currencies
    CAD,
    CHF,
    CLP,
    // Asian Currencies
    CNY,
    // DeFi Tokens
    COMP,
    COP,
    CZK,
    DAI,
    DKK,
    DOT,
    EGP,
    ETH,
    EUR,
    GBP,
    // African Regional Currencies
    GHS,
    HKD,
    HRK,
    HUF,
    // Asian Regional Currencies
    IDR,
    ILS,
    INR,
    JOD,
    JPY,
    KES,
    KRW,
    KWD,
    LINK,
    LTC,
    MAD,
    MKR,
    MXN,
    MYR,
    NGN,
    NOK,
    NZD,
    OMR,
    PEN,
    PHP,
    PLN,
    PYG,
    QAR,
    RON,
    RSD,
    SAR,
    // European Currencies
    SEK,
    SGD,
    SUSHI,
    THB,
    TND,
    TRY,
    TWD,
    UAH,
    UNI,
    USD,
    USDC,
    USDT,
    UYU,
    VND,
    XAG,
    XAL,
    // Precious Metals
    XAU,
    // Base Metals
    XCU,
    XDI,
    XNI,
    XPD,
    XPT,
    XRP,
    XZN,
    YFI,
    // African/Middle Eastern Currencies
    ZAR,
};
pub use error::{MoneyError, MoneyResult};
pub use rate::Rate;
pub use rounding::RoundingMode;
