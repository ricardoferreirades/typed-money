//! Serialization and deserialization examples.
//!
//! This example demonstrates how to serialize and deserialize monetary amounts
//! using serde. This is useful for storing amounts in databases, sending over
//! APIs, or persisting to files.
//!
//! Run with: `cargo run --example serialization --features serde_support`

#[cfg(feature = "serde_support")]
fn main() {
    use serde_json;
    use typed_money::{Amount, BTC, EUR, GBP, JPY, USD};

    println!("=== Serialization Examples ===\n");

    // ========================================
    // Basic Serialization
    // ========================================
    println!("1. BASIC SERIALIZATION");
    println!("----------------------");

    let amount = Amount::<USD>::from_major(123) + Amount::<USD>::from_minor(45);
    println!("Original: {}", amount);

    let json = serde_json::to_string(&amount).unwrap();
    println!("Serialized: {}", json);

    let deserialized: Amount<USD> = serde_json::from_str(&json).unwrap();
    println!("Deserialized: {}", deserialized);

    assert_eq!(amount, deserialized);
    println!("✓ Round-trip successful!\n");

    // ========================================
    // Different Currencies
    // ========================================
    println!("2. MULTIPLE CURRENCIES");
    println!("----------------------");

    let amounts = vec![
        (
            "USD",
            serde_json::to_string(&Amount::<USD>::from_major(100)).unwrap(),
        ),
        (
            "EUR",
            serde_json::to_string(&Amount::<EUR>::from_major(85)).unwrap(),
        ),
        (
            "GBP",
            serde_json::to_string(&Amount::<GBP>::from_major(75)).unwrap(),
        ),
        (
            "JPY",
            serde_json::to_string(&Amount::<JPY>::from_major(11000)).unwrap(),
        ),
        (
            "BTC",
            serde_json::to_string(&Amount::<BTC>::from_minor(100_000_000)).unwrap(),
        ),
    ];

    for (currency, json) in amounts {
        println!("{}: {}", currency, json);
    }

    // ========================================
    // Pretty JSON
    // ========================================
    println!("\n3. PRETTY JSON");
    println!("--------------");

    let amount = Amount::<USD>::from_major(999) + Amount::<USD>::from_minor(99);
    let pretty = serde_json::to_string_pretty(&amount).unwrap();
    println!("{}", pretty);

    // ========================================
    // Struct Serialization
    // ========================================
    println!("\n4. SERIALIZING STRUCTS");
    println!("----------------------");

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct Product {
        name: String,
        price: Amount<USD>,
        stock: u32,
    }

    let product = Product {
        name: "Laptop".to_string(),
        price: Amount::<USD>::from_major(999) + Amount::<USD>::from_minor(99),
        stock: 15,
    };

    let json = serde_json::to_string_pretty(&product).unwrap();
    println!("Product:\n{}", json);

    let deserialized: Product = serde_json::from_str(&json).unwrap();
    println!("\nDeserialized product: {}", deserialized.name);
    println!("  Price: {}", deserialized.price);
    println!("  Stock: {}", deserialized.stock);

    // ========================================
    // Collections
    // ========================================
    println!("\n5. SERIALIZING COLLECTIONS");
    println!("--------------------------");

    let prices = vec![
        Amount::<USD>::from_major(10),
        Amount::<USD>::from_major(20),
        Amount::<USD>::from_major(30),
    ];

    let json = serde_json::to_string(&prices).unwrap();
    println!("Vec<Amount<USD>>: {}", json);

    let deserialized: Vec<Amount<USD>> = serde_json::from_str(&json).unwrap();
    println!("Deserialized {} items", deserialized.len());

    // ========================================
    // Multi-Currency Struct
    // ========================================
    println!("\n6. MULTI-CURRENCY DATA");
    println!("----------------------");

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct Invoice {
        id: u64,
        usd_amount: Amount<USD>,
        eur_amount: Amount<EUR>,
        gbp_amount: Amount<GBP>,
    }

    let invoice = Invoice {
        id: 12345,
        usd_amount: Amount::<USD>::from_major(1000),
        eur_amount: Amount::<EUR>::from_major(850),
        gbp_amount: Amount::<GBP>::from_major(750),
    };

    let json = serde_json::to_string_pretty(&invoice).unwrap();
    println!("Invoice:\n{}", json);

    // ========================================
    // Error Handling in Deserialization
    // ========================================
    println!("\n7. DESERIALIZATION ERRORS");
    println!("-------------------------");

    // Valid JSON
    let valid_json = r#"{"value":"123.45","currency":"USD"}"#;
    match serde_json::from_str::<Amount<USD>>(valid_json) {
        Ok(amount) => println!("✓ Valid: {} → {}", valid_json, amount),
        Err(e) => println!("✗ Error: {}", e),
    }

    // Invalid currency mismatch
    let invalid_currency = r#"{"value":"123.45","currency":"EUR"}"#;
    match serde_json::from_str::<Amount<USD>>(invalid_currency) {
        Ok(amount) => println!("✓ Parsed: {}", amount),
        Err(e) => println!("✗ Currency mismatch: {}", e),
    }

    // Invalid value
    let invalid_value = r#"{"value":"invalid","currency":"USD"}"#;
    match serde_json::from_str::<Amount<USD>>(invalid_value) {
        Ok(amount) => println!("✓ Parsed: {}", amount),
        Err(e) => println!("✗ Invalid value: {}", e),
    }

    // Missing field
    let missing_field = r#"{"value":"123.45"}"#;
    match serde_json::from_str::<Amount<USD>>(missing_field) {
        Ok(amount) => println!("✓ Parsed: {}", amount),
        Err(e) => println!("✗ Missing field: {}", e),
    }

    // ========================================
    // Real-World: API Response
    // ========================================
    println!("\n8. REAL-WORLD: API RESPONSE");
    println!("---------------------------");

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct PaymentResponse {
        transaction_id: String,
        amount: Amount<USD>,
        fee: Amount<USD>,
        status: String,
    }

    // Simulate API response
    let response_json = r#"{
        "transaction_id": "txn_123456",
        "amount": {"value": "100.00", "currency": "USD"},
        "fee": {"value": "2.50", "currency": "USD"},
        "status": "completed"
    }"#;

    match serde_json::from_str::<PaymentResponse>(response_json) {
        Ok(response) => {
            println!("Transaction: {}", response.transaction_id);
            println!("  Amount: {}", response.amount);
            println!("  Fee: {}", response.fee);
            println!("  Status: {}", response.status);

            let net = response.amount - response.fee;
            println!("  Net: {}", net);
        }
        Err(e) => println!("Failed to parse response: {}", e),
    }

    // ========================================
    // Real-World: Saving to File
    // ========================================
    println!("\n9. REAL-WORLD: PERSISTENCE");
    println!("--------------------------");

    #[derive(Debug, serde::Serialize, serde::Deserialize)]
    struct AccountBalance {
        account_id: String,
        usd_balance: Amount<USD>,
        eur_balance: Amount<EUR>,
        last_updated: u64,
    }

    let account = AccountBalance {
        account_id: "ACC_789".to_string(),
        usd_balance: Amount::<USD>::from_major(5000),
        eur_balance: Amount::<EUR>::from_major(4250),
        last_updated: 1700000000,
    };

    let json = serde_json::to_string_pretty(&account).unwrap();
    println!("Account data (ready to save to file):\n{}", json);

    // In production, you would save this to a file:
    // std::fs::write("account.json", json)?;

    // And load it back:
    let loaded: AccountBalance = serde_json::from_str(&json).unwrap();
    println!("\nLoaded account: {}", loaded.account_id);
    println!("  USD: {}", loaded.usd_balance);
    println!("  EUR: {}", loaded.eur_balance);

    println!("\n=== All serialization examples completed! ===");
}

#[cfg(not(feature = "serde_support"))]
fn main() {
    println!("This example requires the 'serde_support' feature.");
    println!("Run with: cargo run --example serialization --features serde_support");
}
