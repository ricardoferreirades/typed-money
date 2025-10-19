use criterion::{black_box, criterion_group, criterion_main, Criterion};
use typed_money::{Amount, Rate, USD, EUR, GBP, JPY};

fn bench_currency_conversion_usd_to_eur(c: &mut Criterion) {
    let amount = Amount::<USD>::from_major(100);
    let rate = Rate::<USD, EUR>::new(0.85);
    
    c.bench_function("currency_conversion_usd_to_eur", |b| {
        b.iter(|| black_box(amount.convert(&rate)))
    });
}

fn bench_currency_conversion_eur_to_gbp(c: &mut Criterion) {
    let amount = Amount::<EUR>::from_major(100);
    let rate = Rate::<EUR, GBP>::new(0.87);
    
    c.bench_function("currency_conversion_eur_to_gbp", |b| {
        b.iter(|| black_box(amount.convert(&rate)))
    });
}

fn bench_currency_conversion_jpy_to_usd(c: &mut Criterion) {
    let amount = Amount::<JPY>::from_major(10000);
    let rate = Rate::<JPY, USD>::new(0.0067);
    
    c.bench_function("currency_conversion_jpy_to_usd", |b| {
        b.iter(|| black_box(amount.convert(&rate)))
    });
}

fn bench_rate_creation(c: &mut Criterion) {
    c.bench_function("rate_creation", |b| {
        b.iter(|| black_box(Rate::<USD, EUR>::new(0.85)))
    });
}

fn bench_rate_inverse(c: &mut Criterion) {
    let rate = Rate::<USD, EUR>::new(0.85);
    
    c.bench_function("rate_inverse", |b| {
        b.iter(|| black_box(rate.inverse()))
    });
}

fn bench_rate_value_access(c: &mut Criterion) {
    let rate = Rate::<USD, EUR>::new(0.85);
    
    c.bench_function("rate_value_access", |b| {
        b.iter(|| black_box(rate.value()))
    });
}

fn bench_chained_conversion(c: &mut Criterion) {
    let amount = Amount::<USD>::from_major(100);
    let usd_to_eur = Rate::<USD, EUR>::new(0.85);
    let eur_to_gbp = Rate::<EUR, GBP>::new(0.87);
    
    c.bench_function("chained_conversion", |b| {
        b.iter(|| {
            let eur_amount = amount.convert(&usd_to_eur);
            black_box(eur_amount.convert(&eur_to_gbp))
        })
    });
}

fn bench_conversion_with_metadata(c: &mut Criterion) {
    let amount = Amount::<USD>::from_major(100);
    let rate = Rate::<USD, EUR>::new(0.85)
        .with_timestamp_unix_secs(1_700_000_000)
        .with_source("ECB");
    
    c.bench_function("conversion_with_metadata", |b| {
        b.iter(|| black_box(amount.convert(&rate)))
    });
}

fn bench_rate_metadata_access(c: &mut Criterion) {
    let rate = Rate::<USD, EUR>::new(0.85)
        .with_timestamp_unix_secs(1_700_000_000)
        .with_source("ECB");
    
    c.bench_function("rate_metadata_access", |b| {
        b.iter(|| {
            let _timestamp = rate.timestamp_unix_secs();
            let _source = rate.source();
            black_box((_timestamp, _source))
        })
    });
}

criterion_group!(
    conversion_benches,
    bench_currency_conversion_usd_to_eur,
    bench_currency_conversion_eur_to_gbp,
    bench_currency_conversion_jpy_to_usd,
    bench_rate_creation,
    bench_rate_inverse,
    bench_rate_value_access,
    bench_chained_conversion,
    bench_conversion_with_metadata,
    bench_rate_metadata_access
);

criterion_main!(conversion_benches);
