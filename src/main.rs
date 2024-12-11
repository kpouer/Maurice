use log::error;
use maurice::gui::Gui;
use maurice::hardware::machine::Machine;
use maurice::hardware::screen::{DEFAULT_PIXEL_SIZE, HEIGHT, WIDTH};
use maurice::raw_image::RawImage;
use maurice::user_input::UserInput;
use speedy2d::Window;
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    env_logger::init();
    let (image_data_sender, image_data_receiver) = channel::<RawImage>();
    let (user_input_sender, user_input_receiver) = channel::<UserInput>();
    // let machine = Machine::new(image_data_sender, user_input_receiver);
    thread::spawn(move || {
        let mut machine = Machine::new(image_data_sender, user_input_receiver);
        machine.run_loop()
    });
    let window = Window::new_centered(
        "Maurice",
        (
            (DEFAULT_PIXEL_SIZE * WIDTH) as u32,
            (DEFAULT_PIXEL_SIZE * HEIGHT) as u32,
        ),
    );

    match window {
        Ok(window) => window.run_loop(Gui::new(user_input_sender, image_data_receiver)),
        Err(e) => {
            error!("Error creating window: {e}");
        }
    }
}
