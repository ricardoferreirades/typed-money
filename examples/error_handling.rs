//! Error handling examples.
//!
//! This example demonstrates how to handle errors in the typed-money library,
//! including parsing errors, precision errors, invalid rates, and recovery strategies.
//!
//! Run with: `cargo run --example error_handling`

use std::str::FromStr;
use typed_money::{Amount, MoneyError, MoneyResult, Rate, RoundingMode, EUR, USD};

fn main() {
    println!("=== Error Handling Examples ===\n");

    // ========================================
    // Parse Errors
    // ========================================
    println!("1. PARSING ERRORS");
    println!("-----------------");

    // Valid parsing
    match Amount::<USD>::from_str("123.45") {
        Ok(amount) => println!("✓ Parsed: {}", amount),
        Err(e) => println!("✗ Error: {}", e),
    }

    // Invalid formats
    let invalid_inputs = vec!["invalid", "12.34.56", "abc123", "", "  "];

    println!("\nInvalid inputs:");
    for input in invalid_inputs {
        match Amount::<USD>::from_str(input) {
            Ok(amount) => println!("  '{}' → {}", input, amount),
            Err(MoneyError::ParseError { input: inp, .. }) => {
                println!("  '{}' → Parse error (input: '{}')", input, inp);
            }
            Err(e) => println!("  '{}' → Error: {}", input, e),
        }
    }

    // ========================================
    // Precision Errors
    // ========================================
    println!("\n2. PRECISION ERRORS");
    println!("-------------------");

    // Division creates excess precision
    let amount = Amount::<USD>::from_major(10) / 3; // $3.333...
    println!("After division: {}", amount);
    println!("Decimal places: {}", amount.precision());
    println!("Currency expects: {}", Amount::<USD>::currency_precision());

    // Check for precision issues
    match amount.check_precision() {
        Ok(()) => println!("✓ Precision OK"),
        Err(MoneyError::PrecisionError {
            expected,
            actual,
            suggestion,
            ..
        }) => {
            println!("✗ Precision error:");
            println!("  Expected: {} decimals", expected);
            println!("  Actual: {} decimals", actual);
            println!("  Suggestion: {}", suggestion);

            // Recover by normalizing
            let normalized = amount.normalize();
            println!("\n✓ After normalizing: {}", normalized);
            println!("  Precision OK: {}", normalized.check_precision().is_ok());
        }
        Err(e) => println!("✗ Unexpected error: {}", e),
    }

    // ========================================
    // Invalid Rate Errors
    // ========================================
    println!("\n3. INVALID RATE ERRORS");
    println!("----------------------");

    // Valid rate
    match Rate::<USD, EUR>::try_new(0.85) {
        Ok(rate) => println!("✓ Valid rate: {}", rate.value()),
        Err(e) => println!("✗ Error: {}", e),
    }

    // Invalid rates
    let invalid_rates = vec![
        ("negative", -1.0),
        ("zero", 0.0),
        ("NaN", f64::NAN),
        ("infinity", f64::INFINITY),
    ];

    println!("\nInvalid rates:");
    for (desc, value) in invalid_rates {
        match Rate::<USD, EUR>::try_new(value) {
            Ok(rate) => println!(
                "  {} ({}) → Unexpected success: {}",
                desc,
                value,
                rate.value()
            ),
            Err(MoneyError::InvalidRate { reason, .. }) => {
                println!("  {} ({}) → {}", desc, value, reason);
            }
            Err(e) => println!("  {} ({}) → Error: {}", desc, value, e),
        }
    }

    // ========================================
    // Error Suggestions
    // ========================================
    println!("\n4. ERROR RECOVERY SUGGESTIONS");
    println!("------------------------------");

    let errors = vec![
        MoneyError::PrecisionError {
            currency: "USD",
            expected: 2,
            actual: 5,
            suggestion: "Use normalize() or round() to adjust precision".to_string(),
        },
        MoneyError::InvalidRate {
            value: "-1.0".to_string(),
            reason: "Rate must be positive".to_string(),
        },
        MoneyError::ParseError {
            input: "abc".to_string(),
            expected_currency: Some("USD"),
            reason: "Invalid number format".to_string(),
        },
    ];

    for error in errors {
        println!("\nError: {}", error);
        println!("Suggestion: {}", error.suggestion());
        if let Some(currency) = error.currency() {
            println!("Currency: {}", currency);
        }
    }

    // ========================================
    // Safe Division
    // ========================================
    println!("\n5. SAFE DIVISION");
    println!("----------------");

    let amount = Amount::<USD>::from_major(100);

    // Division by zero is handled
    let divisor = 0;
    println!("Dividing {} by {}...", amount, divisor);

    if divisor == 0 {
        println!("✗ Cannot divide by zero!");
        println!("  Handled before attempting division");
    } else {
        let result = amount / divisor;
        println!("✓ Result: {}", result);
    }

    // Safe division with non-zero
    let safe_divisor = 3;
    let result = amount / safe_divisor;
    println!("\nDividing {} by {}: {}", amount, safe_divisor, result);

    // ========================================
    // Pattern Matching on Errors
    // ========================================
    println!("\n6. PATTERN MATCHING");
    println!("-------------------");

    fn parse_and_handle(input: &str) -> MoneyResult<Amount<USD>> {
        Amount::<USD>::from_str(input)
    }

    let inputs = vec!["123.45", "invalid", "999.99"];

    for input in inputs {
        print!("Parsing '{}': ", input);

        match parse_and_handle(input) {
            Ok(amount) => {
                println!("✓ Success: {}", amount);
            }
            Err(MoneyError::ParseError { input, reason, .. }) => {
                println!("✗ Parse error");
                println!("  Input: '{}'", input);
                println!("  Reason: {}", reason);
            }
            Err(e) => {
                println!("✗ Other error: {}", e);
            }
        }
    }

    // ========================================
    // Error Propagation with ?
    // ========================================
    println!("\n7. ERROR PROPAGATION");
    println!("--------------------");

    fn calculate_total(price_str: &str, quantity: i64) -> MoneyResult<Amount<USD>> {
        let price = Amount::<USD>::from_str(price_str)?;
        let total = price * quantity;
        total.check_precision()?;
        Ok(total)
    }

    match calculate_total("29.99", 3) {
        Ok(total) => println!("✓ Total: {}", total),
        Err(e) => println!("✗ Error: {}", e),
    }

    match calculate_total("invalid", 3) {
        Ok(total) => println!("✓ Total: {}", total),
        Err(e) => println!("✗ Error: {}", e),
    }

    // ========================================
    // Recovering from Errors
    // ========================================
    println!("\n8. ERROR RECOVERY");
    println!("-----------------");

    fn parse_with_fallback(input: &str, fallback: Amount<USD>) -> Amount<USD> {
        Amount::<USD>::from_str(input).unwrap_or(fallback)
    }

    let fallback = Amount::<USD>::from_major(0);

    let inputs = vec!["123.45", "invalid", "67.89"];
    for input in inputs {
        let amount = parse_with_fallback(input, fallback);
        println!("'{}' → {}", input, amount);
    }

    // ========================================
    // Validation Before Operations
    // ========================================
    println!("\n9. VALIDATION");
    println!("-------------");

    fn safe_divide(amount: Amount<USD>, divisor: i64) -> MoneyResult<Amount<USD>> {
        if divisor == 0 {
            return Err(MoneyError::InvalidAmount {
                currency: Some("USD"),
                reason: "Cannot divide by zero".to_string(),
            });
        }

        let result = amount / divisor;

        // Check precision after division
        result.check_precision()?;

        Ok(result)
    }

    let amount = Amount::<USD>::from_major(100);

    match safe_divide(amount, 0) {
        Ok(result) => println!("  Division by 0: {}", result),
        Err(e) => println!("  Division by 0: Error - {}", e),
    }

    match safe_divide(amount, 3) {
        Ok(result) => println!("  Division by 3: {}", result),
        Err(e) => println!("  Division by 3: Error - {}", e),
    }

    // ========================================
    // Real-World: Parsing User Input
    // ========================================
    println!("\n10. REAL-WORLD: PARSING USER INPUT");
    println!("-----------------------------------");

    fn process_payment(amount_str: &str) -> MoneyResult<String> {
        // Parse the amount
        let amount = Amount::<USD>::from_str(amount_str)?;

        // Validate it's positive
        if amount.to_minor() <= 0 {
            return Err(MoneyError::InvalidAmount {
                currency: Some("USD"),
                reason: "Payment amount must be positive".to_string(),
            });
        }

        // Process the payment
        let fee = (amount * 3) / 100; // 3% fee
        let total = amount + fee;

        Ok(format!(
            "Payment: {}, Fee: {}, Total: {}",
            amount, fee, total
        ))
    }

    let test_inputs = vec!["100.00", "0", "-50", "abc", "250.50"];

    for input in test_inputs {
        println!("\nProcessing payment: '{}'", input);
        match process_payment(input) {
            Ok(summary) => println!("  ✓ {}", summary),
            Err(e) => {
                println!("  ✗ Error: {}", e);
                let suggestion = e.suggestion();
                println!("    Hint: {}", suggestion);
            }
        }
    }

    // ========================================
    // Rounding to Fix Precision
    // ========================================
    println!("\n11. FIXING PRECISION WITH ROUNDING");
    println!("-----------------------------------");

    let amount = Amount::<USD>::from_major(10) / 3; // Creates excess precision

    println!("Original: {}", amount);
    println!(
        "Precision check: {}",
        if amount.check_precision().is_ok() {
            "✓ OK"
        } else {
            "✗ Excess precision"
        }
    );

    // Fix with rounding
    let rounded = amount.round(RoundingMode::HalfUp);
    println!("\nRounded: {}", rounded);
    println!(
        "Precision check: {}",
        if rounded.check_precision().is_ok() {
            "✓ OK"
        } else {
            "✗ Excess precision"
        }
    );

    // Or normalize (uses banker's rounding)
    let normalized = amount.normalize();
    println!("\nNormalized: {}", normalized);
    println!(
        "Precision check: {}",
        if normalized.check_precision().is_ok() {
            "✓ OK"
        } else {
            "✗ Excess precision"
        }
    );

    println!("\n=== All error handling examples completed! ===");
}
