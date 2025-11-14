//! Currency metadata access methods for Amount.
#[cfg(not(feature = "std"))]
use crate::inner_prelude::*;
use crate::{Amount, Currency};

/// Extension trait for accessing currency metadata from Amount instances.
///
/// This trait provides convenient methods to access currency metadata
/// without needing to know the specific currency type.
///
/// # Example
///
/// ```
/// use typed_money::{Amount, USD, CurrencyMetadata};
///
/// let amount = Amount::<USD>::from_major(100);
/// println!("Currency: {}", amount.currency_name());
/// println!("Country: {}", amount.currency_country());
/// println!("Is major: {}", amount.is_major_currency());
/// ```
pub trait CurrencyMetadata {
    /// Returns the full name of the currency.
    fn currency_name(&self) -> &'static str;

    /// Returns the primary country or region that issues this currency.
    fn currency_country(&self) -> &'static str;

    /// Returns the geographic region where this currency is primarily used.
    fn currency_region(&self) -> &'static str;

    /// Returns the type of currency (Fiat, Cryptocurrency, or Commodity).
    fn currency_type(&self) -> crate::CurrencyType;

    /// Returns whether this is a major currency.
    fn is_major_currency(&self) -> bool;

    /// Returns whether this is a stable currency.
    fn is_stable_currency(&self) -> bool;

    /// Returns the year when this currency was introduced.
    fn currency_introduced_year(&self) -> u16;

    /// Returns the official ISO 4217 numeric code.
    fn currency_iso_number(&self) -> u16;

    /// Returns the character used to separate thousands.
    fn thousands_separator(&self) -> char;

    /// Returns the character used as decimal separator.
    fn decimal_separator(&self) -> char;

    /// Returns the position of currency symbol relative to the amount.
    fn symbol_position(&self) -> crate::SymbolPosition;

    /// Returns whether to include a space between symbol and amount.
    fn space_between_symbol(&self) -> bool;

    /// Returns the static volatility rating.
    fn volatility_rating(&self) -> crate::VolatilityRating;

    /// Returns the static liquidity rating.
    fn liquidity_rating(&self) -> crate::LiquidityRating;

    /// Returns a formatted string with currency information.
    fn currency_info(&self) -> String;
}

