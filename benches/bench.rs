use criterion::{Criterion, criterion_group, criterion_main};
use maurice_lib::hardware::M6809;
use maurice_lib::hardware::memory::Memory;
use maurice_lib::hardware::screen::Screen;
use maurice_lib::hardware::sound::Sound;

fn criterion_benchmark(c: &mut Criterion) {
    bench_get_pixels(c);
    bench_sound(c);
    bench_draw_led(c);
    bench_dopaint(c);
}

fn bench_dopaint(c: &mut Criterion) {
    let mut screen = Screen::new(1);
    let mut memory = Memory::default();
    c.bench_function("dopaint_ratio1", |b| b.iter(|| screen.dopaint(&mut memory)));

    let mut screen = Screen::new(3);
    let mut memory = Memory::default();
    c.bench_function("dopaint_ratio3", |b| b.iter(|| screen.dopaint(&mut memory)));
}

fn bench_sound(c: &mut Criterion) {
    let mut sound = Sound::default();
    let mut memory = Memory::default();
    let cpu = M6809::M6809::new(&mut memory);
    c.bench_function("play_sound", |b| b.iter(|| sound.play_sound(&cpu)));
}

fn bench_draw_led(c: &mut Criterion) {
    let mut screen = Screen::new(1);
    c.bench_function("draw_led_ratio1", |b| b.iter(|| screen.draw_led()));

    let mut screen = Screen::new(3);
    c.bench_function("draw_led_ratio3", |b| b.iter(|| screen.draw_led()));
}

fn bench_get_pixels(c: &mut Criterion) {
    let screen = Screen::new(1);

    c.bench_function("get_pixels1", |b| b.iter(|| screen.get_pixels()));
    let screen = Screen::new(3);

    c.bench_function("get_pixels3", |b| b.iter(|| screen.get_pixels()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
