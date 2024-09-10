use log::error;
use speedy2d::dimen::{UVec2, Vec2};
use speedy2d::image::ImageDataType::RGB;
use speedy2d::image::ImageSmoothingMode::NearestNeighbor;
use speedy2d::window::{KeyScancode, ModifiersState, VirtualKeyCode, WindowHandler, WindowHelper};
use speedy2d::Graphics2D;
use std::sync::mpsc::{Receiver, Sender};

use crate::hardware::keyboard::vkey::map_virtual_key_code;
use crate::raw_image::RawImage;
use crate::user_input::UserInput;
use crate::user_input::UserInput::{HardReset, OpenK7File, SoftReset};

#[derive(Debug)]
pub(crate) struct Gui {
    user_input_sender: Sender<UserInput>,
    image_data_receiver: Receiver<RawImage>,
}

impl Gui {
    pub(crate) fn new(
        user_input_sender: Sender<UserInput>,
        image_data_receiver: Receiver<RawImage>,
    ) -> Self {
        Self {
            user_input_sender,
            image_data_receiver,
        }
    }
}

impl WindowHandler for Gui {
    fn on_resize(&mut self, _: &mut WindowHelper<()>, size_pixels: UVec2) {
        self.user_input_sender
            .send(UserInput::WindowResized(size_pixels.into()))
            .ok();
    }

    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        if let Ok(buf) = self.image_data_receiver.recv() {
            let image =
                graphics.create_image_from_raw_pixels(RGB, NearestNeighbor, buf.size(), &buf.data);
            match image {
                Ok(image) => graphics.draw_image(Vec2::ZERO, &image),
                Err(err) => error!("Error creating image from raw pixels {err:?}"),
            }
        } else {
            println!("No image available");
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
            _ => map_virtual_key_code(virtual_key_code, scancode).map(UserInput::KeyDown),
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
