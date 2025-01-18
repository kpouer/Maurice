pub(crate) mod color;

use crate::hardware::memory::Memory;
use crate::hardware::screen::color::{COLOR_DEPTH, PALETTE};
use crate::int;
use crate::raw_image::RawImage;
use std::sync::{Arc, Mutex};

pub const WIDTH: usize = 320;
pub const HEIGHT: usize = 200;

pub const DEFAULT_PIXEL_SIZE: usize = 3;

#[derive(Debug)]
pub struct Screen {
    pub(crate) mouse_clic: bool,
    pub(crate) mouse_x: int,
    pub(crate) mouse_y: int,
    pixels: Arc<Mutex<Vec<u8>>>,
    pub(crate) led: u8,
    pub(crate) show_led: u8,
    ratio: usize,
}

impl Screen {
    pub fn new(ratio: usize) -> Self {
        Screen {
            mouse_clic: false,
            mouse_x: -1,
            mouse_y: -1,
            pixels: Arc::new(Mutex::new(vec![
                0;
                WIDTH * ratio * HEIGHT * ratio * COLOR_DEPTH
            ])),
            led: 0,
            show_led: 0,
            ratio,
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
        self.pixels = Arc::new(Mutex::new(vec![
            0;
            WIDTH * ratio * HEIGHT * ratio * COLOR_DEPTH
        ]));
    }

    pub(crate) fn paint(&mut self, mem: &mut Memory) {
        self.dopaint(mem);
        if self.show_led > 0 {
            self.show_led -= 1;
            let sec = if self.led != 0 {
                [0xFF, 0x00, 0x00]
            } else {
                [0x00, 0x00, 0x00]
            };
            let mut line = Vec::with_capacity(16 * self.ratio * sec.len());
            for _ in 0..16 * self.ratio {
                line.extend(sec);
            }
            let mut pixels = self.pixels.lock().unwrap();
            for y in 1..17 {
                let start = y * WIDTH * self.ratio * self.ratio * COLOR_DEPTH - line.len();
                let slice = &mut pixels[start..start + line.len()];
                slice.copy_from_slice(&line);
            }
        }
    }

    pub fn get_pixels(&mut self) -> RawImage {
        RawImage::new_with_data(WIDTH * self.ratio, HEIGHT * self.ratio, self.pixels.clone())
    }

    pub(crate) fn dopaint(&mut self, mem: &mut Memory) {
        let mut i = 0;

        let mut pixels = self.pixels.lock().unwrap();
        for y in 0..HEIGHT {
            let offset = y * WIDTH * self.ratio * self.ratio * COLOR_DEPTH;
            if !mem.is_dirty(y) {
                i += 40;
            } else {
                let mut x = 0;
                for _ in 0..40 {
                    let col = mem.COLOR(i);
                    let c2 = (col & 0x0F) as usize;
                    let c1 = (col >> 4) as usize;
                    let cc2 = &PALETTE[c1];
                    let cc1 = &PALETTE[c2];

                    let pt = mem.POINT(i);
                    const PATTERN: [int; 8] = [0x80, 0x40, 0x20, 0x10, 0x08, 0x04, 0x02, 0x01];
                    for v in PATTERN {
                        for _ in 0..self.ratio {
                            let range_start = x * COLOR_DEPTH + offset;
                            let pixel_range = range_start..range_start + COLOR_DEPTH;
                            if (v & pt) != 0 {
                                pixels[pixel_range].copy_from_slice(cc2);
                            } else {
                                pixels[pixel_range].copy_from_slice(cc1);
                            }
                            x += 1;
                        }
                    }
                    i += 1;
                }
            }
            for a in 1..self.ratio {
                pixels.copy_within(
                    offset..offset + WIDTH * COLOR_DEPTH * self.ratio,
                    offset + WIDTH * self.ratio * COLOR_DEPTH * a,
                );
            }
        }
    }
}
