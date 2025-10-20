use crate::Currency;

/// Maker (MKR)
///
/// Maker is the governance token of the MakerDAO protocol, which is responsible
/// for creating and managing the Dai stablecoin. MKR holders can vote on
/// important protocol decisions and risk parameters.
///
/// # Examples
///
/// ```
/// use typed_money::{Amount, Currency, MKR};
///
/// let amount = Amount::<MKR>::from_major(1);
/// assert_eq!(amount.to_major_floor(), 1);
/// assert_eq!(MKR::CODE, "MKR");
/// assert_eq!(MKR::SYMBOL, "MKR");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MKR;

impl Currency for MKR {
    const DECIMALS: u8 = 18;
    const CODE: &'static str = "MKR";
    const SYMBOL: &'static str = "MKR";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Amount;

    #[test]
    fn test_mkr_currency_properties() {
        assert_eq!(MKR::DECIMALS, 18);
        assert_eq!(MKR::CODE, "MKR");
        assert_eq!(MKR::SYMBOL, "MKR");
    }

    #[test]
    fn test_mkr_amount_creation() {
        let amount = Amount::<MKR>::from_major(1);
        assert_eq!(amount.to_major_floor(), 1);
    }

    #[test]
    fn test_mkr_amount_with_wei() {
        let amount = Amount::<MKR>::from_minor(1_000_000_000_000_000_000); // 1.000000000000000000 MKR
        assert_eq!(amount.to_major_floor(), 1);
        assert_eq!(amount.to_minor(), 1_000_000_000_000_000_000);
    }
}
