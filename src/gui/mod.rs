mod about;
mod debug;
mod dialogs;

use crate::hardware::keyboard::vkey::MO5VirtualKeyCode;
use egui::{DroppedFile, Pos2};
use {
    eframe::{epaint::TextureHandle, App, Frame},
    egui::{pos2, Color32, Context, Event, Key, Rect, TextureOptions, Ui, ViewportCommand},
};

use crate::gui::dialogs::Dialogs;
use crate::hardware::machine::Machine;
use crate::hardware::screen::{HEIGHT, WIDTH};
use log::info;

#[derive(Default)]
pub struct Gui {
    machine: Machine,
    image: Option<TextureHandle>,
    dialogs: Dialogs,
    #[cfg(not(target_arch = "wasm32"))]
    file_dialog: Option<egui_file_dialog::FileDialog>,
}

impl Gui {
    fn handle_input(&mut self, ctx: &Context) {
        ctx.input(|input_state| {
            let modifiers = input_state.modifiers;
            self.machine.keyboard.modifiers = modifiers.into();
            if !input_state.raw.dropped_files.is_empty() {
                self.handle_dropped_files(&input_state.raw.dropped_files);
            }
            input_state.events.iter().for_each(|event| match event {
                Event::Key {
                    key,
                    physical_key: _,
                    pressed,
                    repeat: false,
                    modifiers: _,
                } => {
                    match key {
                        Key::F7 => self.machine.reset_soft(),
                        Key::F8 => self.machine.reset_hard(),
                        _ => {
                            if let Ok(vk) = MO5VirtualKeyCode::try_from(*key) {
                                if *pressed {
                                    self.machine.keyboard.key_pressed(vk, &mut self.machine.mem);
                                } else {
                                    self.machine
                                        .keyboard
                                        .key_released(vk, &mut self.machine.mem);
                                };
                            }
                        }
                    };
                }
                _ => {}
            });
        });
    }

    fn handle_dropped_files(&mut self, dropped_files: &[DroppedFile]) {
        for file in dropped_files.iter() {
            if let Some(path) = &file.path {
                info!("Dropped file: {}", path.to_str().unwrap());
                self.machine.set_k7_file(path);
            }
        }
    }

    fn update_texture(&mut self, ctx: &Context) {
        let pixels = self.machine.run_loop();

        if let Some(buf) = pixels {
            let image = egui::ColorImage::from_rgb([buf.width, buf.height], buf.data);
            match &mut self.image {
                None => {
                    self.image =
                        Some(ctx.load_texture("my_texture", image, TextureOptions::default()))
                }
                Some(texture) => texture.set(image, TextureOptions::default()),
            }
        }
    }

    fn build_menu_panel(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                self.file_menu(ui);
                self.run_menu(ui);
                self.reset_menu(ui);
                #[cfg(not(target_arch = "wasm32"))]
                {
                    self.image_menu(ui, ctx);
                    // todo debug the debug dialog
                    self.debug_menu(ui);
                }
                self.help_menu(ui);
            });
        });
    }

    fn file_menu(&mut self, ui: &mut Ui) {
        ui.menu_button("File", |ui| {
            #[cfg(not(target_arch = "wasm32"))]
            if ui.button("Select K7").clicked() {
                let mut fd = egui_file_dialog::FileDialog::new();
                fd.pick_file();
                self.file_dialog = Some(fd);
            }
            if ui.button("Rewind Tape").clicked() {
                self.machine.rewind_k7();
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
                self.machine.stop();
            }
            if ui.button("Go").clicked() {
                self.machine.start();
            }
        });
    }

    fn reset_menu(&mut self, ui: &mut Ui) {
        ui.menu_button("Reset", |ui| {
            if ui.button("Soft Reset").clicked() {
                self.machine.reset_soft();
            }
            if ui.button("Hard Reset").clicked() {
                self.machine.reset_hard();
            }
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn image_menu(&mut self, ui: &mut Ui, ctx: &Context) {
        ui.menu_button("Image", |ui| {
            if ui.button("Zoom 1x").clicked() {
                ctx.send_viewport_cmd(ViewportCommand::InnerSize(
                    [WIDTH as f32, HEIGHT as f32].into(),
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
        });
    }

    //Reset
    //     Soft Reset
    // Hard Reset
    fn debug_menu(&mut self, ui: &mut Ui) {
        ui.menu_button("Debug", |ui| {
            if ui.button("Debug").clicked() {
                self.dialogs.set_show_debug();
            }
        });
    }

    //Reset
    //     Soft Reset
    // Hard Reset
    fn help_menu(&mut self, ui: &mut Ui) {
        ui.menu_button("Help", |ui| {
            if ui.button("About").clicked() {
                self.dialogs.set_show_about();
            }
        });
    }
}

impl App for Gui {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        self.handle_input(ctx);
        self.build_menu_panel(ctx);
        self.dialogs.eventually_show_dialogs(ctx, &mut self.machine);
        self.update_texture(ctx);
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(fd) = &mut self.file_dialog {
            fd.update(ctx);
            if let Some(path) = fd.take_picked() {
                self.machine.set_k7_file(path);
                self.file_dialog = None;
            }
        }

        if let Some(image) = &self.image {
            let available_rect = ctx.available_rect();
            let rect = Rect {
                min: pos2(available_rect.left(), available_rect.top()),
                max: pos2(available_rect.width(), available_rect.height()),
            };
            let uv = Rect {
                min: Pos2::ZERO,
                max: pos2(1.0, 1.0),
            };

            egui::CentralPanel::default()
                .show(ctx, |ui| {
                    ui.painter().image(image.into(), rect, uv, Color32::WHITE);
                })
                .response
                .request_focus();
            ctx.request_repaint();
        }
    }
}
