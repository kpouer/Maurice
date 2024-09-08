mod color;

use crate::domension::Dimension;
use crate::hardware::memory::Memory;
use crate::hardware::screen::color::{Color, PALETTE};
use crate::int;
use crate::raw_image::RawImage;
use std::cmp;

pub(crate) const WIDTH: usize = 320;
pub(crate) const HEIGHT: usize = 200;
pub(crate) const DEFAULT_PIXEL_SIZE: usize = 3;

#[derive(Debug)]
pub(crate) struct Screen {
    pub(crate) mouse_clic: bool,
    pub(crate) mouse_x: int,
    pub(crate) mouse_y: int,
    pixels: [&'static Color; WIDTH * HEIGHT],
    filter: bool,
    pub(crate) led: int,
    pub(crate) show_led: int,
    ratio: usize,
}

impl Screen {
    pub(crate) fn new() -> Self {
        Screen {
            mouse_clic: false,
            mouse_x: -1,
            mouse_y: -1,
            pixels: [&PALETTE[0]; WIDTH * HEIGHT],
            filter: false,
            led: 0,
            show_led: 0,
            ratio: DEFAULT_PIXEL_SIZE,
        }
    }

    pub(crate) fn new_size(&mut self, new_size: Dimension) {
        let x_ratio = new_size.width / WIDTH;
        let y_ratio = new_size.height / HEIGHT;
        self.ratio = cmp::min(x_ratio, y_ratio);
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

    pub(crate) fn get_pixels(&self) -> RawImage {
        let pixel_size = self.ratio;
        let mut raw_image = RawImage::new(WIDTH * pixel_size, HEIGHT * pixel_size);

        let buffer = &mut raw_image.data;
        let mut index = 0;
        let line_size = WIDTH * pixel_size * 3;
        let mut line_buffer = vec![0; line_size];
        for y in 0..HEIGHT {
            self.get_line(y, &mut line_buffer, pixel_size);

            for _ in 0..pixel_size {
                buffer[index..index + line_size].copy_from_slice(&line_buffer);
                index += line_size;
            }
        }

        raw_image
    }

    fn get_line(&self, y: usize, line_buffer: &mut [u8], pixel_size: usize) {
        let mut x_offset = 0;
        for x in 0..WIDTH {
            let color = self.pixels[x + y * WIDTH].as_ref();
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
