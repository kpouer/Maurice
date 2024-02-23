use std::path::PathBuf;
use speedy2d::window::ModifiersState;
use crate::hardware::keyboard::vkey::CustomVirtualKeyCode;

pub(crate) enum UserInput {
    SetK7(PathBuf),
    Stop,
    Start,
    SoftReset,
    HardReset,
    KeyDown(CustomVirtualKeyCode),
    KeyUp(CustomVirtualKeyCode),
    KeyboardModifierChanged(ModifiersState),
}