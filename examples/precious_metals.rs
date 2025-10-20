//! Example demonstrating precious metals and base metals usage.

use typed_money::{Amount, Currency, XAG, XAL, XAU, XCU, XDI, XNI, XPD, XPT, XZN};

fn main() {
    println!("=== Precious Metals Examples ===");

    // Gold (XAU) - troy ounces
    let gold = Amount::<XAU>::from_major(1);
    println!("Gold: {} ({} troy ounces)", gold, gold.to_major_floor());

    // Silver (XAG) - troy ounces
    let silver = Amount::<XAG>::from_major(10);
    println!(
        "Silver: {} ({} troy ounces)",
        silver,
        silver.to_major_floor()
    );

    // Platinum (XPT) - troy ounces
    let platinum = Amount::<XPT>::from_major(2);
    println!(
        "Platinum: {} ({} troy ounces)",
        platinum,
        platinum.to_major_floor()
    );

    // Palladium (XPD) - troy ounces
    let palladium = Amount::<XPD>::from_major(5);
    println!(
        "Palladium: {} ({} troy ounces)",
        palladium,
        palladium.to_major_floor()
    );

    // Diamond (XDI) - carats
    let diamond = Amount::<XDI>::from_major(3);
    println!("Diamond: {} ({} carats)", diamond, diamond.to_major_floor());

    println!("\n=== Base Metals Examples ===");

    // Copper (XCU) - metric tons
    let copper = Amount::<XCU>::from_major(100);
    println!(
        "Copper: {} ({} metric tons)",
        copper,
        copper.to_major_floor()
    );

    // Aluminum (XAL) - metric tons
    let aluminum = Amount::<XAL>::from_major(50);
    println!(
        "Aluminum: {} ({} metric tons)",
        aluminum,
        aluminum.to_major_floor()
    );

    // Zinc (XZN) - metric tons
    let zinc = Amount::<XZN>::from_major(25);
    println!("Zinc: {} ({} metric tons)", zinc, zinc.to_major_floor());

    // Nickel (XNI) - metric tons
    let nickel = Amount::<XNI>::from_major(15);
    println!(
        "Nickel: {} ({} metric tons)",
        nickel,
        nickel.to_major_floor()
    );

    println!("\n=== Currency Properties ===");
    println!(
        "Gold (XAU): {} decimals, code: {}, symbol: {}",
        XAU::DECIMALS,
        XAU::CODE,
        XAU::SYMBOL
    );
    println!(
        "Silver (XAG): {} decimals, code: {}, symbol: {}",
        XAG::DECIMALS,
        XAG::CODE,
        XAG::SYMBOL
    );
    println!(
        "Diamond (XDI): {} decimals, code: {}, symbol: {}",
        XDI::DECIMALS,
        XDI::CODE,
        XDI::SYMBOL
    );
    println!(
        "Copper (XCU): {} decimals, code: {}, symbol: {}",
        XCU::DECIMALS,
        XCU::CODE,
        XCU::SYMBOL
    );

    println!("\n=== Arithmetic Operations ===");

    // Test arithmetic with precious metals
    let gold1 = Amount::<XAU>::from_major(1);
    let gold2 = Amount::<XAU>::from_major(2);
    let total_gold = gold1 + gold2;
    println!("Gold addition: {} + {} = {}", gold1, gold2, total_gold);

    // Test scalar multiplication
    let silver_amount = Amount::<XAG>::from_major(5);
    let multiplied_silver = silver_amount * 3;
    println!(
        "Silver multiplication: {} * 3 = {}",
        silver_amount, multiplied_silver
    );

    println!("\n=== Precision Testing ===");

    // Test 4 decimal precision
    let precise_gold = Amount::<XAU>::from_minor(12345); // 1.2345 troy ounces
    println!(
        "Precise gold: {} (minor units: {})",
        precise_gold,
        precise_gold.to_minor()
    );

    let precise_copper = Amount::<XCU>::from_minor(56789); // 5.6789 metric tons
    println!(
        "Precise copper: {} (minor units: {})",
        precise_copper,
        precise_copper.to_minor()
    );
}
