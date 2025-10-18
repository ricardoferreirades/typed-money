//! Rounding modes demonstration.
//!
//! This example demonstrates all 7 rounding modes available in the library,
//! showing when to use each one and how they behave with different values.
//!
//! Run with: `cargo run --example rounding`

use typed_money::{Amount, RoundingMode, JPY, USD};

fn main() {
    println!("=== Rounding Modes Demonstration ===\n");

    // ========================================
    // The Seven Rounding Modes
    // ========================================
    println!("1. ALL ROUNDING MODES");
    println!("---------------------");

    let amount = Amount::<USD>::from_major(10) / 3; // $3.333...
    println!("Original amount: {} ($3.33333...)\n", amount);

    println!("Rounding modes:");
    println!(
        "  HalfUp:    {} (round 0.5 up)",
        amount.round(RoundingMode::HalfUp)
    );
    println!(
        "  HalfDown:  {} (round 0.5 down)",
        amount.round(RoundingMode::HalfDown)
    );
    println!(
        "  HalfEven:  {} (banker's rounding)",
        amount.round(RoundingMode::HalfEven)
    );
    println!(
        "  Up:        {} (away from zero)",
        amount.round(RoundingMode::Up)
    );
    println!(
        "  Down:      {} (towards zero)",
        amount.round(RoundingMode::Down)
    );
    println!(
        "  Floor:     {} (towards -∞)",
        amount.round(RoundingMode::Floor)
    );
    println!(
        "  Ceiling:   {} (towards +∞)",
        amount.round(RoundingMode::Ceiling)
    );

    // ========================================
    // HalfUp - Most Common
    // ========================================
    println!("\n2. HALF UP (Most Common in Retail)");
    println!("-----------------------------------");

    let examples = [
        (Amount::<USD>::from_minor(125), "1.25"), // Midpoint
        (Amount::<USD>::from_minor(124), "1.24"),
        (Amount::<USD>::from_minor(126), "1.26"),
        (Amount::<USD>::from_minor(335), "3.35"), // Midpoint
    ];

    println!("Round to nearest dollar (HalfUp):");
    for (amount, desc) in examples {
        let rounded = amount.round(RoundingMode::HalfUp);
        println!("  ${} → {}", desc, rounded);
    }

    // ========================================
    // HalfEven - Banker's Rounding
    // ========================================
    println!("\n3. HALF EVEN (Banker's Rounding)");
    println!("---------------------------------");
    println!("Minimizes bias in repeated operations\n");

    let midpoints = [
        (Amount::<USD>::from_minor(150), "1.50"), // → $2 (even)
        (Amount::<USD>::from_minor(250), "2.50"), // → $2 (even)
        (Amount::<USD>::from_minor(350), "3.50"), // → $4 (even)
        (Amount::<USD>::from_minor(450), "4.50"), // → $4 (even)
    ];

    println!("Rounding midpoints (HalfEven):");
    for (amount, desc) in midpoints {
        let rounded = amount.round(RoundingMode::HalfEven);
        println!("  ${} → {} (rounds to nearest even)", desc, rounded);
    }

    // ========================================
    // Floor vs Ceiling
    // ========================================
    println!("\n4. FLOOR vs CEILING");
    println!("-------------------");

    let positive = Amount::<USD>::from_minor(349); // $3.49
    let negative = Amount::<USD>::from_major(-3) - Amount::<USD>::from_minor(49); // -$3.49

    println!("Positive value: ${:.2}", 3.49);
    println!(
        "  Floor:   {} (towards -∞)",
        positive.round(RoundingMode::Floor)
    );
    println!(
        "  Ceiling: {} (towards +∞)",
        positive.round(RoundingMode::Ceiling)
    );

    println!("\nNegative value: ${}", -3.49);
    println!(
        "  Floor:   {} (more negative)",
        negative.round(RoundingMode::Floor)
    );
    println!(
        "  Ceiling: {} (less negative)",
        negative.round(RoundingMode::Ceiling)
    );

    // ========================================
    // Up vs Down
    // ========================================
    println!("\n5. UP vs DOWN");
    println!("-------------");

    let positive = Amount::<USD>::from_minor(349); // $3.49
    let negative = Amount::<USD>::from_major(-3) - Amount::<USD>::from_minor(49); // -$3.49

    println!("Positive value: ${:.2}", 3.49);
    println!(
        "  Up:   {} (away from zero)",
        positive.round(RoundingMode::Up)
    );
    println!(
        "  Down: {} (towards zero)",
        positive.round(RoundingMode::Down)
    );

    println!("\nNegative value: ${}", -3.49);
    println!(
        "  Up:   {} (away from zero)",
        negative.round(RoundingMode::Up)
    );
    println!(
        "  Down: {} (towards zero)",
        negative.round(RoundingMode::Down)
    );

    // ========================================
    // Currency-Specific Rounding
    // ========================================
    println!("\n6. CURRENCY-SPECIFIC ROUNDING");
    println!("------------------------------");

    // USD has 2 decimals
    let usd = Amount::<USD>::from_major(10) / 3;
    println!("USD (2 decimals): {}", usd);
    println!("  Rounded: {}", usd.round(RoundingMode::HalfUp));

    // JPY has 0 decimals
    let jpy = Amount::<JPY>::from_major(1000) / 3;
    println!("\nJPY (0 decimals): {}", jpy);
    println!(
        "  Rounded: {} (no decimal places)",
        jpy.round(RoundingMode::HalfUp)
    );

    // ========================================
    // Use Cases
    // ========================================
    println!("\n7. WHEN TO USE EACH MODE");
    println!("------------------------");

    println!("HalfUp:");
    println!("  ✓ Retail/consumer applications");
    println!("  ✓ Most intuitive for end users");

    println!("\nHalfEven (Banker's Rounding):");
    println!("  ✓ Financial calculations");
    println!("  ✓ Statistical analysis");
    println!("  ✓ Minimizes cumulative errors");

    println!("\nFloor:");
    println!("  ✓ Conservative estimates (costs)");
    println!("  ✓ Always round down to benefit customer");

    println!("\nCeiling:");
    println!("  ✓ Conservative estimates (buffers)");
    println!("  ✓ Ensure sufficient funds");

    println!("\nDown (Truncate):");
    println!("  ✓ Fast rounding when direction doesn't matter");
    println!("  ✓ Tax calculations (some jurisdictions)");

    println!("\nUp:");
    println!("  ✓ Always round up for safety margin");
    println!("  ✓ Ensure coverage of costs");

    // ========================================
    // Real-World: Tax Calculation
    // ========================================
    println!("\n8. REAL-WORLD: TAX CALCULATION");
    println!("-------------------------------");

    let subtotal = Amount::<USD>::from_major(99) + Amount::<USD>::from_minor(99);
    let tax_rate = 7; // 7%

    println!("Subtotal: {}", subtotal);
    println!("Tax rate: {}%", tax_rate);

    let tax_exact = (subtotal * tax_rate) / 100;
    println!("\nExact tax: {}", tax_exact);

    println!("\nRounded tax:");
    println!(
        "  HalfUp:  {} (standard)",
        tax_exact.round(RoundingMode::HalfUp)
    );
    println!(
        "  Floor:   {} (customer-friendly)",
        tax_exact.round(RoundingMode::Floor)
    );
    println!(
        "  Ceiling: {} (conservative)",
        tax_exact.round(RoundingMode::Ceiling)
    );

    let tax_rounded = tax_exact.round(RoundingMode::HalfUp);
    let total = subtotal + tax_rounded;
    println!("\nFinal total: {}", total);

    // ========================================
    // Real-World: Interest Calculation
    // ========================================
    println!("\n9. REAL-WORLD: INTEREST CALCULATION");
    println!("------------------------------------");

    let principal = Amount::<USD>::from_major(10000);
    let annual_rate = 5; // 5% annual
    let months = 6;

    println!("Principal: {}", principal);
    println!("Annual rate: {}%", annual_rate);
    println!("Period: {} months", months);

    // Calculate monthly interest
    let interest_exact = (principal * annual_rate * months) / (100 * 12);
    println!("\nExact interest: {}", interest_exact);

    // Using banker's rounding for financial accuracy
    let interest_rounded = interest_exact.round(RoundingMode::HalfEven);
    println!("Rounded (HalfEven): {}", interest_rounded);

    let final_amount = principal + interest_rounded;
    println!("Final amount: {}", final_amount);

    // ========================================
    // Demonstrating Banker's Rounding Benefits
    // ========================================
    println!("\n10. BANKER'S ROUNDING BENEFITS");
    println!("------------------------------");

    println!("Summing many midpoint values:\n");

    // Create amounts that are exactly at midpoints
    let amounts = vec![
        Amount::<USD>::from_minor(150), // $1.50
        Amount::<USD>::from_minor(250), // $2.50
        Amount::<USD>::from_minor(350), // $3.50
        Amount::<USD>::from_minor(450), // $4.50
    ];

    let mut sum_halfup = Amount::<USD>::from_major(0);
    let mut sum_halfeven = Amount::<USD>::from_major(0);

    for amount in &amounts {
        sum_halfup = sum_halfup + amount.round(RoundingMode::HalfUp);
        sum_halfeven = sum_halfeven + amount.round(RoundingMode::HalfEven);
    }

    println!(
        "Sum with HalfUp:   {} (always rounds up midpoints)",
        sum_halfup
    );
    println!("Sum with HalfEven: {} (balanced rounding)", sum_halfeven);
    println!("\nHalfEven minimizes bias when summing many midpoint values!");

    println!("\n=== All rounding examples completed! ===");
}
