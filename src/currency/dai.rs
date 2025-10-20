use crate::Currency;

/// Dai (DAI)
///
/// Dai is a stablecoin cryptocurrency that aims to maintain a value of $1 USD.
/// It is created and managed by the MakerDAO protocol and is backed by
/// collateral assets rather than fiat currency.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, DAI};
///
/// let amount = Amount::<DAI>::from_major(100);
/// assert_eq!(amount.to_major_floor(), 100);
/// assert_eq!(DAI::CODE, "DAI");
/// assert_eq!(DAI::SYMBOL, "DAI");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DAI;

impl Currency for DAI {
    const DECIMALS: u8 = 18;
    const CODE: &'static str = "DAI";
    const SYMBOL: &'static str = "DAI";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_dai_currency_properties() {
        assert_eq!(DAI::DECIMALS, 18);
        assert_eq!(DAI::CODE, "DAI");
        assert_eq!(DAI::SYMBOL, "DAI");
    }

    #[test]
    fn test_dai_amount_creation() {
        let amount = Amount::<DAI>::from_major(100);
        assert_eq!(amount.to_major_floor(), 100);
    }

    #[test]
    fn test_dai_amount_with_wei() {
        let amount = Amount::<DAI>::from_minor(1_000_000_000_000_000_000); // 1.000000000000000000 DAI
        assert_eq!(amount.to_major_floor(), 1);
        assert_eq!(amount.to_minor(), 1_000_000_000_000_000_000);
    }
}
