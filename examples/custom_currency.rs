//! Custom currency definition examples.
//!
//! This example demonstrates how to define your own currencies and use them
//! with the typed-money library. This is useful for currencies not included
//! in the library or for custom tokens/units.
//!
//! Run with: `cargo run --example custom_currency`

#![allow(clippy::upper_case_acronyms)]

use typed_money::{Amount, Currency, Rate, RoundingMode};

// ========================================
// Define Custom Fiat Currencies
// ========================================

/// Canadian Dollar
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct CAD;

impl Currency for CAD {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "CAD";
    const SYMBOL: &'static str = "C$";
}

/// Swiss Franc
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct CHF;

impl Currency for CHF {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "CHF";
    const SYMBOL: &'static str = "CHF";
}

/// Australian Dollar
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct AUD;

impl Currency for AUD {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "AUD";
    const SYMBOL: &'static str = "A$";
}

// ========================================
// Define Cryptocurrency
// ========================================

/// Dogecoin - cryptocurrency with 8 decimal places
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct DOGE;

impl Currency for DOGE {
    const DECIMALS: u8 = 8;
    const CODE: &'static str = "DOGE";
    const SYMBOL: &'static str = "Ð";
}

// ========================================
// Define Game/Virtual Currency
// ========================================

/// Gold coins in a video game (no fractional coins)
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct GOLD;

impl Currency for GOLD {
    const DECIMALS: u8 = 0;
    const CODE: &'static str = "GOLD";
    const SYMBOL: &'static str = "⚜";
}

/// Premium gems with fractional amounts (for precise calculations)
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct GEMS;

impl Currency for GEMS {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "GEMS";
    const SYMBOL: &'static str = "💎";
}

// ========================================
// Define Loyalty Points
// ========================================

/// Reward points (with fractional points for precise calculations)
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct POINTS;

impl Currency for POINTS {
    const DECIMALS: u8 = 2;
    const CODE: &'static str = "POINTS";
    const SYMBOL: &'static str = "★";
}

