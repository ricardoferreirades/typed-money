//! Currency metadata types and enums.

use std::fmt;

/// Type of currency (Fiat, Cryptocurrency, or Commodity).
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CurrencyType {
    /// Traditional government-issued currency (USD, EUR, GBP, etc.)
    Fiat,
    /// Digital cryptocurrency (BTC, ETH, etc.)
    Cryptocurrency,
    /// Commodity-based currency (XAU, XAG, etc.)
    Commodity,
}

impl fmt::Display for CurrencyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CurrencyType::Fiat => write!(f, "Fiat"),
            CurrencyType::Cryptocurrency => write!(f, "Cryptocurrency"),
            CurrencyType::Commodity => write!(f, "Commodity"),
        }
    }
}

/// Position of currency symbol relative to the amount.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SymbolPosition {
    /// Symbol appears before the amount (e.g., "$100")
    Before,
    /// Symbol appears after the amount (e.g., "100€")
    After,
}

impl fmt::Display for SymbolPosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SymbolPosition::Before => write!(f, "Before"),
            SymbolPosition::After => write!(f, "After"),
        }
    }
}

/// Volatility rating for a currency.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum VolatilityRating {
    /// Low volatility (stable currencies, major fiat)
    Low,
    /// Medium volatility (most fiat currencies)
    Medium,
    /// High volatility (cryptocurrencies, emerging market currencies)
    High,
}

impl fmt::Display for VolatilityRating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VolatilityRating::Low => write!(f, "Low"),
            VolatilityRating::Medium => write!(f, "Medium"),
            VolatilityRating::High => write!(f, "High"),
        }
    }
}

/// Liquidity rating for a currency.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LiquidityRating {
    /// Low liquidity (rarely traded, limited markets)
    Low,
    /// Medium liquidity (regular trading, some market depth)
    Medium,
    /// High liquidity (heavily traded, deep markets)
    High,
}

impl fmt::Display for LiquidityRating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiquidityRating::Low => write!(f, "Low"),
            LiquidityRating::Medium => write!(f, "Medium"),
            LiquidityRating::High => write!(f, "High"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_type_display() {
        assert_eq!(CurrencyType::Fiat.to_string(), "Fiat");
        assert_eq!(CurrencyType::Cryptocurrency.to_string(), "Cryptocurrency");
        assert_eq!(CurrencyType::Commodity.to_string(), "Commodity");
    }

    #[test]
    fn test_symbol_position_display() {
        assert_eq!(SymbolPosition::Before.to_string(), "Before");
        assert_eq!(SymbolPosition::After.to_string(), "After");
    }

    #[test]
    fn test_volatility_rating_display() {
        assert_eq!(VolatilityRating::Low.to_string(), "Low");
        assert_eq!(VolatilityRating::Medium.to_string(), "Medium");
        assert_eq!(VolatilityRating::High.to_string(), "High");
    }

    #[test]
    fn test_liquidity_rating_display() {
        assert_eq!(LiquidityRating::Low.to_string(), "Low");
        assert_eq!(LiquidityRating::Medium.to_string(), "Medium");
        assert_eq!(LiquidityRating::High.to_string(), "High");
    }

    #[test]
    fn test_enum_ordering() {
        // Test that enums can be ordered consistently
        assert!(CurrencyType::Fiat < CurrencyType::Cryptocurrency);
        assert!(CurrencyType::Cryptocurrency < CurrencyType::Commodity);
        
        assert!(SymbolPosition::Before < SymbolPosition::After);
        
        assert!(VolatilityRating::Low < VolatilityRating::Medium);
        assert!(VolatilityRating::Medium < VolatilityRating::High);
        
        assert!(LiquidityRating::Low < LiquidityRating::Medium);
        assert!(LiquidityRating::Medium < LiquidityRating::High);
    }
}
