use crate::hardware::keyboard::modifiers::Modifiers;
use crate::hardware::keyboard::vkey::MO5VirtualKeyCode;
use std::path::PathBuf;

pub enum UserInput {
    FileOpened(PathBuf),
    RewindK7File,
    Stop,
    Start,
    SoftReset,
    HardReset,
    KeyDown(MO5VirtualKeyCode),
    KeyUp(MO5VirtualKeyCode),
    KeyboardModifierChanged(Modifiers),
    WindowResized(crate::dimension::Dimension),
}