fn main() {
    println!("=== Custom Currency Examples ===\n");

    // ========================================
    // Using Custom Fiat Currencies
    // ========================================
    println!("1. CUSTOM FIAT CURRENCIES");
    println!("-------------------------");

    let cad = Amount::<CAD>::from_major(100);
    let chf = Amount::<CHF>::from_major(85);
    let aud = Amount::<AUD>::from_major(150);

    println!("Canadian Dollar: {}", cad);
    println!("Swiss Franc:     {}", chf);
    println!("Australian Dollar: {}", aud);

    // Arithmetic works the same
    let total_cad = cad + Amount::<CAD>::from_major(50);
    println!("\nC$100 + C$50 = {}", total_cad);

    // ========================================
    // Currency Properties
    // ========================================
    println!("\n2. CURRENCY PROPERTIES");
    println!("----------------------");

    println!(
        "CAD: code={}, symbol={}, decimals={}",
        CAD::CODE,
        CAD::SYMBOL,
        CAD::DECIMALS
    );
    println!(
        "CHF: code={}, symbol={}, decimals={}",
        CHF::CODE,
        CHF::SYMBOL,
        CHF::DECIMALS
    );
    println!(
        "DOGE: code={}, symbol={}, decimals={}",
        DOGE::CODE,
        DOGE::SYMBOL,
        DOGE::DECIMALS
    );
    println!(
        "GOLD: code={}, symbol={}, decimals={}",
        GOLD::CODE,
        GOLD::SYMBOL,
        GOLD::DECIMALS
    );

    // ========================================
    // Converting Between Custom Currencies
    // ========================================
    println!("\n3. CONVERTING CUSTOM CURRENCIES");
    println!("--------------------------------");

    let _cad_amount = Amount::<CAD>::from_major(100);
    let cad_to_aud = Rate::<CAD, AUD>::new(1.08); // 1 CAD = 1.08 AUD

    println!("Example: C$100 at rate {} CAD→AUD", cad_to_aud.value());

    // ========================================
    // Cryptocurrency Example
    // ========================================
    println!("\n4. CRYPTOCURRENCY (DOGECOIN)");
    println!("-----------------------------");

    // Dogecoin with 8 decimal places (like Bitcoin)
    let doge = Amount::<DOGE>::from_minor(100_000_000); // 1 DOGE
    println!("Amount: {} (1 DOGE)", doge);

    let small_amount = Amount::<DOGE>::from_minor(12_345_678); // 0.12345678 DOGE
    println!("Small amount: {} (0.12345678 DOGE)", small_amount);

    let total = doge + small_amount;
    println!("Total: {}", total);

    // ========================================
    // Video Game Currency
    // ========================================
    println!("\n5. VIDEO GAME CURRENCIES");
    println!("------------------------");

    // Gold coins (no fractional amounts)
    let gold = Amount::<GOLD>::from_major(1000);
    println!("Gold coins: {}", gold);

    let loot = Amount::<GOLD>::from_major(150);
    let total_gold = gold + loot;
    println!(
        "After looting: {} (was {}, found {})",
        total_gold, gold, loot
    );

    // Premium gems (with fractional amounts for bonuses)
    let gems = Amount::<GEMS>::from_major(50);
    println!("\nPremium gems: {}", gems);

    // 10% bonus
    let bonus = (gems * 10) / 100;
    println!("Bonus (10%): {}", bonus);

    let total_gems = gems + bonus;
    println!("Total with bonus: {}", total_gems);

    // ========================================
    // Loyalty Points System
    // ========================================
    println!("\n6. LOYALTY POINTS SYSTEM");
    println!("------------------------");

    let purchase = Amount::<POINTS>::from_major(100);
    println!("Purchase: {}", purchase);

    // Earn 5% back in points
    let earn_rate = 5;
    let points_earned = (purchase * earn_rate) / 100;
    println!("Points earned ({}%): {}", earn_rate, points_earned);

    // Accumulate points
    let existing_points = Amount::<POINTS>::from_major(237);
    let new_balance = existing_points + points_earned;
    println!("\nPrevious balance: {}", existing_points);
    println!("New balance: {}", new_balance);

    // Redeem points
    let redemption = Amount::<POINTS>::from_major(50);
    let final_balance = new_balance - redemption;
    println!("\nAfter redeeming {}: {}", redemption, final_balance);

    // ========================================
    // Exchange Rate Between Custom Currencies
    // ========================================
    println!("\n7. EXCHANGE RATES BETWEEN CUSTOM CURRENCIES");
    println!("--------------------------------------------");

    // Define exchange rates
    let cad_to_aud = Rate::<CAD, AUD>::new(1.08); // 1 CAD = 1.08 AUD

    let cad_amount = Amount::<CAD>::from_major(100);
    println!("Canadian Dollars: {}", cad_amount);
    println!("Exchange rate: 1 CAD = {} AUD", cad_to_aud.value());

    let aud_amount = cad_amount.convert(&cad_to_aud);
    println!("Australian Dollars: {}", aud_amount);

    // ========================================
    // Type Safety Still Works
    // ========================================
    println!("\n8. TYPE SAFETY");
    println!("--------------");

    println!("✓ Can add CAD + CAD");
    println!("✓ Can add GOLD + GOLD");
    println!("✗ Cannot add CAD + CHF (compile error!)");
    println!("✗ Cannot add GOLD + GEMS (compile error!)");
    println!("\nCustom currencies have the same type safety as built-in ones!");

    // These would NOT compile:
    // let invalid = cad + chf;      // Compile error: different currencies
    // let invalid2 = gold + gems;   // Compile error: different currencies

    // ========================================
    // Rounding with Custom Currencies
    // ========================================
    println!("\n9. ROUNDING");
    println!("-----------");

    let cad = Amount::<CAD>::from_major(100) / 3; // C$33.333...
    println!("CAD (exact): {}", cad);
    println!("CAD (rounded): {}", cad.round(RoundingMode::HalfUp));

    let doge = Amount::<DOGE>::from_minor(100_000_000) / 3; // 0.33333333 DOGE
    println!("\nDOGE (exact): {}", doge);
    println!("DOGE (rounded): {}", doge.round(RoundingMode::HalfUp));

    let gold = Amount::<GOLD>::from_major(1000) / 3; // 333.333... coins
    println!("\nGOLD (exact): {}", gold);
    println!(
        "GOLD (rounded): {} (no decimals)",
        gold.round(RoundingMode::HalfUp)
    );

    // ========================================
    // Real-World: Multi-Currency Wallet
    // ========================================
    println!("\n10. REAL-WORLD: MULTI-CURRENCY WALLET");
    println!("--------------------------------------");

    println!("Wallet balances:");
    let cad_balance = Amount::<CAD>::from_major(500);
    let aud_balance = Amount::<AUD>::from_major(300);
    let chf_balance = Amount::<CHF>::from_major(200);

    println!("  CAD: {}", cad_balance);
    println!("  AUD: {}", aud_balance);
    println!("  CHF: {}", chf_balance);

    println!("\nTransaction: Spend C$50");
    let new_cad = cad_balance - Amount::<CAD>::from_major(50);
    println!("  New CAD balance: {}", new_cad);

    println!("\nType system ensures you can't accidentally mix currencies!");

    println!("\n=== All custom currency examples completed! ===");
}
