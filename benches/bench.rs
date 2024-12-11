use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let screen = Screen::new();
    let mut counter = StatsManager::default();
    for i in 0..1000 {
        counter.publish(i);
    }
    c.bench_function("min", |b| b.iter(|| counter.min()));

    c.bench_function("max", |b| b.iter(|| counter.max()));

    c.bench_function("percentile_90", |b| b.iter(|| counter.percentile_90()));

    c.bench_function("average", |b| b.iter(|| counter.average()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
