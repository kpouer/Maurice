use speedy2d::window::ModifiersState;

use crate::hardware::keyboard::vkey::CustomVirtualKeyCode;

pub(crate) enum UserInput {
    OpenK7File,
    Stop,
    Start,
    SoftReset,
    HardReset,
    KeyDown(CustomVirtualKeyCode),
    KeyUp(CustomVirtualKeyCode),
    KeyboardModifierChanged(ModifiersState),
}