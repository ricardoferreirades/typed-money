//! Global Currencies Example
//!
//! This example demonstrates the comprehensive currency support in typed-money,
//! showcasing currencies from all major regions around the world.

use typed_money::{
    Amount,
    Currency,
    Rate,
    RoundingMode,
    AED,
    ARS,
    AUD,
    // American Currencies
    BRL,
    BTC,
    // Major Fiat Currencies
    CAD,
    CHF,
    CLP,
    // Asian Currencies
    CNY,
    CZK,
    DKK,
    EGP,
    ETH,
    EUR,
    GBP,
    HKD,
    HUF,
    ILS,
    INR,
    JPY,
    KRW,
    MXN,
    NOK,
    NZD,
    PLN,
    SAR,
    // European Currencies
    SEK,
    SGD,
    TRY,
    TWD,
    // Core currencies
    USD,
    // African/Middle Eastern Currencies
    ZAR,
};

fn main() {
    println!("=== Typed Money - Global Currencies Example ===\n");

    // 1. MAJOR FIAT CURRENCIES
    println!("1. MAJOR FIAT CURRENCIES");
    println!("------------------------");
    let usd = Amount::<USD>::from_major(100);
    let cad = Amount::<CAD>::from_major(135);
    let chf = Amount::<CHF>::from_major(90);
    let aud = Amount::<AUD>::from_major(150);
    let nzd = Amount::<NZD>::from_major(160);

    println!("USD: {} ({} decimals)", usd, USD::DECIMALS);
    println!("CAD: {} ({} decimals)", cad, CAD::DECIMALS);
    println!("CHF: {} ({} decimals)", chf, CHF::DECIMALS);
    println!("AUD: {} ({} decimals)", aud, AUD::DECIMALS);
    println!("NZD: {} ({} decimals)", nzd, NZD::DECIMALS);
    println!();

    // 2. ASIAN CURRENCIES
    println!("2. ASIAN CURRENCIES");
    println!("-------------------");
    let cny = Amount::<CNY>::from_major(720);
    let krw = Amount::<KRW>::from_major(130_000);
    let sgd = Amount::<SGD>::from_major(135);
    let hkd = Amount::<HKD>::from_major(780);
    let twd = Amount::<TWD>::from_major(3100);
    let inr = Amount::<INR>::from_major(8300);

    println!("CNY: {} ({} decimals)", cny, CNY::DECIMALS);
    println!("KRW: {} ({} decimals)", krw, KRW::DECIMALS);
    println!("SGD: {} ({} decimals)", sgd, SGD::DECIMALS);
    println!("HKD: {} ({} decimals)", hkd, HKD::DECIMALS);
    println!("TWD: {} ({} decimals)", twd, TWD::DECIMALS);
    println!("INR: {} ({} decimals)", inr, INR::DECIMALS);
    println!();

    // 3. EUROPEAN CURRENCIES
    println!("3. EUROPEAN CURRENCIES");
    println!("----------------------");
    let eur = Amount::<EUR>::from_major(85);
    let gbp = Amount::<GBP>::from_major(75);
    let sek = Amount::<SEK>::from_major(1050);
    let nok = Amount::<NOK>::from_major(1100);
    let dkk = Amount::<DKK>::from_major(650);
    let pln = Amount::<PLN>::from_major(400);
    let czk = Amount::<CZK>::from_major(2300);
    let huf = Amount::<HUF>::from_major(36_000);

    println!("EUR: {} ({} decimals)", eur, EUR::DECIMALS);
    println!("GBP: {} ({} decimals)", gbp, GBP::DECIMALS);
    println!("SEK: {} ({} decimals)", sek, SEK::DECIMALS);
    println!("NOK: {} ({} decimals)", nok, NOK::DECIMALS);
    println!("DKK: {} ({} decimals)", dkk, DKK::DECIMALS);
    println!("PLN: {} ({} decimals)", pln, PLN::DECIMALS);
    println!("CZK: {} ({} decimals)", czk, CZK::DECIMALS);
    println!("HUF: {} ({} decimals)", huf, HUF::DECIMALS);
    println!();

    // 4. AMERICAN CURRENCIES
    println!("4. AMERICAN CURRENCIES");
    println!("----------------------");
    let brl = Amount::<BRL>::from_major(500);
    let mxn = Amount::<MXN>::from_major(1700);
    let ars = Amount::<ARS>::from_major(85_000);
    let clp = Amount::<CLP>::from_major(90_000);

    println!("BRL: {} ({} decimals)", brl, BRL::DECIMALS);
    println!("MXN: {} ({} decimals)", mxn, MXN::DECIMALS);
    println!("ARS: {} ({} decimals)", ars, ARS::DECIMALS);
    println!("CLP: {} ({} decimals)", clp, CLP::DECIMALS);
    println!();

    // 5. AFRICAN/MIDDLE EASTERN CURRENCIES
    println!("5. AFRICAN/MIDDLE EASTERN CURRENCIES");
    println!("------------------------------------");
    let zar = Amount::<ZAR>::from_major(1800);
    let egp = Amount::<EGP>::from_major(3100);
    let aed = Amount::<AED>::from_major(370);
    let sar = Amount::<SAR>::from_major(375);
    let ils = Amount::<ILS>::from_major(360);
    let try_currency = Amount::<TRY>::from_major(3000);

    println!("ZAR: {} ({} decimals)", zar, ZAR::DECIMALS);
    println!("EGP: {} ({} decimals)", egp, EGP::DECIMALS);
    println!("AED: {} ({} decimals)", aed, AED::DECIMALS);
    println!("SAR: {} ({} decimals)", sar, SAR::DECIMALS);
    println!("ILS: {} ({} decimals)", ils, ILS::DECIMALS);
    println!("TRY: {} ({} decimals)", try_currency, TRY::DECIMALS);
    println!();

    // 6. CRYPTOCURRENCIES
    println!("6. CRYPTOCURRENCIES");
    println!("-------------------");
    let btc = Amount::<BTC>::from_minor(100_000_000); // 1.00000000 BTC
    let eth = Amount::<ETH>::from_minor(1_000_000_000_000_000_000); // 1.000000000000000000 ETH

    println!("BTC: {} ({} decimals)", btc, BTC::DECIMALS);
    println!("ETH: {} ({} decimals)", eth, ETH::DECIMALS);
    println!();

    // 7. CURRENCY CONVERSION EXAMPLES
    println!("7. CURRENCY CONVERSION EXAMPLES");
    println!("-------------------------------");

    // USD to EUR
    let usd_to_eur = Rate::<USD, EUR>::new(0.85)
        .with_timestamp_unix_secs(1_700_000_000)
        .with_source("ECB");
    let eur_converted = usd.convert(&usd_to_eur);
    println!(
        "USD to EUR: {} → {} (rate: {})",
        usd,
        eur_converted,
        usd_to_eur.value()
    );

    // EUR to GBP
    let eur_to_gbp = Rate::<EUR, GBP>::new(0.87);
    let gbp_converted = eur_converted.convert(&eur_to_gbp);
    println!(
        "EUR to GBP: {} → {} (rate: {})",
        eur_converted,
        gbp_converted,
        eur_to_gbp.value()
    );

    // USD to JPY
    let usd_to_jpy = Rate::<USD, JPY>::new(150.0);
    let jpy_converted = usd.convert(&usd_to_jpy);
    println!(
        "USD to JPY: {} → {} (rate: {})",
        usd,
        jpy_converted,
        usd_to_jpy.value()
    );
    println!();

    // 8. ROUNDING EXAMPLES
    println!("8. ROUNDING EXAMPLES");
    println!("--------------------");
    let usd_amount = Amount::<USD>::from_minor(12345); // $123.45
    println!("Original: {}", usd_amount);
    println!("Half Up: {}", usd_amount.round(RoundingMode::HalfUp));
    println!("Half Down: {}", usd_amount.round(RoundingMode::HalfDown));
    println!("Half Even: {}", usd_amount.round(RoundingMode::HalfEven));
    println!("Floor: {}", usd_amount.round(RoundingMode::Floor));
    println!("Ceiling: {}", usd_amount.round(RoundingMode::Ceiling));
    println!();

    // 9. CURRENCY METADATA
    println!("9. CURRENCY METADATA");
    println!("--------------------");
    let currencies = [
        ("USD", USD::CODE, USD::SYMBOL, USD::DECIMALS),
        ("EUR", EUR::CODE, EUR::SYMBOL, EUR::DECIMALS),
        ("JPY", JPY::CODE, JPY::SYMBOL, JPY::DECIMALS),
        ("CNY", CNY::CODE, CNY::SYMBOL, CNY::DECIMALS),
        ("BRL", BRL::CODE, BRL::SYMBOL, BRL::DECIMALS),
        ("AED", AED::CODE, AED::SYMBOL, AED::DECIMALS),
    ];

    for (name, code, symbol, decimals) in currencies {
        println!("{}: {} {} ({} decimals)", name, symbol, code, decimals);
    }
    println!();

    // 10. TYPE SAFETY DEMONSTRATION
    println!("10. TYPE SAFETY DEMONSTRATION");
    println!("-----------------------------");
    println!("✓ USD + USD = Valid");
    println!("✓ EUR + EUR = Valid");
    println!("✓ CAD + CAD = Valid");
    println!("✗ USD + EUR = Compile Error!");
    println!("✗ CAD + JPY = Compile Error!");
    println!("✗ BRL + TRY = Compile Error!");
    println!();

    println!("=== Global Currencies Example Completed! ===");
    println!("Total currencies supported: 30+");
    println!("Regions covered: North America, Europe, Asia, South America, Africa, Middle East");
    println!("Type safety: 100% compile-time currency mixing prevention");
}
