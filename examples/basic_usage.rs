//! Basic usage examples for the typed-money library.
//!
//! This example demonstrates the fundamental operations you can perform
//! with monetary amounts, including creation, arithmetic, and comparisons.
//!
//! Run with: `cargo run --example basic_usage`

use typed_money::{Amount, Currency, EUR, GBP, JPY, USD};

fn main() {
    println!("=== Typed Money - Basic Usage Examples ===\n");

    // ========================================
    // Creating Amounts
    // ========================================
    println!("1. CREATING AMOUNTS");
    println!("-------------------");

    // From major units (dollars, euros, etc.)
    let price = Amount::<USD>::from_major(100);
    println!("From major units: {} (${}.00)", price, 100);

    // From minor units (cents, pence, etc.)
    let tax = Amount::<USD>::from_minor(850); // $8.50
    println!("From minor units: {} (850 cents = $8.50)", tax);

    // Zero-decimal currency (Japanese Yen)
    let yen = Amount::<JPY>::from_major(1000);
    println!("Zero decimals (JPY): {} (¥1000, no cents)", yen);

    // ========================================
    // Arithmetic Operations
    // ========================================
    println!("\n2. ARITHMETIC OPERATIONS");
    println!("------------------------");

    let a = Amount::<USD>::from_major(100);
    let b = Amount::<USD>::from_major(50);

    // Addition
    let sum = a + b;
    println!("Addition: ${} + ${} = {}", 100, 50, sum);

    // Subtraction
    let diff = a - b;
    println!("Subtraction: ${} - ${} = {}", 100, 50, diff);

    // Multiplication by scalar
    let doubled = a * 2;
    println!("Multiplication: ${} × 2 = {}", 100, doubled);

    // Division by scalar
    let halved = a / 2;
    println!("Division: ${} ÷ 2 = {}", 100, halved);

    // Division creates precise decimals
    let thirds = Amount::<USD>::from_major(10) / 3;
    println!(
        "Precise division: $10 ÷ 3 = {} (${:.4})",
        thirds,
        10.0 / 3.0
    );

    // ========================================
    // Comparisons
    // ========================================
    println!("\n3. COMPARISONS");
    println!("--------------");

    let amount1 = Amount::<USD>::from_major(100);
    let amount2 = Amount::<USD>::from_major(50);
    let amount3 = Amount::<USD>::from_major(100);

    println!("${} == ${}? {}", 100, 100, amount1 == amount3);
    println!("${} != ${}? {}", 100, 50, amount1 != amount2);
    println!("${} > ${}? {}", 100, 50, amount1 > amount2);
    println!("${} < ${}? {}", 50, 100, amount2 < amount1);
    println!("${} >= ${}? {}", 100, 100, amount1 >= amount3);
    println!("${} <= ${}? {}", 50, 100, amount2 <= amount1);

    // ========================================
    // Working with Different Currencies
    // ========================================
    println!("\n4. MULTIPLE CURRENCIES");
    println!("----------------------");

    let usd = Amount::<USD>::from_major(100);
    let eur = Amount::<EUR>::from_major(85);
    let gbp = Amount::<GBP>::from_major(75);
    let jpy = Amount::<JPY>::from_major(11000);

    println!("USD: {}", usd);
    println!("EUR: {}", eur);
    println!("GBP: {}", gbp);
    println!("JPY: {}", jpy);

    // ========================================
    // Currency Properties
    // ========================================
    println!("\n5. CURRENCY PROPERTIES");
    println!("----------------------");

    println!(
        "USD: {} decimals, code: {}, symbol: {}",
        USD::DECIMALS,
        USD::CODE,
        USD::SYMBOL
    );
    println!(
        "EUR: {} decimals, code: {}, symbol: {}",
        EUR::DECIMALS,
        EUR::CODE,
        EUR::SYMBOL
    );
    println!(
        "JPY: {} decimals, code: {}, symbol: {}",
        JPY::DECIMALS,
        JPY::CODE,
        JPY::SYMBOL
    );

    // ========================================
    // Extracting Values
    // ========================================
    println!("\n6. EXTRACTING VALUES");
    println!("--------------------");

    let amount = Amount::<USD>::from_major(123);
    println!("Amount: {}", amount);
    println!("  Major units (floor): ${}", amount.to_major_floor());
    println!("  Minor units: {} cents", amount.to_minor());

    let amount_with_cents = Amount::<USD>::from_minor(12345); // $123.45
    println!("Amount: {}", amount_with_cents);
    println!(
        "  Major units (floor): ${}",
        amount_with_cents.to_major_floor()
    );
    println!(
        "  Major units (half up): ${}",
        amount_with_cents.to_major_half_up()
    );
    println!("  Minor units: {} cents", amount_with_cents.to_minor());

    // ========================================
    // Type Safety
    // ========================================
    println!("\n7. TYPE SAFETY");
    println!("--------------");

    println!("✓ Can add USD + USD");
    println!("✓ Can compare USD with USD");
    println!("✗ Cannot add USD + EUR (compile error!)");
    println!("✗ Cannot compare USD with EUR (compile error!)");
    println!("\nType safety prevents currency mixing at compile time!");

    // The following would NOT compile:
    // let usd = Amount::<USD>::from_major(100);
    // let eur = Amount::<EUR>::from_major(85);
    // let invalid = usd + eur;  // Compile error: type mismatch!

    // ========================================
    // Real-World Example: Shopping Cart
    // ========================================
    println!("\n8. REAL-WORLD EXAMPLE: SHOPPING CART");
    println!("-------------------------------------");

    let item1_price = Amount::<USD>::from_major(29); // $29.99
    let item1_price = item1_price + Amount::<USD>::from_minor(99);

    let item2_price = Amount::<USD>::from_major(15); // $15.50
    let item2_price = item2_price + Amount::<USD>::from_minor(50);

    let item3_price = Amount::<USD>::from_major(42); // $42.00

    println!("Item 1: {}", item1_price);
    println!("Item 2: {}", item2_price);
    println!("Item 3: {}", item3_price);

    let subtotal = item1_price + item2_price + item3_price;
    println!("\nSubtotal: {}", subtotal);

    let tax_rate = 8; // 8%
    let tax = (subtotal * tax_rate) / 100;
    println!("Tax ({}%): {}", tax_rate, tax);

    let total = subtotal + tax;
    println!("Total: {}", total);
    println!("Total in cents: {} cents", total.to_minor());

    println!("\n=== All examples completed successfully! ===");
}
