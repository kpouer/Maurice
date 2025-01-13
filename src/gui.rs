use crate::hardware::keyboard::vkey::MO5VirtualKeyCode;
use crate::user_input::UserInput;
use crate::user_input::UserInput::{HardReset, OpenK7File, RewindK7File, SoftReset, Start, Stop};
#[cfg(feature = "speedy2d-display")]
use speedy2d::{
    dimen::{UVec2, Vec2},
    image::{ImageDataType::RGB, ImageSmoothingMode::NearestNeighbor},
    window::{KeyScancode, ModifiersState, VirtualKeyCode, WindowHandler, WindowHelper},
    Graphics2D,
};
use std::sync::{Arc, Mutex};
#[cfg(feature = "egui-display")]
use {
    eframe::{epaint::TextureHandle, App, Frame},
    egui::{pos2, Color32, Context, Event, Key, Rect, TextureOptions, Ui, ViewportCommand},
};

use crate::gui::DialogKind::About;
use crate::hardware::screen::{HEIGHT, WIDTH};
use crate::machine_snap_event::MachineSnapEvent;
use log::info;
use std::sync::mpsc::{Receiver, Sender};

#[derive(Default)]
enum DialogKind {
    #[default]
    None,
    Debug,
    About,
}

#[derive(Default)]
struct Dialog {
    current: DialogKind,
}

pub struct Gui {
    user_input_sender: Sender<UserInput>,
    image_data_receiver: Receiver<MachineSnapEvent>,
    #[cfg(feature = "egui-display")]
    image: Option<TextureHandle>,
    #[cfg(feature = "egui-display")]
    dialog: DialogKind,
    registers: String,
    unassemble: String,
}

impl Gui {
    pub fn new(
        user_input_sender: Sender<UserInput>,
        image_data_receiver: Receiver<MachineSnapEvent>,
    ) -> Self {
        Self {
            user_input_sender,
            image_data_receiver,
            #[cfg(feature = "egui-display")]
            image: None,
            #[cfg(feature = "egui-display")]
            dialog: DialogKind::None,
            registers: String::new(),
            unassemble: String::new(),
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
        let mut current_event = None;
        while let Ok(machine_event_snap) = self.image_data_receiver.try_recv() {
            current_event = Some(machine_event_snap);
        }

        if let Some(mut evt) = current_event.take() {
            if let Some(buf) = evt.raw_image.take() {
                self.registers = evt.registers;
                self.unassemble = evt.unassembled;
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

    fn build_menu_panel(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                self.file_menu(ui);
                self.run_menu(ui);
                self.reset_menu(ui);
                self.image_menu(ui, ctx);
                self.debug_menu(ui);
                self.help_menu(ui);
            });
        });
    }

    fn file_menu(&mut self, ui: &mut Ui) {
        ui.menu_button("File", |ui| {
            if ui.button("Select K7").clicked() {
                self.user_input_sender.send(OpenK7File).ok();
            }
            if ui.button("Rewind Tape").clicked() {
                self.user_input_sender.send(RewindK7File).ok();
            }
            if ui.button("Exit").clicked() {
                info!("Exit");
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
            }
        });
    }

    fn run_menu(&mut self, ui: &mut Ui) {
        ui.menu_button("Run", |ui| {
            if ui.button("Stop").clicked() {
                self.user_input_sender.send(Stop).ok();
            }
            if ui.button("Go").clicked() {
                self.user_input_sender.send(Start).ok();
            }
        });
    }

    fn reset_menu(&mut self, ui: &mut Ui) {
        ui.menu_button("Reset", |ui| {
            if ui.button("Soft Reset").clicked() {
                self.user_input_sender.send(SoftReset).ok();
            }
            if ui.button("Hard Reset").clicked() {
                self.user_input_sender.send(HardReset).ok();
            }
        });
    }

