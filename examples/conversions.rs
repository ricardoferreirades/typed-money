//! Currency conversion examples.
//!
//! This example demonstrates how to convert between currencies using exchange rates,
//! including rate creation, inverse rates, and rate metadata for auditability.
//!
//! Run with: `cargo run --example conversions`

use typed_money::{Amount, Rate, EUR, GBP, JPY, USD};

fn main() {
    println!("=== Currency Conversion Examples ===\n");

    // ========================================
    // Basic Conversion
    // ========================================
    println!("1. BASIC CONVERSION");
    println!("-------------------");

    let usd_amount = Amount::<USD>::from_major(100);
    println!("Starting amount: {}", usd_amount);

    // Create an exchange rate: 1 USD = 0.85 EUR
    let usd_to_eur = Rate::<USD, EUR>::new(0.85);
    println!("Exchange rate: 1 USD = 0.85 EUR");

    // Convert
    let eur_amount = usd_amount.convert(&usd_to_eur);
    println!("Converted amount: {}", eur_amount);
    println!("(${} × 0.85 = €{})\n", 100, eur_amount.to_major_floor());

    // ========================================
    // Inverse Rates
    // ========================================
    println!("2. INVERSE RATES");
    println!("----------------");

    let usd_to_eur = Rate::<USD, EUR>::new(0.85);
    println!("USD → EUR rate: {}", usd_to_eur.value());

    // Get the inverse rate automatically
    let eur_to_usd = usd_to_eur.inverse();
    println!("EUR → USD rate (inverse): {}", eur_to_usd.value());
    println!("(1 / 0.85 = ~1.176)\n");

    // Converting back
    let original_usd = Amount::<USD>::from_major(100);
    let eur = original_usd.convert(&usd_to_eur);
    let back_to_usd = eur.convert(&eur_to_usd);

    println!("Original: {}", original_usd);
    println!("To EUR:   {}", eur);
    println!("Back to USD: {}", back_to_usd);
    println!("(Should be approximately the same)\n");

    // ========================================
    // Multiple Conversions
    // ========================================
    println!("3. MULTIPLE CONVERSIONS");
    println!("-----------------------");

    let usd = Amount::<USD>::from_major(1000);
    println!("Starting with: {}", usd);

    // Define multiple rates
    let usd_to_eur = Rate::<USD, EUR>::new(0.85);
    let usd_to_gbp = Rate::<USD, GBP>::new(0.73);
    let usd_to_jpy = Rate::<USD, JPY>::new(110.0);

    println!("\nExchange rates:");
    println!("  1 USD = {} EUR", usd_to_eur.value());
    println!("  1 USD = {} GBP", usd_to_gbp.value());
    println!("  1 USD = {} JPY", usd_to_jpy.value());

    println!("\nConversions:");
    let eur = usd.convert(&usd_to_eur);
    let gbp = usd.convert(&usd_to_gbp);
    let jpy = usd.convert(&usd_to_jpy);

    println!("  {} → {}", usd, eur);
    println!("  {} → {}", usd, gbp);
    println!("  {} → {}", usd, jpy);

    // ========================================
    // Chained Conversions
    // ========================================
    println!("\n4. CHAINED CONVERSIONS");
    println!("----------------------");

    println!("Converting USD → EUR → GBP");

    let usd = Amount::<USD>::from_major(100);
    let usd_to_eur = Rate::<USD, EUR>::new(0.85);
    let eur_to_gbp = Rate::<EUR, GBP>::new(0.86);

    println!("Starting: {}", usd);
    let eur = usd.convert(&usd_to_eur);
    println!("After USD→EUR: {}", eur);
    let gbp = eur.convert(&eur_to_gbp);
    println!("After EUR→GBP: {}", gbp);
    println!("(${} × 0.85 × 0.86 = £{})\n", 100, gbp.to_major_floor());

    // ========================================
    // Rate Metadata for Auditability
    // ========================================
    println!("5. RATE METADATA FOR AUDITABILITY");
    println!("----------------------------------");

    // Create a rate with metadata
    let rate = Rate::<USD, EUR>::new(0.85)
        .with_timestamp_unix_secs(1700000000)
        .with_source("ECB"); // European Central Bank

    println!("Rate: {}", rate.value());
    println!("Timestamp: {:?}", rate.timestamp_unix_secs());
    println!("Source: {:?}", rate.source());

    let amount = Amount::<USD>::from_major(1000);
    let converted = amount.convert(&rate);
    println!(
        "\nConverted {} to {} using rate from {:?}",
        amount,
        converted,
        rate.source().unwrap_or("unknown")
    );

    // ========================================
    // Real-World Example: International Payment
    // ========================================
    println!("\n6. REAL-WORLD EXAMPLE: INTERNATIONAL PAYMENT");
    println!("---------------------------------------------");

    // Customer pays in USD
    let payment_usd = Amount::<USD>::from_major(250);
    println!("Customer payment: {}", payment_usd);

    // Current exchange rate (would come from an API in production)
    let current_rate = Rate::<USD, EUR>::new(0.92)
        .with_timestamp_unix_secs(1700000000)
        .with_source("API");

    // Convert to EUR
    let payment_eur = payment_usd.convert(&current_rate);
    println!("Converted to EUR: {}", payment_eur);

    // Calculate fees
    let fee_percentage = 3; // 3% transaction fee
    let fee = (payment_eur * fee_percentage) / 100;
    println!("Transaction fee ({}%): {}", fee_percentage, fee);

    let final_amount = payment_eur - fee;
    println!("Final amount to recipient: {}", final_amount);

    println!("\nTransaction summary:");
    println!("  Original: {}", payment_usd);
    println!(
        "  Rate: {} (from {:?})",
        current_rate.value(),
        current_rate.source().unwrap_or("N/A")
    );
    println!("  Converted: {}", payment_eur);
    println!("  Fee: -{}", fee);
    println!("  Final: {}", final_amount);

    // ========================================
    // Type Safety with Conversions
    // ========================================
    println!("\n7. TYPE SAFETY");
    println!("--------------");

    println!("✓ Can only convert with matching rate types");
    println!("✓ USD amount requires Rate<USD, X>");
    println!("✗ Cannot use Rate<EUR, GBP> on USD amount (compile error!)");
    println!("\nThe type system ensures you always use the correct exchange rate!");

    // This would NOT compile:
    // let usd = Amount::<USD>::from_major(100);
    // let wrong_rate = Rate::<EUR, GBP>::new(0.86);
    // let invalid = usd.convert(&wrong_rate);  // Compile error!

    println!("\n=== All conversion examples completed! ===");
}
