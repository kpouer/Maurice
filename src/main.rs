#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use eframe::epaint::ColorImage;
use egui::ImageData;

use crate::gui::Gui;
use crate::hardware::machine::Machine;
use crate::hardware::screen::{DEFAULT_PIXEL_SIZE, HEIGHT, WIDTH};
use crate::user_input::{eventually_process_user_input, UserInput};

mod gui;
mod hardware;
pub(crate) mod data_input_stream;
mod user_input;

pub(crate) type int = i32;

fn main() {//-> Result<(), eframe::Error> {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([(DEFAULT_PIXEL_SIZE as usize * WIDTH) as f32, (DEFAULT_PIXEL_SIZE as usize * HEIGHT) as f32])
            .with_min_inner_size([WIDTH as f32, HEIGHT as f32])
        ,
        ..Default::default()
    };
    let mut machine = Machine::default();
    machine.screen.set_pixel_size(DEFAULT_PIXEL_SIZE, &mut machine.mem);

    let (image_data_sender, image_data_receiver) = channel::<ImageData>();
    let (user_input_sender, user_input_receiver) = channel::<UserInput>();
    thread::spawn(move|| run_loop(&mut machine, &user_input_receiver, &image_data_sender));
    eframe::run_native(
        "Bernard",
        native_options,
        Box::new(|cc| Box::new(Gui::new(image_data_receiver, user_input_sender))),
    ).unwrap();
}

fn run_loop(mut machine: &mut Machine, user_input_receiver: &Receiver<UserInput>, image_data_sender: &Sender<ImageData>) {
    loop {
        eventually_process_user_input(&mut machine, &user_input_receiver);
        if !machine.running {
            thread::sleep(std::time::Duration::from_millis(1000 / 60));
            continue;
        }
        machine.run();
        machine.screen.paint(&mut machine.mem);
        let color_image = ColorImage::from_rgb([WIDTH, HEIGHT], &machine.screen.rgb_pixels.as_slice());
        let image_data = ImageData::from(color_image);
        image_data_sender.send(image_data).unwrap();
    }
}