    //Reset
    //     Soft Reset
    // Hard Reset
    fn image_menu(&mut self, ui: &mut Ui, ctx: &Context) {
        ui.menu_button("Image", |ui| {
            if ui.button("Zoom 1x").clicked() {
                ctx.send_viewport_cmd(ViewportCommand::InnerSize(
                    [(1 * WIDTH) as f32, (1 * HEIGHT) as f32].into(),
                ))
            }
            if ui.button("Zoom 2x").clicked() {
                ctx.send_viewport_cmd(ViewportCommand::InnerSize(
                    [(2 * WIDTH) as f32, (2 * HEIGHT) as f32].into(),
                ))
            }
            if ui.button("Zoom 3x").clicked() {
                ctx.send_viewport_cmd(ViewportCommand::InnerSize(
                    [(3 * WIDTH) as f32, (3 * HEIGHT) as f32].into(),
                ))
            }
            if ui.button("Filter").clicked() {
                //todo: implement
            }
        });
    }

    //Reset
    //     Soft Reset
    // Hard Reset
    fn debug_menu(&mut self, ui: &mut Ui) {
        ui.menu_button("Debug", |ui| {
            if ui.button("Debug").clicked() {
                self.dialog = DialogKind::Debug;
            }
        });
    }

    //Reset
    //     Soft Reset
    // Hard Reset
    fn help_menu(&mut self, ui: &mut Ui) {
        ui.menu_button("Help", |ui| {
            if ui.button("About").clicked() {
                self.dialog = About;
            }
        });
    }

    fn eventually_show_dialog(&mut self, ctx: &Context) {
        match self.dialog {
            DialogKind::None => {}
            DialogKind::Debug => self.show_debug(ctx),
            DialogKind::About => self.show_about(ctx),
        }
    }

    fn show_debug(&mut self, ctx: &Context) {
        let debug = self.registers.clone();
        let unassemble = self.unassemble.clone();
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("about_viewport"),
            egui::ViewportBuilder::default()
                .with_title("About Maurice")
                .with_inner_size([400.0, 200.0]),
            move |ctx, class| {
                assert!(
                    class == egui::ViewportClass::Immediate,
                    "This egui backend doesn't support multiple viewports"
                );

                let dbg = debug.clone();
                let unassemble = unassemble.clone();
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.label(dbg);
                        ui.label(unassemble);
                    });
                });
                if ctx.input(|i| i.viewport().close_requested()) {
                    // Tell parent to close us.
                    self.dialog = DialogKind::None;
                }
            },
        );
    }

    fn show_about(&mut self, ctx: &Context) {
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("about_viewport"),
            egui::ViewportBuilder::default()
                .with_title("About Maurice")
                .with_inner_size([400.0, 200.0]),
            move |ctx, class| {
                assert!(
                    class == egui::ViewportClass::Immediate,
                    "This egui backend doesn't support multiple viewports"
                );

                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.label(
                        "
                       Maurice

            (C) G.Fetis 1997-1998-2006
            (C) DevilMarkus http://cpc.devilmarkus.de 2006
            (C) M.Le Goff 2014
            (C) Matthieu Casanova 2023-2025

            Rust conversion of Marcel o Cinq MO5 Emulator (Java version)
            ",
                    );
                });
                if ctx.input(|i| i.viewport().close_requested()) {
                    // Tell parent to close us.
                    self.dialog = DialogKind::None;
                }
            },
        );
    }
}

#[cfg(feature = "egui-display")]
impl App for Gui {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        self.handle_input(ctx);
        self.build_menu_panel(ctx);
        self.eventually_show_dialog(ctx);
        self.update_texture(ctx);

        if let Some(image) = &self.image {
            let available_rect = ctx.available_rect();
            let rect = Rect {
                min: pos2(available_rect.left(), available_rect.top()),
                max: pos2(available_rect.width(), available_rect.height()),
            };
            let uv = Rect {
                min: pos2(0.0, 0.0),
                max: pos2(1.0, 1.0),
            };

            egui::CentralPanel::default()
                .show(ctx, |ui| {
                    ui.painter().image(image.into(), rect, uv, Color32::WHITE);
                })
                .response
                .request_focus();
            ctx.request_repaint_after(std::time::Duration::from_millis(50));
        }
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
