use crate::domension::Dimension;
use crate::hardware::keyboard::vkey::CustomVirtualKeyCode;
use speedy2d::window::ModifiersState;

pub(crate) enum UserInput {
    OpenK7File,
    Stop,
    Start,
    SoftReset,
    HardReset,
    KeyDown(CustomVirtualKeyCode),
    KeyUp(CustomVirtualKeyCode),
    KeyboardModifierChanged(ModifiersState),
    WindowResized(Dimension),
}
