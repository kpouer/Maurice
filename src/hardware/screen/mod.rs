pub(crate) mod color;

use crate::hardware::memory::Memory;
use crate::hardware::screen::color::{COLOR_DEPTH, PALETTE};
use crate::int;
use crate::raw_image::RawImage;

pub const WIDTH: usize = 320;
pub const HEIGHT: usize = 200;

pub const DEFAULT_PIXEL_SIZE: usize = 3;

#[derive(Debug)]
pub struct Screen {
    pub(crate) mouse_clic: bool,
    pub(crate) mouse_x: int,
    pub(crate) mouse_y: int,
    pixels: Vec<u8>,
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
            pixels: vec![0; WIDTH * ratio * HEIGHT * ratio * COLOR_DEPTH],
            led: 0,
            show_led: 0,
            ratio,
        }
    }

    fn set_ratio(&mut self, mut ratio: usize) {
        if ratio == 0 {
            ratio = 1;
        }
        self.ratio = ratio;
        self.pixels = vec![0; WIDTH * ratio * HEIGHT * ratio * COLOR_DEPTH];
    }

    pub(crate) fn paint(&mut self, mem: &mut Memory) {
        self.dopaint(mem);
        if self.show_led > 0 {
            self.show_led -= 1;
            self.draw_led();
        }
    }

    pub fn draw_led(&mut self) {
        let led_width_pixels = 16 * self.ratio;
        let led_width_bytes = led_width_pixels * COLOR_DEPTH;
        let row_stride = WIDTH * self.ratio * self.ratio * COLOR_DEPTH;

        let first_row_end = row_stride;
        let first_row_start = row_stride - led_width_bytes;

        if self.led != 0 {
                let (first_row, _) = self.pixels.split_at_mut(first_row_end);
                let target_chunk = &mut first_row[first_row_start..first_row_end];

                for pixel in target_chunk.chunks_exact_mut(COLOR_DEPTH) {
                    pixel[0] = 0xFF;
                    pixel[1] = 0x00;
                    pixel[2] = 0x00;
            }
        } else {
            self.pixels[first_row_start..first_row_end].fill(0x00);
        }

        let source_range = first_row_start..first_row_end;
        for y in 2..17 {
            let dest_start = y * row_stride - led_width_bytes;
            self.pixels.copy_within(source_range.clone(), dest_start);
        }
    }

    pub fn get_pixels(&self) -> RawImage<'_> {
        RawImage::new_with_data(WIDTH * self.ratio, HEIGHT * self.ratio, &self.pixels)
    }

    pub(crate) fn dopaint(&mut self, mem: &mut Memory) {
        let mut i = 0;

        let pixels = &mut self.pixels;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_draw_led_with_led_on() {
        let mut screen = Screen::new(1);
        screen.led = 1;
        screen.draw_led();

        // Check that the LED area (top-right corner, lines 1-16, last 16 pixels) is red
        for y in 1..17 {
            let line_offset = y * WIDTH * COLOR_DEPTH;
            let led_start = line_offset - 16 * COLOR_DEPTH;
            for x in 0..16 {
                let pixel_offset = led_start + x * COLOR_DEPTH;
                assert_eq!(screen.pixels[pixel_offset], 0xFF, "Red channel should be 0xFF at y={}, x={}", y, x);
                assert_eq!(screen.pixels[pixel_offset + 1], 0x00, "Green channel should be 0x00 at y={}, x={}", y, x);
                assert_eq!(screen.pixels[pixel_offset + 2], 0x00, "Blue channel should be 0x00 at y={}, x={}", y, x);
            }
        }
    }

    #[test]
    fn test_draw_led_with_led_off() {
        let mut screen = Screen::new(1);
        screen.led = 0;
        screen.draw_led();

        // Check that the LED area is black
        for y in 1..17 {
            let line_offset = y * WIDTH * COLOR_DEPTH;
            let led_start = line_offset - 16 * COLOR_DEPTH;
            for x in 0..16 {
                let pixel_offset = led_start + x * COLOR_DEPTH;
                assert_eq!(screen.pixels[pixel_offset], 0x00, "Red channel should be 0x00 at y={}, x={}", y, x);
                assert_eq!(screen.pixels[pixel_offset + 1], 0x00, "Green channel should be 0x00 at y={}, x={}", y, x);
                assert_eq!(screen.pixels[pixel_offset + 2], 0x00, "Blue channel should be 0x00 at y={}, x={}", y, x);
            }
        }
    }

    #[test]
    fn test_draw_led_with_ratio_3() {
        let mut screen = Screen::new(3);
        screen.led = 1;
        screen.draw_led();

        // Check that the LED area is properly scaled (16*3 = 48 pixels wide)
        for y in 1..17 {
            let line_offset = y * WIDTH * 3 * 3 * COLOR_DEPTH;
            let led_start = line_offset - 16 * 3 * COLOR_DEPTH;
            for x in 0..(16 * 3) {
                let pixel_offset = led_start + x * COLOR_DEPTH;
                assert_eq!(screen.pixels[pixel_offset], 0xFF, "Red channel should be 0xFF at y={}, x={}", y, x);
                assert_eq!(screen.pixels[pixel_offset + 1], 0x00, "Green channel should be 0x00 at y={}, x={}", y, x);
                assert_eq!(screen.pixels[pixel_offset + 2], 0x00, "Blue channel should be 0x00 at y={}, x={}", y, x);
            }
        }
    }

    #[test]
    fn test_draw_led_does_not_affect_other_pixels() {
        let mut screen = Screen::new(1);

        // Set some pixels outside the LED area
        let test_offset = 20 * WIDTH * COLOR_DEPTH;
        screen.pixels[test_offset] = 0xAA;
        screen.pixels[test_offset + 1] = 0xBB;
        screen.pixels[test_offset + 2] = 0xCC;

        screen.led = 1;
        screen.draw_led();

        // Verify that pixels outside the LED area are unchanged
        assert_eq!(screen.pixels[test_offset], 0xAA);
        assert_eq!(screen.pixels[test_offset + 1], 0xBB);
        assert_eq!(screen.pixels[test_offset + 2], 0xCC);
    }
}
