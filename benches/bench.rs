use criterion::{criterion_group, criterion_main, Criterion};
use maurice::hardware::screen::Screen;

fn criterion_benchmark(c: &mut Criterion) {
    let mut screen = Screen::new();

    c.bench_function("min", |b| b.iter(|| screen.get_pixels()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
