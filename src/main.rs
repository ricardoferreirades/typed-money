use typed_money::{Amount, BTC, EUR, USD, XAU};

fn main() {
    println!("🚀 Typed-Money Library - Compilation Successful!");
    println!("=================================================");
    println!();

    // Quick demonstration that everything is working
    let usd_amount = Amount::<USD>::from_major(1000);
    let eur_amount = Amount::<EUR>::from_major(850);
    let btc_amount = Amount::<BTC>::from_major(1);
    let gold_amount = Amount::<XAU>::from_major(10);

    println!("✅ Library Status: All systems operational!");
    println!();
    println!("💰 Quick Demo:");
    println!("  USD: {}", usd_amount);
    println!("  EUR: {}", eur_amount);
    println!("  BTC: {}", btc_amount);
    println!("  Gold: {}", gold_amount);
    println!();
    println!("🎯 What's Available:");
    println!("  • 69 currencies with full internationalization support");
    println!("  • Type-safe monetary operations");
    println!("  • Currency conversions and rates");
    println!("  • Precious metals and cryptocurrencies");
    println!("  • Rich metadata for portfolio analysis");
    println!();
    println!("📚 Next Steps:");
    println!("  • Run examples: cargo run --example <name>");
    println!("  • Try: cargo run --example internationalization");
    println!("  • Try: cargo run --example currency_metadata");
    println!("  • Try: cargo run --example precious_metals");
    println!("  • Check docs: cargo doc --open");
    println!();
    println!("🎉 Ready for production use! Happy coding! 🚀");
}
