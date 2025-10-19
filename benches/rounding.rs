use criterion::{black_box, criterion_group, criterion_main, Criterion};
use typed_money::{Amount, RoundingMode, EUR, USD};

fn bench_rounding_half_up(c: &mut Criterion) {
    let amount = Amount::<USD>::from_major(100) + Amount::<USD>::from_minor(5); // 100.05

    c.bench_function("rounding_half_up", |b| {
        b.iter(|| black_box(amount.round(RoundingMode::HalfUp)))
    });
}

fn bench_rounding_half_down(c: &mut Criterion) {
    let amount = Amount::<USD>::from_major(100) + Amount::<USD>::from_minor(5); // 100.05

    c.bench_function("rounding_half_down", |b| {
        b.iter(|| black_box(amount.round(RoundingMode::HalfDown)))
    });
}

fn bench_rounding_half_even(c: &mut Criterion) {
    let amount = Amount::<USD>::from_major(100) + Amount::<USD>::from_minor(5); // 100.05

    c.bench_function("rounding_half_even", |b| {
        b.iter(|| black_box(amount.round(RoundingMode::HalfEven)))
    });
}

fn bench_rounding_up(c: &mut Criterion) {
    let amount = Amount::<USD>::from_major(100) + Amount::<USD>::from_minor(1); // 100.01

    c.bench_function("rounding_up", |b| {
        b.iter(|| black_box(amount.round(RoundingMode::Up)))
    });
}

fn bench_rounding_down(c: &mut Criterion) {
    let amount = Amount::<USD>::from_major(100) + Amount::<USD>::from_minor(1); // 100.01

    c.bench_function("rounding_down", |b| {
        b.iter(|| black_box(amount.round(RoundingMode::Down)))
    });
}

fn bench_rounding_ceiling(c: &mut Criterion) {
    let amount = Amount::<USD>::from_major(100) + Amount::<USD>::from_minor(1); // 100.01

    c.bench_function("rounding_ceiling", |b| {
        b.iter(|| black_box(amount.round(RoundingMode::Ceiling)))
    });
}

fn bench_rounding_floor(c: &mut Criterion) {
    let amount = Amount::<USD>::from_major(100) + Amount::<USD>::from_minor(1); // 100.01

    c.bench_function("rounding_floor", |b| {
        b.iter(|| black_box(amount.round(RoundingMode::Floor)))
    });
}

fn bench_rounding_negative_numbers(c: &mut Criterion) {
    let amount = Amount::<USD>::from_major(-100) - Amount::<USD>::from_minor(5); // -100.05

    c.bench_function("rounding_negative_half_up", |b| {
        b.iter(|| black_box(amount.round(RoundingMode::HalfUp)))
    });
}

fn bench_rounding_zero_decimal_currency(c: &mut Criterion) {
    let amount = Amount::<EUR>::from_major(100) + Amount::<EUR>::from_minor(5); // 100.05 (but JPY has 0 decimals)

    c.bench_function("rounding_zero_decimal_currency", |b| {
        b.iter(|| black_box(amount.round(RoundingMode::HalfUp)))
    });
}

fn bench_rounding_large_precision(c: &mut Criterion) {
    // Create amount with high precision
    let amount = Amount::<USD>::from_major(100) + Amount::<USD>::from_minor(123456); // 100.123456

    c.bench_function("rounding_large_precision", |b| {
        b.iter(|| black_box(amount.round(RoundingMode::HalfUp)))
    });
}

fn bench_rounding_edge_cases(c: &mut Criterion) {
    let zero = Amount::<USD>::from_major(0);
    let max_amount = Amount::<USD>::from_major(i64::MAX);

    c.bench_function("rounding_edge_cases", |b| {
        b.iter(|| {
            let _zero_rounded = black_box(zero.round(RoundingMode::HalfUp));
            let _max_rounded = black_box(max_amount.round(RoundingMode::HalfUp));
        })
    });
}

fn bench_rounding_mode_enum_creation(c: &mut Criterion) {
    c.bench_function("rounding_mode_enum_creation", |b| {
        b.iter(|| {
            let _modes = [
                black_box(RoundingMode::HalfUp),
                black_box(RoundingMode::HalfDown),
                black_box(RoundingMode::HalfEven),
                black_box(RoundingMode::Up),
                black_box(RoundingMode::Down),
                black_box(RoundingMode::Ceiling),
                black_box(RoundingMode::Floor),
            ];
        })
    });
}

criterion_group!(
    rounding_benches,
    bench_rounding_half_up,
    bench_rounding_half_down,
    bench_rounding_half_even,
    bench_rounding_up,
    bench_rounding_down,
    bench_rounding_ceiling,
    bench_rounding_floor,
    bench_rounding_negative_numbers,
    bench_rounding_zero_decimal_currency,
    bench_rounding_large_precision,
    bench_rounding_edge_cases,
    bench_rounding_mode_enum_creation
);

criterion_main!(rounding_benches);
