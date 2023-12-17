use speedy2d::color::Color;
use speedy2d::dimen::{UVec2, Vector2};
use speedy2d::error::{BacktraceError, ErrorMessage};
use speedy2d::Graphics2D;
use speedy2d::image::ImageDataType::RGB;
use speedy2d::image::ImageHandle;
use speedy2d::image::ImageSmoothingMode::NearestNeighbor;
use speedy2d::shape::Rectangle;
use crate::hardware::memory::Memory;
use crate::int;

const PALETTE: [u32; 16] = [
    0x000000,
    0xF00000,
    0x00F000,
    0xF0F000,
    0x0000F0,
    0xF000F0,
    0x00F0F0,
    0xF0F0F0,
    0x636363,
    0xF06363,
    0x63F063,
    0xF0F063,
    0x0063F0,
    0xF063F0,
    0x63F0F0,
    0xF06300,
];

pub(crate) const WIDTH: usize = 320;
pub(crate) const HEIGHT: usize = 200;

#[derive(Debug)]
pub(crate) struct Screen {
    pub(crate) mouse_clic: bool,
    pub(crate) mouse_x: int,
    pub(crate) mouse_y: int,
    pixels: Vec<u32>,
    pixel_size: f64,
    filter: bool,
    pub(crate) led: int,
    pub(crate) show_led: int,
    pub(crate) must_redraw: bool,
}

impl Screen {
    pub(crate) fn new() -> Self {
        Screen {
            mouse_clic: false,
            mouse_x: -1,
            mouse_y: -1,
            pixels: vec![0xff000000;320*200],
            pixel_size: 2.0,
            filter: false,
            led: 0,
            show_led: 0,
            must_redraw: false,
        }
    }

    pub(crate) fn set_pixel_size(&mut self, ps: f64, mem: &mut Memory) {
        self.pixel_size = ps;
        mem.set_all_dirty();
    }

    pub(crate) fn repaint(&mut self) {
        self.must_redraw = true;
    }

    pub(crate) fn paint(&mut self, graphics: &mut Graphics2D, mem: &mut Memory) -> Result<ImageHandle, BacktraceError<ErrorMessage>> {
        // let og = BuffImg.getGraphics(mem);
        if self.show_led > 0 {
            self.show_led -= 1;
            let color = if self.led != 0 {
                Color::from_rgb(255., 0., 0.)
            } else {
                Color::from_rgb(0., 0., 0.)
            };
            let rectangle:Rectangle<f32> = Rectangle::new(Vector2::new(320. - 16., 0.), Vector2::new(16., 8.));
            graphics.draw_rectangle(rectangle, color);
        }
        self.dopaint(mem);
        let mut buffer: Vec<u8> = Vec::new();
        self.pixels
            .iter()
            .for_each(|p| {
                let r = (p & 0xFF) as u8;
                let g = ((p >> 8) & 0xFF) as u8;
                let b = ((p >> 16) & 0xFF) as u8;
                buffer.push(r);
                buffer.push(g);
                buffer.push(b);
            });
        let raw = buffer.as_slice();
        let size = UVec2::new(WIDTH as u32, HEIGHT as u32);
        let image = graphics.create_image_from_raw_pixels(
            RGB,
            NearestNeighbor,
            size,
            raw);
        image
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
                    let c2 = col & 0x0F;
                    let c1 = col >> 4;
                    let cc2 = PALETTE[c1 as usize];
                    let cc1 = PALETTE[c2 as usize];

                    let pt = mem.POINT(i);
                    if (0x80 & pt) != 0 {
                        self.pixels[x + offset] = cc2;
                    } else {
                        self.pixels[x + offset] = cc1;
                    }
                    x += 1;
                    if (0x40 & pt) != 0 {
                        self.pixels[x + offset] = cc2;
                    } else {
                        self.pixels[x + offset] = cc1;
                    }
                    x += 1;
                    if (0x20 & pt) != 0 {
                        self.pixels[x + offset] = cc2;
                    } else {
                        self.pixels[x + offset] = cc1;
                    }
                    x += 1;
                    if (0x10 & pt) != 0 {
                        self.pixels[x + offset] = cc2;
                    } else {
                        self.pixels[x + offset] = cc1;
                    }
                    x += 1;
                    if (0x08 & pt) != 0 {
                        self.pixels[x + offset] = cc2;
                    } else {
                        self.pixels[x + offset] = cc1;
                    }
                    x += 1;
                    if (0x04 & pt) != 0 {
                        self.pixels[x + offset] = cc2;
                    } else {
                        self.pixels[x + offset] = cc1;
                    }
                    x += 1;
                    if (0x02 & pt) != 0 {
                        self.pixels[x + offset] = cc2;
                    } else {
                        self.pixels[x + offset] = cc1;
                    }
                    x += 1;
                    if (0x01 & pt) != 0 {
                        self.pixels[x + offset] = cc2;
                    } else {
                        self.pixels[x + offset] = cc1;
                    }
                    x += 1;
                    i += 1;
                }
            }
        }
    }
}
