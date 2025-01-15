pub(crate) mod color;

use crate::hardware::memory::Memory;
use crate::hardware::screen::color::{Color, COLOR_DEPTH, PALETTE};
use crate::int;
use crate::raw_image::RawImage;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

pub const WIDTH: usize = 320;
pub const HEIGHT: usize = 200;

pub const DEFAULT_PIXEL_SIZE: usize = 3;

#[derive(Debug)]
pub struct Screen {
    pub(crate) mouse_clic: bool,
    pub(crate) mouse_x: int,
    pub(crate) mouse_y: int,
    pixels: [&'static Color; WIDTH * HEIGHT],
    filter: bool,
    pub(crate) led: int,
    pub(crate) show_led: int,
    ratio: usize,
    tmp_lines: Arc<Mutex<Vec<u8>>>,
}

impl Screen {
    pub fn new(ratio: usize) -> Self {
        Screen {
            mouse_clic: false,
            mouse_x: -1,
            mouse_y: -1,
            pixels: [&PALETTE[0]; WIDTH * HEIGHT],
            filter: false,
            led: 0,
            show_led: 0,
            ratio,
            tmp_lines: Arc::new(Mutex::new(vec![
                0;
                HEIGHT * WIDTH * ratio * ratio * COLOR_DEPTH
            ])),
        }
    }

    pub(crate) fn new_size(&mut self, new_size: crate::dimension::Dimension) {
        let x_ratio = new_size.width / WIDTH;
        let y_ratio = new_size.height / HEIGHT;
        self.set_ratio(std::cmp::min(x_ratio, y_ratio));
    }

    fn set_ratio(&mut self, mut ratio: usize) {
        if ratio == 0 {
            ratio = 1;
        }
        self.ratio = ratio;
        self.tmp_lines = Arc::new(Mutex::new(vec![0; HEIGHT * WIDTH * ratio * ratio * ratio]));
    }

    pub(crate) fn paint(&mut self, mem: &mut Memory) {
        if self.show_led > 0 {
            //todo : restore this
            // self.show_led -= 1;
            // let color = if self.led != 0 {
            //     Color::from_rgb(255., 0., 0.)
            // } else {
            //     Color::from_rgb(0., 0., 0.)
            // };
            // let rectangle: Rectangle<f32> = Rectangle::new(Vector2::new(WIDTH as f32 - 16., 0.), Vector2::new(16., 8.));
            //  graphics.draw_rectangle(rectangle, color);
        }
        self.dopaint(mem);
    }

    pub fn get_pixels(&mut self) -> RawImage {
        let pixel_size = self.ratio;
        let line_length = WIDTH * pixel_size;
        {
            let mut tmp_lines = self.tmp_lines.lock().unwrap();
            tmp_lines
                .par_chunks_mut(line_length * pixel_size * COLOR_DEPTH)
                .enumerate()
                .for_each(|(line_index, line_buffer)| {
                    // here line_buffer contains the pixels of pixel_size lines
                    let range = ..line_length * COLOR_DEPTH;
                    let line_target = &mut line_buffer[range];
                    let line_source = &self.pixels[line_index * WIDTH..(line_index + 1) * WIDTH];
                    Self::fill_line(line_source, line_target, pixel_size);
                    for yy in 1..pixel_size {
                        line_buffer.copy_within(range, line_length * COLOR_DEPTH * yy)
                    }
                });
        }

        RawImage::new_with_data(line_length, HEIGHT * pixel_size, self.tmp_lines.clone())
    }

    fn fill_line(pixel_source: &[&'static Color], line_buffer: &mut [u8], pixel_size: usize) {
        let mut x_offset = 0;

        for color in pixel_source {
            for _ in 0..pixel_size {
                line_buffer[x_offset..x_offset + 3].copy_from_slice(*color);
                x_offset += 3;
            }
        }
    }

    pub(crate) fn dopaint(&mut self, mem: &mut Memory) {
        let mut i = 0;

        for y in 0..HEIGHT {
            let offset: usize = y * WIDTH;
            if !mem.is_dirty(y) {
                i += 40;
            } else {
                let mut x: usize = 0;
                for _ in 0..40 {
                    let col = mem.COLOR(i);
                    let c2 = (col & 0x0F) as usize;
                    let c1 = (col >> 4) as usize;
                    let cc2 = &PALETTE[c1];
                    let cc1 = &PALETTE[c2];

                    let pt = mem.POINT(i);
                    const PATTERN: [int; 8] = [0x80, 0x40, 0x20, 0x10, 0x08, 0x04, 0x02, 0x01];
                    for v in PATTERN {
                        if (v & pt) != 0 {
                            self.pixels[x + offset] = cc2;
                        } else {
                            self.pixels[x + offset] = cc1;
                        }
                        x += 1;
                    }
                    i += 1;
                }
            }
        }
    }
}
