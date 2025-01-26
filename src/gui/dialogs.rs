use crate::gui::about::About;
#[cfg(not(target_arch = "wasm32"))]
use crate::gui::debug::Debug;
use crate::hardware::machine::Machine;
use egui::{Context, Widget};

#[derive(Default)]
pub(crate) struct Dialogs {
    debug: bool,
    about: bool,
}

impl Dialogs {
    pub(crate) fn eventually_show_dialogs(&mut self, ctx: &Context, machine: &mut Machine) {
        if self.debug {
            self.show_debug(ctx, machine);
        }
        if self.about {
            self.show_about(ctx);
        }
    }

    pub(crate) fn set_show_about(&mut self) {
        self.about = true;
    }

    pub(crate) fn set_show_debug(&mut self) {
        self.debug = true;
    }

    fn show_debug(&mut self, ctx: &Context, machine: &mut Machine) {
        // #[cfg(target_arch = "wasm32")]
        // {
        //     egui::Window::new("Debug Maurice")
        //         .default_size([600.0, 600.0])
        //         .min_size([600.0, 600.0])
        //         .max_size([600.0, 600.0])
        //         .open(&mut self.debug)
        //         .resizable(true)
        //         .show(ctx, |ui| Debug::new(machine).ui(ui));
        // }

        #[cfg(not(target_arch = "wasm32"))]
        ctx.show_viewport_immediate(
            egui::ViewportId::from_hash_of("about_viewport"),
            egui::ViewportBuilder::default()
                .with_title("Debug Maurice")
                .with_inner_size([400.0, 200.0]),
            move |ctx, class| {
                assert!(
                    class == egui::ViewportClass::Immediate,
                    "This egui backend doesn't support multiple viewports"
                );

                egui::CentralPanel::default().show(ctx, |ui| {
                    Debug::new(machine).ui(ui);
                });
                if ctx.input(|i| i.viewport().close_requested()) {
                    // Tell parent to close us.
                    self.debug = false;
                }
            },
        );
    }

    fn show_about(&mut self, ctx: &Context) {
        #[cfg(target_arch = "wasm32")]
        {
            egui::Window::new("About Maurice")
                .default_size([400.0, 200.0])
                .min_size([400.0, 200.0])
                .max_size([400.0, 200.0])
                .open(&mut self.about)
                .resizable(false)
                .show(ctx, |ui| About::new().ui(ui));
        }

        #[cfg(not(target_arch = "wasm32"))]
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

                egui::CentralPanel::default().show(ctx, |ui| About::new().ui(ui));
                if ctx.input(|i| i.viewport().close_requested()) {
                    // Tell parent to close us.
                    self.about = false;
                }
            },
        );
    }
}
