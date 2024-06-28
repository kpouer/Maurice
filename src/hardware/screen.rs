use crate::hardware::memory::Memory;
use crate::int;

const PALETTE: [u32; 16] = [
    0x000000, 0xF00000, 0x00F000, 0xF0F000, 0x0000F0, 0xF000F0, 0x00F0F0, 0xF0F0F0, 0x636363,
    0xF06363, 0x63F063, 0xF0F063, 0x0063F0, 0xF063F0, 0x63F0F0, 0xF06300,
];

pub(crate) const WIDTH: usize = 320;
pub(crate) const HEIGHT: usize = 200;
pub(crate) const DEFAULT_PIXEL_SIZE: usize = 1;

#[derive(Debug)]
pub(crate) struct Screen {
    pub(crate) mouse_clic: bool,
    pub(crate) mouse_x: int,
    pub(crate) mouse_y: int,
    pub(crate) pixels: Vec<u32>,
    pub(crate) rgb_pixels: Vec<u8>,
    filter: bool,
    pub(crate) led: int,
    pub(crate) show_led: int,
}

impl Screen {
    pub(crate) fn new() -> Self {
        Screen {
            mouse_clic: false,
            mouse_x: -1,
            mouse_y: -1,
            pixels: vec![0xff000000; WIDTH * HEIGHT],
            rgb_pixels: Vec::new(),
            filter: false,
            led: 0,
            show_led: 0,
        }
    }

    // pub(crate) fn set_pixel_size(&mut self, ps: usize, mem: &mut Memory) {
    //     mem.set_all_dirty();
    // }

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

    pub(crate) fn get_pixels(&self, pixel_size: usize) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();
        for y in 0..HEIGHT {
            for _ in 0..pixel_size {
                for x in 0..WIDTH {
                    for _ in 0..pixel_size {
                        let p = self.pixels[x + y * WIDTH];
                        let r = (p & 0xFF) as u8;
                        let g = ((p >> 8) & 0xFF) as u8;
                        let b = ((p >> 16) & 0xFF) as u8;
                        buffer.push(b);
                        buffer.push(g);
                        buffer.push(r);
                    }
                }
            }
        }
        buffer
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
                    let cc2 = PALETTE[c1];
                    let cc1 = PALETTE[c2];

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
