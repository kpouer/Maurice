use crate::hardware::keyboard::vkey::MO5VirtualKeyCode;
use crate::raw_image::RawImage;
use crate::user_input::UserInput;
use crate::user_input::UserInput::{HardReset, OpenK7File, SoftReset};
#[cfg(feature = "speedy2d-display")]
use speedy2d::{
    dimen::{UVec2, Vec2},
    image::{ImageDataType::RGB, ImageSmoothingMode::NearestNeighbor},
    window::{KeyScancode, ModifiersState, VirtualKeyCode, WindowHandler, WindowHelper},
    Graphics2D,
};
#[cfg(feature = "egui-display")]
use {
    eframe::{epaint::TextureHandle, App, Frame},
    egui::{pos2, Color32, Context, Event, Key, Rect, TextureId, TextureOptions},
};

use std::sync::mpsc::{Receiver, Sender};

pub struct Gui {
    user_input_sender: Sender<UserInput>,
    image_data_receiver: Receiver<RawImage>,
    #[cfg(feature = "egui-display")]
    image: Option<TextureHandle>,
}

impl Gui {
    pub fn new(
        user_input_sender: Sender<UserInput>,
        image_data_receiver: Receiver<RawImage>,
    ) -> Self {
        Self {
            user_input_sender,
            image_data_receiver,
            #[cfg(feature = "egui-display")]
            image: None,
        }
    }
}

#[cfg(feature = "egui-display")]
impl Gui {
    fn handle_input(&mut self, ctx: &Context) {
        ctx.input(|input_state| {
            let modifiers = input_state.modifiers;
            self.user_input_sender
                .send(UserInput::KeyboardModifierChanged(modifiers.into()))
                .ok();
            input_state.events.iter().for_each(|event| match event {
                Event::Key {
                    key,
                    physical_key: _,
                    pressed,
                    repeat: false,
                    modifiers: _,
                } => {
                    let evt = if *pressed {
                        UserInput::KeyDown
                    } else {
                        UserInput::KeyUp
                    };
                    let action = match key {
                        Key::F2 => Some(OpenK7File),
                        Key::F7 => Some(SoftReset),
                        Key::F8 => Some(HardReset),
                        _ => MO5VirtualKeyCode::try_from(*key).ok().map(evt),
                    };
                    if let Some(action) = action {
                        self.user_input_sender.send(action).ok();
                    }
                }
                _ => {}
            });
        });
    }

    fn update_texture(&mut self, ctx: &Context) {
        let mut current_raw_image = None;
        if let Ok(buf) = self.image_data_receiver.recv() {
            current_raw_image = Some(buf);
            while let Ok(buf) = self.image_data_receiver.try_recv() {
                current_raw_image = Some(buf);
            }
        }

        if let Some(buf) = current_raw_image {
            let image = egui::ColorImage::from_rgb([buf.width, buf.height], &buf.data);
            match &mut self.image {
                None => {
                    self.image =
                        Some(ctx.load_texture("my_texture", image, TextureOptions::default()))
                }
                Some(texture) => texture.set(image, TextureOptions::default()),
            }
        }
    }
}

#[cfg(feature = "egui-display")]
impl App for Gui {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        self.update_texture(ctx);

        if let Some(image) = &self.image {
            let texture_id = TextureId::from(image);

            // Draw
            let rect = Rect {
                min: pos2(0.0, 0.0),
                max: pos2(ctx.available_rect().width(), ctx.available_rect().height()),
            };
            let uv = Rect {
                min: pos2(0.0, 0.0),
                max: pos2(1.0, 1.0),
            };
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.painter().image(texture_id, rect, uv, Color32::WHITE);
            });
            ctx.request_repaint_after(std::time::Duration::from_millis(50));
        }

        self.handle_input(ctx);
    }
}

#[cfg(feature = "speedy2d-display")]
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
                Err(err) => log::error!("Error creating image from raw pixels {err:?}"),
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
            Some(key) => MO5VirtualKeyCode::try_from(key)
                .ok()
                .map(UserInput::KeyDown),
            _ => None,
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
        if let Some(vk) = virtual_key_code {
            MO5VirtualKeyCode::try_from(vk)
                .ok()
                .iter()
                .map(|mo5virtual_key_code| UserInput::KeyDown(*mo5virtual_key_code))
                .for_each(|user_input| {
                    self.user_input_sender.send(user_input).ok();
                });
        }
    }

    fn on_keyboard_modifiers_changed(&mut self, _: &mut WindowHelper<()>, state: ModifiersState) {
        self.user_input_sender
            .send(UserInput::KeyboardModifierChanged(state.into()))
            .ok();
    }
}
