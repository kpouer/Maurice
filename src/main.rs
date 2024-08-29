use std::sync::mpsc::channel;
use std::thread;

use speedy2d::Window;

use crate::gui::Gui;
use crate::hardware::machine::Machine;
use crate::hardware::screen::{DEFAULT_PIXEL_SIZE, HEIGHT, WIDTH};
use crate::user_input::UserInput;

mod bios;
pub(crate) mod data_input_stream;
mod gui;
mod hardware;
mod user_input;

pub(crate) type int = i32;

fn main() {
    env_logger::init();
    let (image_data_sender, image_data_receiver) = channel::<Vec<u8>>();
    let (user_input_sender, user_input_receiver) = channel::<UserInput>();
    // let machine = Machine::new(image_data_sender, user_input_receiver);
    thread::spawn(move || {
        let mut machine = Machine::new(image_data_sender, user_input_receiver);
        machine.run_loop()
    });
    let window = Window::new_centered(
        "Maurice",
        (
            DEFAULT_PIXEL_SIZE as u32 * WIDTH as u32,
            DEFAULT_PIXEL_SIZE as u32 * HEIGHT as u32,
        ),
    )
    .unwrap();
    window.run_loop(Gui::new(user_input_sender, image_data_receiver));
}