impl<C: Currency> CurrencyMetadata for Amount<C> {
    fn currency_name(&self) -> &'static str {
        C::NAME
    }

    fn currency_country(&self) -> &'static str {
        C::COUNTRY
    }

    fn currency_region(&self) -> &'static str {
        C::REGION
    }

    fn currency_type(&self) -> crate::CurrencyType {
        C::CURRENCY_TYPE
    }

    fn is_major_currency(&self) -> bool {
        C::IS_MAJOR
    }

    fn is_stable_currency(&self) -> bool {
        C::IS_STABLE
    }

    fn currency_introduced_year(&self) -> u16 {
        C::INTRODUCED_YEAR
    }

    fn currency_iso_number(&self) -> u16 {
        C::ISO_4217_NUMBER
    }

    fn thousands_separator(&self) -> char {
        C::THOUSANDS_SEPARATOR
    }

    fn decimal_separator(&self) -> char {
        C::DECIMAL_SEPARATOR
    }

    fn symbol_position(&self) -> crate::SymbolPosition {
        C::SYMBOL_POSITION
    }

    fn space_between_symbol(&self) -> bool {
        C::SPACE_BETWEEN
    }

    fn volatility_rating(&self) -> crate::VolatilityRating {
        C::VOLATILITY_RATING
    }

    fn liquidity_rating(&self) -> crate::LiquidityRating {
        C::LIQUIDITY_RATING
    }

    fn currency_info(&self) -> String {
        format!(
            "{} ({}) - {} - {} - {}",
            C::NAME,
            C::CODE,
            C::COUNTRY,
            C::CURRENCY_TYPE,
            if C::IS_MAJOR { "Major" } else { "Minor" }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Currency, CurrencyType, LiquidityRating, SymbolPosition, VolatilityRating};

    // Test currency with minimal metadata
    #[derive(Debug, Copy, Clone)]
    struct TestCurrency;

    impl Currency for TestCurrency {
        const DECIMALS: u8 = 2;
        const CODE: &'static str = "TEST";
        const SYMBOL: &'static str = "T";
        // All metadata fields use defaults
    }

    // Test currency with rich metadata
    #[derive(Debug, Copy, Clone)]
    struct RichTestCurrency;

    impl Currency for RichTestCurrency {
        const DECIMALS: u8 = 2;
        const CODE: &'static str = "RICH";
        const SYMBOL: &'static str = "R";

        const NAME: &'static str = "Rich Test Currency";
        const COUNTRY: &'static str = "Test Country";
        const REGION: &'static str = "Test Region";
        const CURRENCY_TYPE: CurrencyType = CurrencyType::Fiat;
        const IS_MAJOR: bool = true;
        const IS_STABLE: bool = true;
        const INTRODUCED_YEAR: u16 = 2024;
        const ISO_4217_NUMBER: u16 = 999;
        const THOUSANDS_SEPARATOR: char = '.';
        const DECIMAL_SEPARATOR: char = ',';
        const SYMBOL_POSITION: SymbolPosition = SymbolPosition::After;
        const SPACE_BETWEEN: bool = true;
        const VOLATILITY_RATING: VolatilityRating = VolatilityRating::Low;
        const LIQUIDITY_RATING: LiquidityRating = LiquidityRating::High;
    }

    #[test]
    fn test_minimal_currency_metadata() {
        let amount = Amount::<TestCurrency>::from_major(100);

        // Test default values
        assert_eq!(amount.currency_name(), "");
        assert_eq!(amount.currency_country(), "");
        assert_eq!(amount.currency_region(), "");
        assert_eq!(amount.currency_type(), CurrencyType::Fiat);
        assert!(!amount.is_major_currency());
        assert!(!amount.is_stable_currency());
        assert_eq!(amount.currency_introduced_year(), 0);
        assert_eq!(amount.currency_iso_number(), 0);
        assert_eq!(amount.thousands_separator(), ',');
        assert_eq!(amount.decimal_separator(), '.');
        assert_eq!(amount.symbol_position(), SymbolPosition::Before);
        assert!(!amount.space_between_symbol());
        assert_eq!(amount.volatility_rating(), VolatilityRating::Medium);
        assert_eq!(amount.liquidity_rating(), LiquidityRating::Medium);
    }

    #[test]
    fn test_rich_currency_metadata() {
        let amount = Amount::<RichTestCurrency>::from_major(100);

        // Test custom values
        assert_eq!(amount.currency_name(), "Rich Test Currency");
        assert_eq!(amount.currency_country(), "Test Country");
        assert_eq!(amount.currency_region(), "Test Region");
        assert_eq!(amount.currency_type(), CurrencyType::Fiat);
        assert!(amount.is_major_currency());
        assert!(amount.is_stable_currency());
        assert_eq!(amount.currency_introduced_year(), 2024);
        assert_eq!(amount.currency_iso_number(), 999);
        assert_eq!(amount.thousands_separator(), '.');
        assert_eq!(amount.decimal_separator(), ',');
        assert_eq!(amount.symbol_position(), SymbolPosition::After);
        assert!(amount.space_between_symbol());
        assert_eq!(amount.volatility_rating(), VolatilityRating::Low);
        assert_eq!(amount.liquidity_rating(), LiquidityRating::High);
    }

    #[test]
    fn test_currency_info_formatting() {
        let minimal_amount = Amount::<TestCurrency>::from_major(100);
        let rich_amount = Amount::<RichTestCurrency>::from_major(100);

        // Test info formatting
        assert_eq!(&minimal_amount.currency_info(), " (TEST) -  - Fiat - Minor");
        assert_eq!(
            &rich_amount.currency_info(),
            "Rich Test Currency (RICH) - Test Country - Fiat - Major"
        );
    }
}
