//! Currency trait definition.

use std::fmt;

use super::metadata::{CurrencyType, SymbolPosition, VolatilityRating, LiquidityRating};

/// Trait representing a currency type.
///
/// This trait uses associated constants to define currency properties at compile time,
/// enabling zero-cost abstractions for type-safe monetary operations.
///
/// The trait includes both required fields (DECIMALS, CODE, SYMBOL) and optional
/// metadata fields with sensible defaults. Users can implement only the required
/// fields for basic functionality, or provide rich metadata for enhanced features.
///
/// # Example
///
/// ## Basic Implementation
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
///     // All metadata fields use defaults
/// }
/// ```
///
/// ## Rich Implementation
/// ```
/// use typed_money::{Currency, CurrencyType, SymbolPosition};
///
/// #[derive(Debug, Copy, Clone)]
/// struct USD;
///
/// impl Currency for USD {
///     const DECIMALS: u8 = 2;
///     const CODE: &'static str = "USD";
///     const SYMBOL: &'static str = "$";
///     
///     // Rich metadata
///     const NAME: &'static str = "US Dollar";
///     const COUNTRY: &'static str = "United States";
///     const REGION: &'static str = "North America";
///     const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
///     const IS_MAJOR: bool = true;
///     const IS_STABLE: bool = true;
///     const INTRODUCED_YEAR: u16 = 1792;
///     const ISO_4217_NUMBER: u16 = 840;
/// }
/// ```
pub trait Currency: Copy + Clone + fmt::Debug + 'static {
    // === REQUIRED FIELDS ===
    
    /// Number of decimal places for this currency (e.g., 2 for USD, 0 for JPY)
    const DECIMALS: u8;

    /// ISO 4217 currency code (e.g., "USD", "EUR")
    const CODE: &'static str;

    /// Currency symbol (e.g., "$", "€", "£")
    const SYMBOL: &'static str;

    // === OPTIONAL METADATA FIELDS ===
    // All metadata fields have sensible defaults and are optional.
    // Users can implement only the fields they need.

    /// Full currency name (e.g., "US Dollar", "Euro", "Bitcoin")
    const NAME: &'static str = "";

    /// Primary country or region that issues this currency
    const COUNTRY: &'static str = "";

    /// Geographic region where this currency is primarily used
    const REGION: &'static str = "";

    /// Type of currency (Fiat, Cryptocurrency, or Commodity)
    const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;

    /// Whether this is a major currency (USD, EUR, GBP, JPY, etc.)
    const IS_MAJOR: bool = false;

    /// Whether this is a stable currency (major fiat, stablecoins)
    const IS_STABLE: bool = false;

    // === FORMATTING METADATA ===

    /// Character used to separate thousands (e.g., ',' for US, '.' for EU)
    const THOUSANDS_SEPARATOR: char = ',';

    /// Character used as decimal separator (e.g., '.' for US, ',' for EU)
    const DECIMAL_SEPARATOR: char = '.';

    /// Position of currency symbol relative to the amount
    const SYMBOL_POSITION: SymbolPosition = SymbolPosition::Before;

    /// Whether to include a space between symbol and amount
    const SPACE_BETWEEN: bool = false;

    // === HISTORICAL METADATA ===

    /// Year when this currency was introduced
    const INTRODUCED_YEAR: u16 = 0;

    /// Official ISO 4217 numeric code
    const ISO_4217_NUMBER: u16 = 0;

    // === TRADING METADATA ===
    // Note: These fields are for static information only.
    // Dynamic trading data (current volatility, liquidity) should be
    // fetched from external sources at runtime.

    /// Static volatility rating (Low, Medium, High)
    const VOLATILITY_RATING: VolatilityRating = VolatilityRating::Medium;

    /// Static liquidity rating (Low, Medium, High)
    const LIQUIDITY_RATING: LiquidityRating = LiquidityRating::Medium;
}
