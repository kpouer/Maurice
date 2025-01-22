use clap::Parser;
use maurice::args::Args;
use maurice::gui::Gui;
use maurice::hardware::machine::Machine;
use maurice::hardware::screen::{DEFAULT_PIXEL_SIZE, HEIGHT, WIDTH};
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    env_logger::init();

    // let args = Args::parse();
    //
    // thread::spawn(move || {
    //     let mut machine = Machine::default();
    //     if let Some(k7) = &args.k7 {
    //         machine.set_k7_file(k7);
    //     }
    //     machine.run_loop()
    // });
    {
        let native_options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_drag_and_drop(true)
                .with_inner_size([
                    (DEFAULT_PIXEL_SIZE * WIDTH) as f32,
                    (DEFAULT_PIXEL_SIZE * HEIGHT) as f32,
                ]),
            ..Default::default()
        };
        let gui = Gui::default();
        let _ = eframe::run_native("Maurice", native_options, Box::new(|_cc| Ok(Box::new(gui))));
    }
}
