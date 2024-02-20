use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use log::info;
use crate::hardware::machine::Machine;

pub(crate) enum UserInput {
    SetK7(PathBuf),
    Stop,
    Start,
    SoftReset,
    HardReset,
}

pub(crate) fn eventually_process_user_input(mut machine: &mut Machine, user_input_receiver: &Receiver<UserInput>) {
    if let Ok(user_input) = user_input_receiver.try_recv() {
        match user_input {
            UserInput::SetK7(k7) => {
                machine.set_k7_file(&k7);
            }
            UserInput::Stop => machine.running = false,
            UserInput::Start => machine.running = true,
            UserInput::SoftReset => machine.reset_soft(),
            UserInput::HardReset => machine.reset_hard(),
        }
    }
}