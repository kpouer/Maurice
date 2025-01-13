use crate::hardware::keyboard::modifiers::Modifiers;
use crate::hardware::keyboard::vkey::MO5VirtualKeyCode;
use std::path::PathBuf;

pub enum UserInput {
    #[cfg(feature = "speedy2d-display")]
    OpenK7File,
    #[cfg(feature = "egui-display")]
    FileOpened(PathBuf),
    RewindK7File,
    Stop,
    Start,
    SoftReset,
    HardReset,
    KeyDown(MO5VirtualKeyCode),
    KeyUp(MO5VirtualKeyCode),
    KeyboardModifierChanged(Modifiers),
    #[cfg(feature = "resizable-api")]
    WindowResized(crate::dimension::Dimension),
}
