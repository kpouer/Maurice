use criterion::{Criterion, criterion_group, criterion_main};
use maurice_lib::hardware::screen::Screen;

fn criterion_benchmark(c: &mut Criterion) {
    let screen = Screen::new(1);

    c.bench_function("get_pixels1", |b| b.iter(|| screen.get_pixels()));
    let screen = Screen::new(3);

    c.bench_function("get_pixels3", |b| b.iter(|| screen.get_pixels()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
