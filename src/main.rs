#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use speedy2d::Window;
use crate::gui::Gui;
use crate::hardware::screen::{DEFAULT_PIXEL_SIZE, HEIGHT, WIDTH};

mod gui;
mod hardware;
pub(crate) mod data_input_stream;

pub(crate) type int = i32;

fn main() {//-> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let window = Window::new_centered("Marcel", (DEFAULT_PIXEL_SIZE as u32 * WIDTH as u32, DEFAULT_PIXEL_SIZE as u32 * HEIGHT as u32)).unwrap();
    window.run_loop(Gui::default());
}

struct Marcel {
    gui: Gui,
}

impl Default for Marcel {
    fn default() -> Self {
        println!("Marcel::new()");
        Self {
            gui: Gui::default(),
        }
    }
}
