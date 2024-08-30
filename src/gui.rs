use log::error;
use speedy2d::dimen::{UVec2, Vec2};
use speedy2d::error::{BacktraceError, ErrorMessage};
use speedy2d::image::ImageDataType::RGB;
use speedy2d::image::ImageHandle;
use speedy2d::image::ImageSmoothingMode::NearestNeighbor;
use speedy2d::window::{KeyScancode, ModifiersState, VirtualKeyCode, WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use std::sync::mpsc::{Receiver, Sender};

use crate::hardware::keyboard::vkey::map_virtual_key_code;
use crate::hardware::screen::{DEFAULT_PIXEL_SIZE, HEIGHT, WIDTH};
use crate::user_input::UserInput;
use crate::user_input::UserInput::{HardReset, OpenK7File, SoftReset};

#[derive(Debug)]
pub(crate) struct Gui {
    image: Option<ImageHandle>,
    user_input_sender: Sender<UserInput>,
    image_data_receiver: Receiver<Vec<u8>>,
}

impl Gui {
    pub(crate) fn new(
        user_input_sender: Sender<UserInput>,
        image_data_receiver: Receiver<Vec<u8>>,
    ) -> Self {
        Gui {
            image: None,
            user_input_sender,
            image_data_receiver,
        }
    }
}

impl WindowHandler for Gui {
    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        if let Ok(buf) = self.image_data_receiver.try_recv() {
            let raw = buf.as_slice();
            let image = graphics.create_image_from_raw_pixels(
                RGB,
                NearestNeighbor,
                UVec2::new(
                    (DEFAULT_PIXEL_SIZE * WIDTH) as u32,
                    (DEFAULT_PIXEL_SIZE * HEIGHT) as u32,
                ),
                raw,
            );
            match image {
                Ok(image) => {
                    self.image = Some(image);
                }
                Err(err) => error!("Error creating image from raw pixels {err:?}"),
            }
        }

        if let Some(image) = &self.image {
            graphics.draw_image(Vec2::ZERO, image);
        }
        helper.request_redraw();
    }

    fn on_key_down(
        &mut self,
        _: &mut WindowHelper<()>,
        virtual_key_code: Option<VirtualKeyCode>,
        scancode: KeyScancode,
    ) {
        let action = match virtual_key_code {
            Some(VirtualKeyCode::F2) => Some(OpenK7File),
            Some(VirtualKeyCode::F7) => Some(SoftReset),
            Some(VirtualKeyCode::F8) => Some(HardReset),
            _ => map_virtual_key_code(virtual_key_code, scancode).map(|vk| UserInput::KeyDown(vk)),
        };
        if let Some(action) = action {
            self.user_input_sender.send(action).ok();
        }
    }

    fn on_key_up(
        &mut self,
        _: &mut WindowHelper<()>,
        virtual_key_code: Option<VirtualKeyCode>,
        scancode: KeyScancode,
    ) {
        if let Some(vk) = map_virtual_key_code(virtual_key_code, scancode) {
            self.user_input_sender.send(UserInput::KeyUp(vk)).ok();
        }
    }

    fn on_keyboard_modifiers_changed(&mut self, _: &mut WindowHelper<()>, state: ModifiersState) {
        self.user_input_sender
            .send(UserInput::KeyboardModifierChanged(state))
            .ok();
    }
}
