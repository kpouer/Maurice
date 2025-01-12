use crate::hardware::keyboard::modifiers::Modifiers;
use crate::hardware::keyboard::vkey::MO5VirtualKeyCode;

pub enum UserInput {
    OpenK7File,
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
