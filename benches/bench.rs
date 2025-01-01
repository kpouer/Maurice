use criterion::{criterion_group, criterion_main, Criterion};
use maurice::hardware::screen::Screen;

fn criterion_benchmark(c: &mut Criterion) {
    let mut screen = Screen::new(1);

    c.bench_function("get_pixels1", |b| b.iter(|| screen.get_pixels()));
    let mut screen = Screen::new(3);

    c.bench_function("get_pixels3", |b| b.iter(|| screen.get_pixels()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
