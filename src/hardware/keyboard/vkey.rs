use speedy2d::window::{KeyScancode, VirtualKeyCode};
use crate::hardware::keyboard::vkey::CustomVirtualKeyCode::Base;

#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
pub(crate) enum CustomVirtualKeyCode {
    Base(VirtualKeyCode)
}

pub(crate) fn map_virtual_key_code(virtual_key_code: Option<VirtualKeyCode>, scancode: KeyScancode) -> Option<CustomVirtualKeyCode> {
    match virtual_key_code {
        Some(vk) => Some(Base(vk)),
        None => {
            match scancode {
                _ => {
                    println!("Unknown scancode: {:?}", scancode);
                    None
                }
            }
        }
    }
}