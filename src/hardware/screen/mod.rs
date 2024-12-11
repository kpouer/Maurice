mod color;

use crate::dimension::Dimension;
use crate::hardware::memory::Memory;
use crate::hardware::screen::color::{Color, PALETTE};
use crate::int;
use crate::raw_image::RawImage;
use rayon::prelude::*;
use std::cmp;

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
    tmp_lines: Vec<Vec<u8>>,
}

impl Screen {
    pub fn new() -> Self {
        Screen {
            mouse_clic: false,
            mouse_x: -1,
            mouse_y: -1,
            pixels: [&PALETTE[0]; WIDTH * HEIGHT],
            filter: false,
            led: 0,
            show_led: 0,
            ratio: DEFAULT_PIXEL_SIZE,
            tmp_lines: vec![vec![0; WIDTH * DEFAULT_PIXEL_SIZE * 3]; HEIGHT],
        }
    }

    pub(crate) fn new_size(&mut self, new_size: Dimension) {
        let x_ratio = new_size.width / WIDTH;
        let y_ratio = new_size.height / HEIGHT;
        self.set_ratio(cmp::min(x_ratio, y_ratio));
    }

    fn set_ratio(&mut self, mut ratio: usize) {
        if ratio == 0 {
            ratio = 1;
        }
        self.ratio = ratio;
        self.tmp_lines = vec![vec![0; WIDTH * ratio * 3]; HEIGHT];
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

    pub(crate) fn get_pixels(&mut self) -> RawImage {
        let pixel_size = self.ratio;

        self.tmp_lines
            .par_iter_mut()
            .enumerate()
            .for_each(|(y, line_buffer)| {
                Self::fill_line(y, &self.pixels, line_buffer, pixel_size);
            });

        let mut raw_image = RawImage::new(WIDTH * pixel_size, HEIGHT * pixel_size);
        let line_size = WIDTH * pixel_size * 3;
        raw_image
            .data
            .par_chunks_mut(line_size)
            .enumerate()
            .for_each(|(y, line_buffer)| {
                let real_y = y / pixel_size;
                let line_data = &self.tmp_lines[real_y];
                line_buffer.copy_from_slice(line_data);
            });

        raw_image
    }

    fn fill_line(
        y: usize,
        pixels: &[&'static Color; WIDTH * HEIGHT],
        line_buffer: &mut [u8],
        pixel_size: usize,
    ) {
        let mut x_offset = 0;

        for x in 0..WIDTH {
            let color = pixels[x + y * WIDTH];
            for _ in 0..pixel_size {
                line_buffer[x_offset..x_offset + 3].copy_from_slice(color);
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
