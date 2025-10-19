use criterion::{black_box, criterion_group, criterion_main, Criterion};
use typed_money::{Amount, USD};

fn bench_amount_addition(c: &mut Criterion) {
    let amount1 = Amount::<USD>::from_major(100);
    let amount2 = Amount::<USD>::from_major(200);
    
    c.bench_function("amount_addition", |b| {
        b.iter(|| black_box(amount1 + amount2))
    });
}

fn bench_amount_subtraction(c: &mut Criterion) {
    let amount1 = Amount::<USD>::from_major(300);
    let amount2 = Amount::<USD>::from_major(100);
    
    c.bench_function("amount_subtraction", |b| {
        b.iter(|| black_box(amount1 - amount2))
    });
}

fn bench_scalar_multiplication(c: &mut Criterion) {
    let amount = Amount::<USD>::from_major(100);
    let multiplier = 2i64;
    
    c.bench_function("scalar_multiplication", |b| {
        b.iter(|| black_box(amount * multiplier))
    });
}

fn bench_scalar_division(c: &mut Criterion) {
    let amount = Amount::<USD>::from_major(200);
    let divisor = 2i64;
    
    c.bench_function("scalar_division", |b| {
        b.iter(|| black_box(amount / divisor))
    });
}

fn bench_amount_equality(c: &mut Criterion) {
    let amount1 = Amount::<USD>::from_major(100);
    let amount2 = Amount::<USD>::from_major(100);
    
    c.bench_function("amount_equality", |b| {
        b.iter(|| black_box(amount1 == amount2))
    });
}

fn bench_amount_ordering(c: &mut Criterion) {
    let amount1 = Amount::<USD>::from_major(100);
    let amount2 = Amount::<USD>::from_major(200);
    
    c.bench_function("amount_ordering", |b| {
        b.iter(|| black_box(amount1 < amount2))
    });
}

fn bench_from_major_constructor(c: &mut Criterion) {
    c.bench_function("from_major_constructor", |b| {
        b.iter(|| black_box(Amount::<USD>::from_major(100)))
    });
}

fn bench_from_minor_constructor(c: &mut Criterion) {
    c.bench_function("from_minor_constructor", |b| {
        b.iter(|| black_box(Amount::<USD>::from_minor(10000)))
    });
}

fn bench_to_major_conversion(c: &mut Criterion) {
    let amount = Amount::<USD>::from_major(100);
    
    c.bench_function("to_major_floor", |b| {
        b.iter(|| black_box(amount.to_major_floor()))
    });
}

fn bench_to_minor_conversion(c: &mut Criterion) {
    let amount = Amount::<USD>::from_major(100);
    
    c.bench_function("to_minor", |b| {
        b.iter(|| black_box(amount.to_minor()))
    });
}

criterion_group!(
    arithmetic_benches,
    bench_amount_addition,
    bench_amount_subtraction,
    bench_scalar_multiplication,
    bench_scalar_division,
    bench_amount_equality,
    bench_amount_ordering,
    bench_from_major_constructor,
    bench_from_minor_constructor,
    bench_to_major_conversion,
    bench_to_minor_conversion
);

criterion_main!(arithmetic_benches);
